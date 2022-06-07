# Weee!
> Weee! It's time for a new release of your product!

`Weee` is version bumper CLI written in Rust for automation version updating process


## Overview
Let's take a look on a typical Python (for example) project
```shell
├── mylib
│   └── __init__.py
├── pyproject.toml
└── requirements.txt
```
***
`pyproject.toml` content:
```toml
[project]
version = "0.1.0"
```
`mylib/__init__.py` content:
```python
__version__ = "0.1.0"

# And some code
...
...
```
