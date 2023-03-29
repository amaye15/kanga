# Kanga

A library to start the kangaroo war to end them all data using Rust.

## Installation

```bash
pip install kangajo
```

## Usage

```python
import kanga

json_data = """
{
    "id": 1,
    "first_name": "Jonathan",
    ...
}
"""

flat_data = kanga.flatten_dict(json_data)
print(flat_data)
```