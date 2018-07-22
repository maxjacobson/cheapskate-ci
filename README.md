# cheapskate-ci

Run your CI locally.

## Installation

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

There's no post-push hook, unfortunately, so after pushing you'll want to manually run:

```shell
cheapskate-ci run --status
```

Which will again run the steps, and then send a successful commit status to GitHub.
