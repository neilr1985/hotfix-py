"""
HotFIX - Python bindings for the HotFIX FIX protocol engine.

This package provides a Pythonic interface to the HotFIX Rust library,
enabling FIX protocol connectivity with Python applications.
"""

# Re-export core types directly from the Rust extension
from hotfix_core import (
    Session,
    Message,
    InboundDecision,
    OutboundDecision,
    encode_message,
)

# Import protocol for type hints
from .protocol import Application

__all__ = [
    # Core types
    "Session",
    "Message",
    "InboundDecision",
    "OutboundDecision",

    # Protocol
    "Application",

    # Utilities
    "encode_message",
]

__version__ = "0.1.0"
