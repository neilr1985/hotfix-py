"""
Protocol definitions for HotFIX application callbacks.
"""

from typing import Protocol, runtime_checkable
from hotfix_core import InboundDecision, OutboundDecision

from typing import TYPE_CHECKING
if TYPE_CHECKING:
    from hotfix_core import Message

@runtime_checkable
class Application(Protocol):
    """
    Protocol for FIX application callback handlers.

    Implement this protocol to handle FIX session events and messages.
    All methods are required.

    Example:
        >>> class MyApp:
        ...     def on_logon(self) -> None:
        ...         print("Session logged on")
        ...
        ...     def on_logout(self, reason: str) -> None:
        ...         print(f"Session logged out: {reason}")
        ...
        ...     def on_inbound_message(self, msg: 'Message') -> InboundDecision:
        ...         print(f"Received: {msg}")
        ...         return InboundDecision.Accept
        ...
        ...     def on_outbound_message(self, msg: 'Message') -> OutboundDecision:
        ...         return OutboundDecision.Send
    """

    def on_logon(self) -> None:
        """
        Called when the FIX session successfully logs on.

        This is invoked after a successful Logon message exchange with the counterparty.
        """
        ...

    def on_logout(self, reason: str) -> None:
        """
        Called when the FIX session logs out.

        Args:
            reason: A string describing why the session logged out
        """
        ...

    def on_inbound_message(self, msg: Message) -> InboundDecision:
        """
        Process an incoming FIX message.

        This method is called for each application-level message received from the counterparty.
        Administrative messages (Logon, Logout, Heartbeat, etc.) are handled automatically.

        Args:
            msg: The incoming FIX message

        Returns:
            InboundDecision.Accept to process the message normally
            InboundDecision.TerminateSession to disconnect immediately
        """
        ...

    def on_outbound_message(self, msg: Message) -> OutboundDecision:
        """
        Validate an outgoing FIX message before it is sent.

        This method is called before each message is transmitted to the counterparty,
        allowing you to inspect, validate, or reject outgoing messages.

        Args:
            msg: The outgoing FIX message

        Returns:
            OutboundDecision.Send to transmit the message
            OutboundDecision.Drop to discard the message without sending
            OutboundDecision.TerminateSession to disconnect immediately
        """
        ...


