use clap::Parser;
mod ai_prompt;
mod gemini_api;
mod git_utils;

#[derive(Parser, Debug)]
#[command(author, version, about = "Generate emoji-rich Git commit messages with Gemini AI", long_about = None)]
struct Args {
    /// Enable debug output
    #[arg(short, long)]
    debug: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let git_diff_result = git_utils::get_git_diff().await; // Await the future

    let git_diff = match git_diff_result {
        Ok(diff) => diff,
        Err(e) => {
            eprintln!("🚨 Error getting git diff: {}", e);
            return;
        }
    };

    match gemini_api::generate_commit_message_with_gemini(&git_diff, args.debug).await {
        // Pass a reference to the result
        Ok(msg) => println!("💡 Suggested Commit: {}", msg),
        Err(e) => eprintln!("🚨 Error: {}", e),
    }
}
