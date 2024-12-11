# github-contributions-generator

Pad your GitHub contributions with an arbitrary amount of commits over a specified date range

## Disclaimer

This is just for fun; don't use it if you think it'll give you an edge up on others.

## Installation

```shell
cargo install --git https://github.com/yoonthegoon/github-contributions-generator.git
```

## Usage

```shell
$ alias gcg="github-contributions-generator"
$ gcg -h
Usage: github-contributions-generator [OPTIONS] --start-date <START_DATE>

Options:
  -s, --start-date <START_DATE>
          Date to start commits
  -e, --end-date <END_DATE>
          Date to end commits [default: 2024-12-11]
  -i, --ignore-dates [<IGNORE_DATES>...]
          List of dates not to commit on (e.g. 2024-11-28 2024-12-25 2025-01-01)
  -c, --commits <MIN> <MAX>
          Number of commits possible per weekday [default: "1 4"]
  -w, --include-weekends
          Does 0 commits on weekends if true
  -h, --help
          Print help
  -V, --version
          Print version
```

If you wanted to push 1-4 commits for every weekday in 2024 until and including today, in a GitHub repository's
top-level directory, execute:

```shell
touch gcg.txt
git add gcg.txt
# run the above only if you've not done this in the current directory before
gcg -s 2024-01-01
```

For each day between the start and end dates, a random number of times between min and max commits, a unique message
will be written to `gcg.txt` and commited. 

If instead you wanted to do 0-12 commits for every day (including weekends) in 2020 but excluding U.S. federal holidays, you can run:

```shell
gcg -s 2023-01-01 -e 2023-12-31 -c 0 12 -w -i 2023-01-02 2023-01-16 2023-02-20 2023-05-29 2023-06-19 2023-07-04 2023-09-04 2023-10-09 2023-11-10 2023-11-23 2023-12-25
```
