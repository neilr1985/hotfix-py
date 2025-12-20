# HotFIX Python Package

This is the pure Python wrapper around the `hotfix_core` Rust extension, providing type hints, Protocol definitions, and comprehensive documentation.

## Package Structure

```
hotfix/
├── __init__.py          # Re-exports from hotfix_core
├── protocol.py          # Application Protocol definition
├── hotfix_core.pyi      # Type stubs for Rust extension (comprehensive docs)
├── py.typed             # PEP 561 marker for type hints
└── README.md            # This file
```

## Design Approach

The package uses **direct re-exports** with type stubs rather than wrapper classes. This approach:
- Avoids PyO3 subclassing limitations
- Provides full type information via `.pyi` stub files
- Keeps the runtime overhead minimal
- Enables comprehensive IDE support and type checking

## Type Hints

The package is fully typed and includes:
- `py.typed` marker for PEP 561 compliance
- Comprehensive type stubs in `hotfix_core.pyi` with full docstrings
- Runtime-checkable `Application` Protocol
- Full type hints for all public APIs

## Usage

### Basic Example

```python
from hotfix import Session, Message, InboundDecision, OutboundDecision, Application

class MyApp(Application):
    def on_logon(self) -> None:
        print("Connected!")

    def on_logout(self, reason: str) -> None:
        print(f"Disconnected: {reason}")

    def on_inbound_message(self, msg: Message) -> InboundDecision:
        print(f"Received: {msg}")
        return InboundDecision.Accept

    def on_outbound_message(self, msg: Message) -> OutboundDecision:
        return OutboundDecision.Send

# Create session
app = MyApp()
session = Session("config.toml", app)

# Send message
msg = Message("D")  # NewOrderSingle
msg.insert(11, "ORDER123")  # ClOrdID
msg.insert(55, "EUR/USD")   # Symbol
session.send_message(msg)
```

## Development

To install in development mode:

```bash
# Install the Rust extension
cd hotfix-core && maturin develop

# Install the Python package
cd .. && pip install -e .
```

## Type Checking

The package is configured for strict type checking with Pyright:

```bash
pyright hotfix/
```

## Architecture

- **hotfix_core**: Rust extension providing Session, Message, and enums
- **hotfix**: Pure Python package that re-exports with type information
- **Application Protocol**: Defines the callback interface for type safety
- **Type stubs (.pyi)**: Provide comprehensive documentation and type hints
