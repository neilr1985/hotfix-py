#!/usr/bin/env python3
"""
Simple example demonstrating HotFIX Python wrapper usage.

This script connects to a FIX session and submits a new order.
"""

import sys
import time
from datetime import datetime, UTC
from pathlib import Path

from hotfix import Session, Message, InboundDecision, OutboundDecision, Application


class MyApplication(Application):
    """
    FIX Application that handles session callbacks.

    Implements the Application protocol to receive FIX session events and messages.
    """

    def on_logon(self) -> None:
        """Called when successfully logged on to the FIX session."""
        print("✓ FIX session logged on!")

    def on_logout(self, reason: str) -> None:
        """Called when the session logs out."""
        print(f"✗ FIX session logged out: {reason}")

    def on_inbound_message(self, msg: Message) -> InboundDecision:
        """
        Process incoming FIX messages.

        Args:
            msg: The incoming FIX message

        Returns:
            InboundDecision: Accept to process the message, TerminateSession to disconnect
        """
        print(f"← Received message: {msg}")
        return InboundDecision.Accept

    def on_outbound_message(self, msg: Message) -> OutboundDecision:
        """
        Process outgoing FIX messages before they are sent.

        Args:
            msg: The outgoing FIX message

        Returns:
            OutboundDecision: Send to transmit, Drop to discard, or TerminateSession to disconnect
        """
        print(f"→ Sending message: {msg}")
        return OutboundDecision.Send


def create_new_order_single(cl_ord_id: str, symbol: str, side: str, quantity: int, price: float) -> Message:
    """
    Create a FIX NewOrderSingle (MsgType=D) message.

    Args:
        cl_ord_id: Client order ID
        symbol: Trading symbol (e.g., "EUR/USD")
        side: Order side ("BUY" or "SELL")
        quantity: Order quantity
        price: Limit price

    Returns:
        Message ready to send
    """
    msg = Message("D")  # MsgType D = NewOrderSingle

    # Required fields for NewOrderSingle
    msg.insert(11, cl_ord_id)  # ClOrdID
    msg.insert(55, symbol)     # Symbol
    msg.insert(54, "1" if side.upper() == "BUY" else "2")  # Side: 1=Buy, 2=Sell
    msg.insert(38, str(quantity))  # OrderQty
    msg.insert(40, "2")  # OrdType: 2=Limit
    msg.insert(44, str(price))  # Price
    msg.insert(60, datetime.now(UTC).strftime("%Y%m%d-%H:%M:%S.%f")[:-3])  # TransactTime

    return msg


def main():
    # Configuration file path
    config_path = Path(__file__).parent / "config" / "test-config.toml"

    if not config_path.exists():
        print(f"Error: Config file not found at {config_path}")
        print("Please create a FIX session configuration file.")
        sys.exit(1)

    print("=== HotFIX Python Wrapper - Simple New Order Example ===\n")

    try:
        # Create application instance
        app = MyApplication()

        # Create and start FIX session with the application
        print(f"Starting FIX session from config: {config_path}")
        session = Session(str(config_path), app)
        print("✓ Session started successfully\n")

        # Give the session a moment to connect and log on
        print("Waiting for connection to establish...")
        time.sleep(2)

        # Create a new order
        order_id = f"ORDER_{int(time.time())}"
        print(f"\nCreating new order:")
        print(f"  ClOrdID: {order_id}")
        print(f"  Symbol:  EUR/USD")
        print(f"  Side:    BUY")
        print(f"  Qty:     100000")
        print(f"  Price:   1.0850")

        order = create_new_order_single(
            cl_ord_id=order_id,
            symbol="EUR/USD",
            side="BUY",
            quantity=100000,
            price=1.0850
        )

        # Send the order
        print("\nSending order to counterparty...")
        session.send_message(order)
        print("✓ Order sent successfully!\n")

        # Keep the session alive for a bit to receive any responses
        print("Session active. Press Ctrl+C to exit...")
        try:
            while True:
                time.sleep(1)
        except KeyboardInterrupt:
            print("\n\nShutting down session...")

    except Exception as e:
        print(f"\n✗ Error: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)

    print("✓ Session closed cleanly")


if __name__ == "__main__":
    main()
