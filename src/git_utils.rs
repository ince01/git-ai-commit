use git2::{Error, Repository, Signature};
use std::env;
use tokio::task;

fn is_first_commit(repo: &Repository) -> bool {
    repo.head().is_err() // If HEAD doesn't exist, it's the first commit
}

fn get_git_repo() -> Result<Repository, Error> {
    let repo_path = env::current_dir().map_err(|e| Error::from_str(&e.to_string()))?;
    Repository::discover(&repo_path)
}

pub async fn get_git_diff() -> Result<String, Error> {
    task::spawn_blocking(move || {
        let repo = get_git_repo()?;

        if is_first_commit(&repo) {
            return Ok("🌱 Initial commit to kickstart the project 🚀".to_string());
        }

        let head = repo.head().expect("Failed to get HEAD");
        let tree = head.peel_to_tree().expect("Failed to get tree");

        let diff = repo.diff_tree_to_index(Some(&tree), None, None)?;

        let mut diff_text = String::new();
        diff.print(git2::DiffFormat::Patch, |_, _, line| {
            diff_text.push_str(std::str::from_utf8(line.content()).unwrap());
            true
        })?;

        if diff_text.trim().is_empty() {
            return Err(Error::from_str("No staged changes to commit"));
        }

        Ok(diff_text)
    })
    .await
    .expect("Task panicked")
}

pub async fn commit_staged_files(message: String) -> Result<(), Error> {
    // Run the Git operations in a blocking task
    task::spawn_blocking(move || {
        // Open the repository
        let repo = get_git_repo()?;

        // Get the index (staging area)
        let mut index = repo.index()?;

        // Create a tree from the index
        let tree_oid = index.write_tree()?;
        let tree = repo.find_tree(tree_oid)?;

        // Get the current HEAD reference
        let head = repo.head().ok(); // HEAD might not exist in a new repo
        let parent_commit = head.and_then(|h| h.peel_to_commit().ok());

        // Get user name and email from git config
        let config = repo.config()?;
        let name = config
            .get_string("user.name")
            .unwrap_or_else(|_| "Unknown".to_string());
        let email = config
            .get_string("user.email")
            .unwrap_or_else(|_| "unknown@example.com".to_string());

        // Create a signature (author and committer)
        let sig = Signature::now(&name, &email)?;

        // Create the commit
        repo.commit(
            Some("HEAD"), // Update HEAD to point to the new commit
            &sig,         // Author
            &sig,         // Committer
            &message,     // Commit message
            &tree,        // Tree
            parent_commit
                .as_ref()
                .map(|c| vec![c])
                .unwrap_or_else(Vec::new)
                .as_slice(),
        )?;

        println!("👨🏽‍💻 Committed successfully: {}", message);
        Ok::<(), Error>(())
    })
    .await
    .map_err(|e| Error::from_str(&e.to_string()))??;

    Ok(())
}
