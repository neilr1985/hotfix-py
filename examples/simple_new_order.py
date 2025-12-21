#!/usr/bin/env python3
"""
Example demonstrating HotFIX Python wrapper with repeating groups.

This script connects to a FIX session and submits an FX swap order,
demonstrating the use of repeating groups for multileg strategies.
"""

import sys
import time
from datetime import datetime, UTC
from pathlib import Path

from hotfix import Session, Message, InboundDecision, OutboundDecision, Application, RepeatingGroup


class MyApplication(Application):
    """FIX Application that handles session callbacks."""

    def on_logon(self) -> None:
        print("✓ FIX session logged on!")

    def on_logout(self, reason: str) -> None:
        print(f"✗ FIX session logged out: {reason}")

    def on_inbound_message(self, msg: Message) -> InboundDecision:
        print(f"← Received message: {msg}")
        return InboundDecision.Accept

    def on_outbound_message(self, msg: Message) -> OutboundDecision:
        print(f"→ Sending message: {msg}")
        return OutboundDecision.Send


def create_fx_swap_order(cl_ord_id: str, symbol: str, quantity: int, near_rate: float, far_rate: float, far_date: str) -> Message:
    """Create a FIX NewOrderMultileg message for an FX swap (buy spot, sell forward)."""
    msg = Message("AB")

    msg.insert(11, cl_ord_id)
    msg.insert(55, symbol)
    msg.insert(54, "1")  # Side (overall direction)
    msg.insert(38, str(quantity))
    msg.insert(40, "D")  # OrdType: Previously Quoted
    msg.insert(60, datetime.now(UTC).strftime("%Y%m%d-%H:%M:%S.%f")[:-3])

    # NoLegs repeating group (555=count, 600=delimiter)
    near_leg = RepeatingGroup(555, 600)
    near_leg.append(600, symbol)
    near_leg.append(624, "1")  # Buy
    near_leg.append(556, "EUR")
    near_leg.append(687, str(quantity))
    near_leg.append(566, str(near_rate))

    far_leg = RepeatingGroup(555, 600)
    far_leg.append(600, symbol)
    far_leg.append(624, "2")  # Sell
    far_leg.append(556, "EUR")
    far_leg.append(687, str(quantity))
    far_leg.append(566, str(far_rate))
    far_leg.append(588, far_date)

    msg.insert_groups(555, [near_leg, far_leg])
    return msg


def main():
    config_path = Path(__file__).parent / "config" / "test-config.toml"

    if not config_path.exists():
        print(f"Error: Config file not found at {config_path}")
        print("Please create a FIX session configuration file.")
        sys.exit(1)

    print("=== HotFIX Python Wrapper - FX Swap Example ===\n")

    try:
        app = MyApplication()

        print(f"Starting FIX session from config: {config_path}")
        session = Session(str(config_path), app)
        print("✓ Session started successfully\n")

        print("Waiting for connection to establish...")
        time.sleep(2)

        order_id = f"SWAP_{int(time.time())}"
        print(f"\nCreating FX swap order:")
        print(f"  ClOrdID:    {order_id}")
        print(f"  Symbol:     EUR/USD")
        print(f"  Quantity:   1000000")
        print(f"  Near Leg:   Buy @ 1.0850 (spot)")
        print(f"  Far Leg:    Sell @ 1.0875 (3M forward)")
        print(f"  Far Date:   20250321")

        swap_order = create_fx_swap_order(
            cl_ord_id=order_id,
            symbol="EUR/USD",
            quantity=1000000,
            near_rate=1.0850,
            far_rate=1.0875,
            far_date="20250321"
        )

        print("\nSending FX swap order to counterparty...")
        session.send_message(swap_order)
        print("✓ FX swap order sent successfully!\n")

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
