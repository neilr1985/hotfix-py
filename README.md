# HotFIX - Python

Python bindings for the [HotFIX](https://github.com/validus-risk-management/hotfix) FIX engine,
providing a fast, type-safe interface for building FIX trading applications in Python.

## Overview

HotFIX enables Python applications to communicate using the FIX (Financial Information eXchange) protocol.

> [!WARNING]
> This project is in early development.
> 
> Currently, most effort is focused on the core engine in Rust,
> with these Python bindings serving as a proof-of-concept
> for the future integration of the engine into Python applications.

## Package Structure

This project consists of two packages:

- **`hotfix_core`** - Rust extension module (built with PyO3/maturin)
- **`hotfix`** - Pure Python package with type hints and convenience wrappers

Users should import from `hotfix`, which re-exports `hotfix_core` with full type information.

## Quick Start - Development

### Installation

```bash
# Create and activate virtual environment
python -m venv .venv
source .venv/bin/activate

# Build and install the Rust extension
cd hotfix-core
maturin develop
cd ..

# Install the Python package
pip install -e .
```

### Basic Usage

```python
from hotfix import Session, Message, InboundDecision, OutboundDecision, Application

class MyApplication(Application):
    """Handle FIX session callbacks."""

    def on_logon(self) -> None:
        print("✓ FIX session connected")

    def on_logout(self, reason: str) -> None:
        print(f"✗ Session disconnected: {reason}")

    def on_inbound_message(self, msg: Message) -> InboundDecision:
        print(f"← Received: {msg}")
        return InboundDecision.Accept

    def on_outbound_message(self, msg: Message) -> OutboundDecision:
        print(f"→ Sending: {msg}")
        return OutboundDecision.Send

# Create session
app = MyApplication()
session = Session("config.toml", app)

# Send a NewOrderSingle message
order = Message("D")  # MsgType=D (NewOrderSingle)
order.insert(11, "ORDER123")      # ClOrdID
order.insert(55, "EUR/USD")       # Symbol
order.insert(54, "1")             # Side (Buy)
order.insert(38, "100000")        # OrderQty
order.insert(40, "2")             # OrdType (Limit)
order.insert(44, "1.0850")        # Price

session.send_message(order)
```

See the complete example in [`examples/simple_new_order.py`](examples/simple_new_order.py).

> [!IMPORTANT]
> The example application requires an acceptor to connect to.
> 
> An easy way to set up an acceptor is using one of the QuickFIX implementations.

## Configuration

Sessions are configured via TOML files. Example configuration:

```toml
[[sessions]]
begin_string = "FIX.4.4"
sender_comp_id = "YOUR_SENDER_ID"
target_comp_id = "COUNTERPARTY_ID"

connection_port = 9880
connection_host = "127.0.0.1"

heartbeat_interval = 30
reset_on_logon = true
```

For the full list of configuration options, refer to the HotFIX documentation.
