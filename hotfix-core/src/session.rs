use hotfix::Application;
use hotfix::application::{InboundDecision as RustInboundDecision, OutboundDecision as RustOutboundDecision};
use hotfix::config::Config;
use hotfix::initiator::Initiator;
use hotfix::store::file::FileStore;
use pyo3::{pyclass, pymethods, PyResult, PyErr, Py, PyAny, Python};
use std::sync::Arc;
use std::thread;
use tokio::runtime::Runtime;
use tokio::sync::{mpsc, oneshot, Mutex};
use crate::message::Message;

/// Python enum for inbound message decisions
#[pyclass]
#[derive(Clone)]
pub enum InboundDecision {
    Accept,
    TerminateSession,
}

/// Python enum for outbound message decisions
#[pyclass]
#[derive(Clone)]
pub enum OutboundDecision {
    Send,
    Drop,
    TerminateSession,
}

impl InboundDecision {
    fn to_rust(&self) -> RustInboundDecision {
        match self {
            InboundDecision::Accept => RustInboundDecision::Accept,
            InboundDecision::TerminateSession => RustInboundDecision::TerminateSession,
        }
    }
}

impl OutboundDecision {
    fn to_rust(&self) -> RustOutboundDecision {
        match self {
            OutboundDecision::Send => RustOutboundDecision::Send,
            OutboundDecision::Drop => RustOutboundDecision::Drop,
            OutboundDecision::TerminateSession => RustOutboundDecision::TerminateSession,
        }
    }
}

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
    /// Create a new FIX session from a config file path with a Python application
    #[new]
    fn new(config_path: String, application: Py<PyAny>) -> PyResult<Self> {
        let (command_tx, mut command_rx) = mpsc::unbounded_channel();

        let runtime_thread = thread::spawn(move || {
            let runtime = Runtime::new().expect("Failed to create Tokio runtime");
            runtime.block_on(async move {
                // Load config
                let mut config = Config::load_from_path(&config_path);
                let session_config = config.sessions.pop().expect("Config must include a session");

                // Create Python application wrapper
                let app = PythonApplication::new(application);

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

/// Application that calls Python callbacks
struct PythonApplication {
    callback: Arc<Mutex<Py<PyAny>>>,
}

impl PythonApplication {
    fn new(callback: Py<PyAny>) -> Self {
        PythonApplication {
            callback: Arc::new(Mutex::new(callback)),
        }
    }
}

#[async_trait::async_trait]
impl Application<Message> for PythonApplication {
    async fn on_outbound_message(&self, msg: &Message) -> RustOutboundDecision {
        let callback = self.callback.clone();
        let msg_clone = msg.clone();

        tokio::task::spawn_blocking(move || {
            Python::attach(|py| {
                let callback = callback.blocking_lock();

                // Call on_outbound_message method
                match callback.call_method1(py, "on_outbound_message", (msg_clone,)) {
                    Ok(result) => {
                        // Try to extract as OutboundDecision enum
                        if let Ok(decision) = result.extract::<OutboundDecision>(py) {
                            decision.to_rust()
                        } else {
                            // Default to Send if extraction fails
                            eprintln!("Warning: on_outbound_message did not return OutboundDecision, defaulting to Send");
                            RustOutboundDecision::Send
                        }
                    }
                    Err(e) => {
                        eprintln!("Error calling on_outbound_message: {}", e);
                        RustOutboundDecision::Send
                    }
                }
            })
        })
        .await
        .unwrap_or(RustOutboundDecision::Send)
    }

    async fn on_inbound_message(&self, msg: Message) -> RustInboundDecision {
        let callback = self.callback.clone();

        tokio::task::spawn_blocking(move || {
            Python::attach(|py| {
                let callback = callback.blocking_lock();

                // Call on_inbound_message method
                match callback.call_method1(py, "on_inbound_message", (msg,)) {
                    Ok(result) => {
                        // Try to extract as InboundDecision enum
                        if let Ok(decision) = result.extract::<InboundDecision>(py) {
                            decision.to_rust()
                        } else {
                            // Default to Accept if extraction fails
                            eprintln!("Warning: on_inbound_message did not return InboundDecision, defaulting to Accept");
                            RustInboundDecision::Accept
                        }
                    }
                    Err(e) => {
                        eprintln!("Error calling on_inbound_message: {}", e);
                        RustInboundDecision::Accept
                    }
                }
            })
        })
        .await
        .unwrap_or(RustInboundDecision::Accept)
    }

    async fn on_logout(&mut self, reason: &str) {
        let callback = self.callback.clone();
        let reason = reason.to_string();

        tokio::task::spawn_blocking(move || {
            Python::attach(|py| {
                let callback = callback.blocking_lock();

                if let Err(e) = callback.call_method1(py, "on_logout", (reason,)) {
                    eprintln!("Error calling on_logout: {}", e);
                }
            })
        })
        .await
        .ok();
    }

    async fn on_logon(&mut self) {
        let callback = self.callback.clone();

        tokio::task::spawn_blocking(move || {
            Python::attach(|py| {
                let callback = callback.blocking_lock();

                if let Err(e) = callback.call_method0(py, "on_logon") {
                    eprintln!("Error calling on_logon: {}", e);
                }
            })
        })
        .await
        .ok();
    }
}
