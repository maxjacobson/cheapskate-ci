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

**Note:** the first time you run this, it will prompt you for a GitHub token.
You'll need to [generate one][token] with the `repo:status` scope and paste it in.
That token will be cached in `~/.local/share/cheapskate-ci/psst.toml` going forward.

[token]: https://github.com/settings/tokens/new

## How to use cheapskate-ci as a required status to push to the default branch

* check out a feature branch
* push up some code
* run `cheapskate-ci run --status`
* check out your default branch
* merge in that branch: `git merge --ff-only -`
* push to your default branch (this will be allowed as long as the CI run succeeded)

If you use a pull request flow, you can do the same thing, but just open and merge the PR instead of merging locally.
