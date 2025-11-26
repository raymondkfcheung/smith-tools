# dot-pr-review

List GitHub issues and PRs involving you, filtered by date and optional search parameters.

## Building from source

Clone the repository and build the release binary:

```shell
git clone https://github.com/raymondkfcheung/smith-tools.git
cd smith-tools/dot-pr-review
cargo build -r
```

After a successful build, the compiled binary will be available here:

```shell
ls -la target/release/dot-pr-review
```

## Running the tool

This tool requires you to specify a date so it knows which items to fetch. The date must be provided in `YYYY-MM-DD` format.

```shell
# Using the release build
target/release/dot-pr-review --updated-since 2025-01-20

# Or run with Cargo during development
cargo run -- --updated-since 2025-01-20
```

You can optionally provide additional filters:

* `--repo owner/name` — limit results to a specific repository
* `--is value` — add an `is:` filter (e.g. `issue`, `pr`)
* `--state value` — add a `state:` filter (e.g. `open`, `closed`)

Examples:

```shell
cargo run -- --updated-since 2025-01-20 --repo paritytech/polkadot-sdk
cargo run -- --updated-since 2025-01-20 --is issue
cargo run -- --updated-since 2025-01-20 --state open
cargo run -- --updated-since 2025-01-20 --is issue --state open
```

To view all available options:

```shell
target/release/dot-pr-review --help
# or
cargo run -- --help
```

```text
List PRs created by or assigned to you, filtered by updated date

Usage: dot-pr-review [OPTIONS] --updated-since <YYYY-MM-DD>

Options:
      --updated-since <YYYY-MM-DD>  Updated on or after this date (YYYY-MM-DD)
      --repo <REPO>                 Optional `repo` filter: owner/name (e.g. paritytech/polkadot-sdk)
      --is <FILTER>                 Optional `is` filter (e.g. issue, pr)
      --state <STATE>               Optional `state` filter (e.g. open, closed)
  -h, --help                        Print help
```
