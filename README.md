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
`pyproject.toml` content:
```toml
[project]
version = "0.1.0"
```
`mylib/__init__.py` content:
```python
__version__ = "0.1.0"

def add(a, b):
    return a + b
```
***
And when releasing a new version of the project, it's required to __bump__ a version to the next. According to the [semver](https://semver.org), for example, adding something new that has comptitibility with previous version is meaning __bumping a minor part__

So, adding this:
```python
# mylib/__init__.py`
__version__ = "0.1.0"

def add(a, b):
    return a + b

def sub(a, b):
    return a - b
```

means the version now should be bumped from `0.1.0` to `0.2.0`, so the content of `pyproject.toml` and `__init__.py` should be changed too

```toml
# pyproject.toml
[project]
version = "0.1.0"
```
`mylib/__init__.py` content:
```python
__version__ = "0.2.0"

def add(a, b):
    return a + b
```
```python
# mylib/__init__.py`
__version__ = "0.2.0"

def add(a, b):
    return a + b

def sub(a, b):
    return a - b
```

### What does `weee` simplify?
Every time you updates you need:
1. Don't forget about bumping version
1. Find all files contain version mention
1. Replace the version part everything
1. Sometimes it requires different formats so every collaborator shoudl know about this
1. Push a new tag with the same version
1. Maybe run some checks before

And these steps are very boilerpart after some time
***
`weee` can automatize all this step!

#### Fristly, init `weee` in your project
