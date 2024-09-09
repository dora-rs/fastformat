# How to contribute to `fastformat`

We welcome bug reports, feature requests, and pull requests!

Please discuss non-trivial changes in a Github issue or on Discord first before implementing them.
This way, we can avoid unnecessary work on both sides.

## Style

We use [`rustfmt`](https://github.com/rust-lang/rustfmt) with its default settings to format our code.
Please run `cargo fmt --all` on your code before submitting a pull request.
Our CI will run an automatic formatting check of your code.

## Publishing new Versions

The maintainers are responsible for publishing new versions of the `fastformat` crates.
Please don't open unsolicited pull requests to create new releases.
Instead, request a new version by opening an issue or by leaving a comment on a merged PR.
