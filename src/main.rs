use clap::Parser;
use rand::rng;
use rand::seq::SliceRandom;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct Issue {
    html_url: String,
    title: String,
    labels: Vec<Label>,
}

#[derive(Deserialize, Debug, Clone)]
struct Label {
    name: String,
}

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    after_help = "use `,` for list\n./bin arg1 arg2 --arg3 val1,val2 --arg4 val1,val2"
)]
struct Args {
    #[arg(required = true)]
    github_username_or_organization: String,
    #[arg(required = true)]
    repository_name: String,
    #[arg(long = "incl-labels", value_delimiter = ',')]
    labels_to_include: Option<Vec<String>>,
    #[arg(long = "excl-labels", value_delimiter = ',')]
    labels_to_exclude: Option<Vec<String>>,
}

fn main() {
    let args = Args::parse();

    let uri = format!(
        "repos/{}/{}/issues?state=open",
        args.github_username_or_organization, args.repository_name
    );

    let result = std::process::Command::new("gh")
        .args(["api", &uri, "--method", "GET", "--paginate"])
        .output();

    if let Ok(result) = result {
        let result = String::from_utf8_lossy(&result.stdout);
        let issues: Vec<Issue> = serde_json::from_str(&result).unwrap();

        let mut issues: Vec<Issue> = issues
            .iter()
            .filter(|issue| {
                issue.labels.iter().any(|label| {
                    args.labels_to_include
                        .clone()
                        .unwrap_or(vec![])
                        .contains(&label.name)
                })
            })
            .filter(|issue| {
                issue.labels.iter().all(|label| {
                    !args
                        .labels_to_exclude
                        .clone()
                        .unwrap_or(vec![])
                        .contains(&label.name)
                })
            })
            .cloned()
            .collect();

        issues.shuffle(&mut rng());

        println!(
            "Randomly picked issue:\n\n{}\n\n{}",
            issues[0].title, issues[0].html_url
        )
    } else {
        println!("Unable to fetch issues")
    }
}
