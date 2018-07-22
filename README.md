# cheapskate-ci

Run your CI locally.

## Setup

* Run `cargo install cheapskate-ci`, or
* Clone the repository and run `cargo install --path .`

## Usage

Example pre-commit hook:

```shell
#!/bin/sh

export RUST_LOG="cheapskate_ci=debug"
cheapskate-ci run
```
