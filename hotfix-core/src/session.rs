use hotfix::Application;
use hotfix::application::{InboundDecision, OutboundDecision};
use hotfix::config::Config;
use hotfix::initiator::Initiator;
use hotfix::store::file::FileStore;
use pyo3::{pyclass, pymethods, PyResult, PyErr};
use std::thread;
use tokio::runtime::Runtime;
use tokio::sync::{mpsc, oneshot};
use crate::message::Message;

/// Commands sent from Python thread to the Tokio runtime thread
enum SessionCommand {
    SendMessage {
        message: Message,
        response_tx: oneshot::Sender<Result<(), String>>,
    },
    Shutdown,
}

/// Python-exposed FIX session that wraps a HotFIX Initiator
#[pyclass]
pub struct Session {
    command_tx: mpsc::UnboundedSender<SessionCommand>,
    runtime_thread: Option<thread::JoinHandle<()>>,
}

#[pymethods]
impl Session {
    /// Create a new FIX session from a config file path
    #[new]
    fn new(config_path: String) -> PyResult<Self> {
        let (command_tx, mut command_rx) = mpsc::unbounded_channel();

        let runtime_thread = thread::spawn(move || {
            let runtime = Runtime::new().expect("Failed to create Tokio runtime");
            runtime.block_on(async move {
                // Load config
                let mut config = Config::load_from_path(&config_path);
                let session_config = config.sessions.pop().expect("Config must include a session");

                // Create dummy application
                let app = DummyApplication;

                // Create store
                let store = FileStore::new("messages", "hotfix-py")
                    .expect("Failed to create message store");

                // Start initiator
                let initiator = Initiator::start(session_config, app, store).await;

                // Event loop - process commands from Python
                while let Some(cmd) = command_rx.recv().await {
                    match cmd {
                        SessionCommand::SendMessage { message, response_tx } => {
                            let result = initiator.send_message(message).await
                                .map_err(|e| e.to_string());
                            response_tx.send(result).ok();
                        }
                        SessionCommand::Shutdown => break,
                    }
                }
            });
        });

        Ok(Session {
            command_tx,
            runtime_thread: Some(runtime_thread),
        })
    }

    /// Send a FIX message to the counterparty
    fn send_message(&self, message: Message) -> PyResult<()> {
        let (response_tx, response_rx) = oneshot::channel();

        self.command_tx.send(SessionCommand::SendMessage {
            message,
            response_tx,
        }).map_err(|_| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Session closed"))?;

        response_rx.blocking_recv()
            .map_err(|_| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("No response from session"))?
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        // Send shutdown command
        self.command_tx.send(SessionCommand::Shutdown).ok();

        // Wait for runtime thread to finish
        if let Some(thread) = self.runtime_thread.take() {
            thread.join().ok();
        }
    }
}

/// Dummy application that accepts all messages (for proof of concept)
struct DummyApplication;

#[async_trait::async_trait]
impl Application<Message> for DummyApplication {
    async fn on_outbound_message(&self, _msg: &Message) -> OutboundDecision {
        OutboundDecision::Send
    }

    async fn on_inbound_message(&self, _msg: Message) -> InboundDecision {
        InboundDecision::Accept
    }

    async fn on_logout(&mut self, _reason: &str) {
        // TODO: Log or handle logout
    }

    async fn on_logon(&mut self) {
        // TODO: Log or handle logon
    }
}
