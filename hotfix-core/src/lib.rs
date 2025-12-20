mod message;
mod session;

use pyo3::prelude::*;

#[pymodule(name = "hotfix_core")]
mod hotfix_core {
    #[pymodule_export]
    use super::message::{encode_message, Message};

    #[pymodule_export]
    use super::session::{Session, InboundDecision, OutboundDecision};
}
