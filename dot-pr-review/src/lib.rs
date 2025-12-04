use anyhow::{Result, anyhow};
use chrono::NaiveDate;
use clap::Parser;

pub mod error;
pub mod fetch;

#[derive(Parser, Debug)]
#[command(name = "dot-pr-review")]
#[command(about = "List PRs created by or assigned to you, filtered by updated date")]
pub struct Args {
    /// Updated on or after this date (YYYY-MM-DD)
    #[arg(long, value_name = "YYYY-MM-DD")]
    updated_since: String,

    /// Optional `repo` filter: owner/name (e.g. paritytech/polkadot-sdk)
    #[arg(long)]
    repo: Option<String>,

    /// Optional `is` filter (e.g. issue, pr)
    #[arg(long, value_name = "FILTER")]
    is: Option<String>,

    /// Optional `state` filter (e.g. open, closed)
    #[arg(long, value_name = "STATE")]
    state: Option<String>,
}

impl Args {
    pub fn prepare() -> Result<String> {
        let args = Self::parse();

        // Parse date
        let date = NaiveDate::parse_from_str(&args.updated_since, "%Y-%m-%d")
            .map_err(|e| anyhow!("invalid date '{}': {e:?}", args.updated_since))?;

        // Turn into ISO date string (GitHub search only needs the date part)
        let date_str = date.format("%Y-%m-%d").to_string();

        // Prepare the GitHub search query
        let mut query = format!("involves:@me updated:>={date_str}");

        if let Some(repo) = &args.repo {
            query.push(' ');
            query.push_str(&format!("repo:{repo}"));
        }

        if let Some(is_filter) = &args.is {
            query.push_str(&format!(" is:{is_filter}"));
        }

        if let Some(state) = &args.state {
            query.push_str(&format!(" state:{state}"));
        }

        query.push_str(&format!(" sort:updated-desc"));

        Ok(query)
    }
}
