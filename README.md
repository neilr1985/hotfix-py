# HotFIX – Python

A Python wrapper for the HotFIX engine.

## Development

Create a virtual environment and install the requirements.
With the virtual environment activated, build the core package
by running

```shell
maturin develop
```

within the `hotfix-core` directory.

With the package built, you can test it out in a Python shell
within the virtual environment.

```python
from hotfix_core import encode_message, Message

m = Message("D")

# Set the desired fields
# For now, values need to be passed as bytes
m.insert(11, b"test-order-1")
m.insert(78, b"1")

# Encode the message
encode_message(m, "FIX.4.4", ord('|'))
```
