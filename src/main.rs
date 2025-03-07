use clap::Parser;
use git2::Repository;
use reqwest::blocking::Client;
use serde_json::json;

fn is_first_commit(repo: &Repository) -> bool {
    repo.head().is_err() // If HEAD doesn't exist, it's the first commit
}

fn get_git_diff() -> String {
    let repo = Repository::open(".").expect("Failed to open repository");

    if is_first_commit(&repo) {
        return "This is the first commit. Generate an initial commit message.".to_string();
    }

    let head = repo.head().expect("Failed to get HEAD");
    let tree = head.peel_to_tree().expect("Failed to get tree");
    let diff = repo
        .diff_tree_to_workdir(Some(&tree), None)
        .expect("Failed to get diff");

    let mut diff_text = String::new();
    diff.print(git2::DiffFormat::Patch, |_, _, line| {
        diff_text.push_str(std::str::from_utf8(line.content()).unwrap());
        true
    })
    .unwrap();

    diff_text
}

fn generate_commit_message(diff: &str) -> String {
    let api_key = "your_openai_api_key";
    let client = Client::new();
    let response = client.post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": "gpt-4",
            "messages": [
                {"role": "system", "content": "You are a helpful assistant that writes concise, meaningful Git commit messages."},
                {"role": "user", "content": format!("Analyze the following Git diff and suggest a commit message:\n\n{}", diff)}
            ]
        }))
        .send()
        .expect("Failed to call OpenAI API");

    let json_response: serde_json::Value = response.json().expect("Failed to parse response");
    json_response["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("Generated commit message failed")
        .to_string()
}

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    commit: bool,
}

fn main() {
    let args = Args::parse();
    let diff = get_git_diff();
    let commit_message = generate_commit_message(&diff);

    println!("Suggested commit message:\n{}", commit_message);

    if args.commit {
        std::process::Command::new("git")
            .args(["commit", "-m", &commit_message])
            .status()
            .expect("Failed to execute Git commit");
    }
}
