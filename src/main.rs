use chrono::{Datelike, Duration, Local, NaiveDate, Weekday};
use clap::Parser;
use rand::Rng;
use std::fmt::Debug;
use std::fs;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Date to start commits
    #[arg(short, long)]
    start_date: NaiveDate,

    /// Date to end commits
    #[arg(short, long, default_value_t = Local::now().date_naive())]
    end_date: NaiveDate,

    /// List of dates not to commit on
    /// (e.g. 2024-11-28 2024-12-25 2025-01-01)
    #[arg(short, long, num_args = 0.., value_delimiter = ' ')]
    ignore_dates: Vec<NaiveDate>,

    /// Number of commits possible per weekday
    #[arg(short, long, value_names = ["MIN", "MAX"], default_value = "1 4", num_args=2, value_delimiter = ' ')]
    commits: Vec<usize>,

    /// Does 0 commits on weekends if true
    #[arg(short = 'w', long, default_value_t = false)]
    include_weekends: bool,
}

fn main() {
    let args = Args::parse();
    let mut rng = rand::thread_rng();

    let start_date = args.start_date;
    let end_date = args.end_date;
    let ignore_dates = args.ignore_dates;
    let min = args.commits[0];
    let max = args.commits[1];
    let include_weekends = args.include_weekends;

    let mut cur = start_date;
    while cur <= end_date {
        if !include_weekends && [Weekday::Sat, Weekday::Sun].contains(&cur.weekday())
            || ignore_dates.contains(&cur)
        {
            cur += Duration::days(1);
            continue;
        }

        for i in min..=rng.gen_range(min..=max) {
            let key = format!("{cur} {i}");
            print!("\r{key}");
            fs::write("commit.txt", &key).unwrap();
            Command::new("git")
                .arg("commit")
                .arg("--date")
                .arg(cur.to_string())
                .arg("-am")
                .arg(&key)
                .output()
                .unwrap();
        }
        cur += Duration::days(1);
    }
    println!();
    Command::new("git").arg("push").output().unwrap();
}
