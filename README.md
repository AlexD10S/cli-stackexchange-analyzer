# CLI StackExchange Analyser
CLI tool to get metrics from a stackexchange site built with Rust 🦀.

Build:
```shell
cargo build -- release
```

Run:
```shell
cargo run -- site date_start date_end
```
The format of the date is dd/mm/YYYY 

Example:
```shell
cargo run -- substrate 01/02/2023 23/02/2023
```

Example with all flags:

```shell
cargo run -- substrate 27/02/2023 28/02/2023 --members 2762 29 --tags --individual --unanswered
```

### Team Metrics
You can specify a list of user_ids from your site to collect metrics of your team.
Example:

```shell
cargo run -- substrate 02/03/2023 06/03/2023 --members 2762 29 
```

### Team Individual Metrics
Use the optional flag --individual if want to retrieve the individual information of the team members:

```shell
cargo run -- substrate 02/03/2023 06/03/2023  --members 2762 29 --individual
```

### Tags Metrics
Also get the hot tags with the optional flag --tags:

```shell
cargo run -- substrate 02/03/2023 06/03/2023 --tags
```

### Export metrics in a CSV
And finally you can export the data in a CSV file instead of printing it in the console with the optional flag --export:

```shell
cargo run -- substrate 02/03/2023 06/03/2023 --export
```


### API KEY
If you have an API KEY to do queries to the StackExchange API: https://api.stackexchange.com/docs 

Create a `.env` file and add the variable `API_KEY` with your key like in the `.env.example` file
This allows you to do more queries