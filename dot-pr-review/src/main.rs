use anyhow::Result;
use dot_pr_review::{Args, fetch::Fetcher};

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
            // `item` is an Issue-like object, but `pull_request` is Some for PRs.
            let is_pr = item.pull_request.is_some();
            let author = &item.user;
            let assginee = &item.assignee.as_ref().unwrap_or(&author);
            if assginee.login != me.login {
                continue;
            }

            let title = &item.title;
            if title.contains("Backport #")
                && author.login.starts_with("paritytech-")
                && author.login.ends_with("[bot]")
            {
                continue;
            }

            let number = &item.number;
            let updated_at = &item.updated_at;
            let state = &item.state;
            let url = &item.html_url;
            let desc = &item.body.as_deref().unwrap_or_default();

            println!("----------------");
            println!(
                "{}#{number}  {title} (updated: {updated_at})",
                if is_pr { "PR" } else { "Issue" }
            );
            println!(
                "    created by {}, assigned to {}",
                author.login, assginee.login
            );
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
