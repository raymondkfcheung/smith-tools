use anyhow::Result;
use dot_pr_review::{
    Args,
    fetch::{Fetcher, IssueFilter},
};

#[tokio::main]
async fn main() -> Result<()> {
    // Prepare the search query
    let query = Args::prepare()?;

    // Page through results in case there are many
    let fectcher = Fetcher::new()?;
    let me = fectcher.current_user().await?;
    let mut page = fectcher.fetch(&query).await?;

    loop {
        for item in &page.items {
            if IssueFilter::should_ignore(item, &me) {
                continue;
            }

            let item_type = if item.pull_request.is_some() {
                "PR"
            } else {
                "Issue"
            };
            let author = &item.user;
            let assginee = if let Some(assigned) = &item.assignee {
                &assigned.login
            } else {
                "Unassigned"
            };

            let title = &item.title;
            let number = &item.number;
            let updated_at = &item.updated_at;
            let state = &item.state;
            let url = &item.html_url;
            let desc = &item.body.as_deref().unwrap_or_default();

            println!("----------------");
            println!("{item_type}#{number}  {title} (updated: {updated_at})");
            println!("    created by {}, assigned to {}", author.login, assginee);
            println!("    [{state:?}] {url}");
            if !desc.is_empty() {
                println!("{desc}");
            }
            println!();
        }

        if let Some(next) = fectcher.fetch_next(&page).await? {
            page = next;
        } else {
            break;
        }
    }

    Ok(())
}
