use git2::{Error, Repository};
use tokio::task;

pub fn is_first_commit(repo: &Repository) -> bool {
    repo.head().is_err() // If HEAD doesn't exist, it's the first commit
}

pub async fn get_git_diff() -> Result<String, Error> {
    task::spawn_blocking(move || {
        let repo = Repository::open(".").expect("Failed to open repository");

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
