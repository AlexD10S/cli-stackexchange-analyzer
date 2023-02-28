# CLI StackExchange Analyser
CLI tool to get metrics from a stackexchange site.

Build:
```shell
cargo build -- release
```

Run:
```shell
cargo run -- site date_start date_end
```

Example:
```shell
cargo run -- substrate 01/02/2023 23/02/2023
```

You can specify a list of user_ids from your site to collect metrics of your team.
Example:

```shell
cargo run -- substrate 02/03/2023 06/03/2023 --members 1 2 3 4
```

And also get the hot tags with the optional flag --tags:

```shell
cargo run -- substrate 02/03/2023 06/03/2023 --tags
```