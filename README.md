# cheapskate-ci

Run your CI locally.

## Setup

* Run `cargo install cheapskate-ci`, or
* Clone the repository and run `cargo install --path .`

## Usage

Create a `cheapskate-ci.toml` in the root of your project.
See [cheapskate-ci.toml](cheapskate-ci.toml) for an example.

Example pre-commit hook (to make sure all commits are passing):

```shell
#!/bin/sh

export RUST_LOG="cheapskate_ci=debug"
cheapskate-ci run
```

Example pre-push hook:

```shell
#!/bin/sh

export RUST_LOG="cheapskate_ci=debug"
cheapskate-ci run --status
```
