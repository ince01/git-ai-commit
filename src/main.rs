use clap::Parser;
use std::io;
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
        Ok(msg) => {
            println!("💡 Suggested Commit: {}", msg);

            // Ask the user if they want to auto-commit
            println!("Do you want to auto-commit with this message? (y/n)");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("🚨 Failed to read input");
            let input = input.trim().to_lowercase();

            if input == "y" {
                // Auto-commit with the generated message
                if let Err(e) = git_utils::commit_staged_files(msg).await {
                    eprintln!("🚨 Error committing: {}", e);
                }
            } else {
                println!("👋 Aborted.");
            }
        }
        Err(e) => eprintln!("🚨 Error: {}", e),
    }
}
