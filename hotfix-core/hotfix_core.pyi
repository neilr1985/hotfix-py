"""
Type stubs for the hotfix_core Rust extension module.

This module provides the core FIX protocol implementation.
"""

from enum import Enum
from typing import Any

class InboundDecision(Enum):
    """
    Decision for handling inbound FIX messages.

    Attributes:
        Accept: Process the message normally
        TerminateSession: Disconnect immediately
    """
    Accept: InboundDecision
    TerminateSession: InboundDecision

class OutboundDecision(Enum):
    """
    Decision for handling outbound FIX messages.

    Attributes:
        Send: Transmit the message to the counterparty
        Drop: Discard the message without sending
        TerminateSession: Disconnect immediately
    """
    Send: OutboundDecision
    Drop: OutboundDecision
    TerminateSession: OutboundDecision

class Message:
    """
    A FIX protocol message.

    This class represents a FIX message that can be constructed field-by-field
    and sent through a session, or received from a counterparty.

    Example:
        >>> msg = Message("D")  # NewOrderSingle
        >>> msg.insert(11, "ORDER123")  # ClOrdID
        >>> msg.insert(55, "EUR/USD")   # Symbol
        >>> msg.insert(54, "1")         # Side (Buy)
        >>> session.send_message(msg)
    """

    def __init__(self, message_type: str) -> None:
        """
        Create a new FIX message.

        Args:
            message_type: The FIX message type code (e.g., "D" for NewOrderSingle)
        """
        ...

    def insert(self, tag: int, value: str) -> None:
        """
        Insert a field into the message.

        Args:
            tag: The FIX tag number (e.g., 11 for ClOrdID, 55 for Symbol)
            value: The field value as a string

        Example:
            >>> msg.insert(11, "ORDER123")  # ClOrdID
            >>> msg.insert(55, "EUR/USD")   # Symbol
            >>> msg.insert(38, "100000")    # OrderQty
        """
        ...

class Session:
    """
    A FIX protocol session.

    This class manages a FIX connection to a counterparty, handling the protocol
    state machine, message sequencing, and heartbeat management automatically.

    The session runs in a background thread, allowing your application to remain
    responsive while maintaining the FIX connection.

    Example:
        >>> class MyApp:
        ...     def on_logon(self) -> None:
        ...         print("Connected!")
        ...     def on_logout(self, reason: str) -> None:
        ...         print(f"Disconnected: {reason}")
        ...     def on_inbound_message(self, msg: Message) -> InboundDecision:
        ...         return InboundDecision.Accept
        ...     def on_outbound_message(self, msg: Message) -> OutboundDecision:
        ...         return OutboundDecision.Send
        ...
        >>> app = MyApp()
        >>> session = Session("config.toml", app)
        >>> msg = Message("D")  # NewOrderSingle
        >>> session.send_message(msg)
    """

    def __init__(self, config_path: str, application: Any) -> None:
        """
        Create and start a new FIX session.

        This constructor loads the configuration, establishes the connection,
        and begins the FIX logon sequence in a background thread.

        Args:
            config_path: Path to the TOML session configuration file
            application: Callback handler implementing the Application protocol

        Raises:
            RuntimeError: If the session fails to start
            FileNotFoundError: If the config file doesn't exist
        """
        ...

    def send_message(self, message: Message) -> None:
        """
        Send a FIX message to the counterparty.

        This method blocks until the message has been queued for transmission.
        The actual transmission happens asynchronously in the background thread.

        Args:
            message: The FIX message to send

        Raises:
            RuntimeError: If the session is closed or the send fails

        Example:
            >>> msg = Message("D")  # NewOrderSingle
            >>> msg.insert(11, "ORDER123")
            >>> msg.insert(55, "EUR/USD")
            >>> session.send_message(msg)
        """
        ...

def encode_message(message: Message, begin_string: str, separator: int) -> bytes:
    """
    Encode a FIX message to bytes.

    This function converts a Message object into the wire format for transmission
    or storage. This is primarily for debugging or custom message handling.

    Args:
        message: The message to encode
        begin_string: FIX version string (e.g., "FIX.4.4")
        separator: Field separator byte (typically 0x01 for SOH)

    Returns:
        The encoded message as bytes

    Example:
        >>> msg = Message("D")
        >>> msg.insert(11, "ORDER123")
        >>> encoded = encode_message(msg, "FIX.4.4", 0x01)
    """
    ...
