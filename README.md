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

cheapskate-ci run
```

Example post-push hook:

```shell
#!/bin/sh

cheapskate-ci run --status
```
