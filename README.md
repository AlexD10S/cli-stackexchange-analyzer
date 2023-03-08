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
cargo run -- substrate 02/03/2023 06/03/2023 --members 1 2 3 4
```

### Team Individual Metrics
Use the optional flag --individual if want to retrieve the individual information of the team members:

```shell
cargo run -- substrate 02/03/2023 06/03/2023  --members 1 2 3 4 --individual
```

### Tags Metrics
And also get the hot tags with the optional flag --tags:

```shell
cargo run -- substrate 02/03/2023 06/03/2023 --tags
```

### Unanswered Questions Metrics
For Stack Exchange API an unanswered questions is a question that:
 - The question has an accepted answer
 - The question has an answer with a score > 0

* See https://stackapps.com/questions/4227/stack-exchange-api-is-returning-booleanfalse-for-is-answered-on-question-wit


If you want to analyse more about your unanswered questions add the flag --unanswered with your team members list

 ```shell
    cargo run -- substrate 02/03/2023 06/03/2023 --members 1 2 3 4 --unanswered
```

### API KEY
If you have an API KEY to do queries to the StackExchange API: https://api.stackexchange.com/docs 

Create a `.env` file and add the variable `API_KEY` with your key like in the `.env.example` file
This allows you to do more queries