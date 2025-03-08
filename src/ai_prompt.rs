pub fn get_ai_commit_message_prompt(diff: &str) -> String {
    format!(
        "You are a skilled software developer assistant tasked with generating a concise and descriptive Git commit message based on the provided Git diff output. Below is the diff output showing the changes made to the codebase:\n\n{}\n\nUsing the diff output above, create a single-line Git commit message that:\n- Starts with an emoji reflecting the change type (e.g., ➕ for adds, 🐛 for fixes, 🔄 for updates, ➖ for removals).\n- Follows with an imperative verb (e.g., \"Add\", \"Fix\", \"Update\", \"Remove\").\n- Summarizes the changes in 50-72 characters, weaving in extra emojis (e.g., 📄 for files, 🚀 for improvements, ✅ for success, ❗ for issues) where they clarify or enhance meaning.\n- Focuses on what was changed and why, avoiding vague terms.\n- Ignores minor formatting/whitespace changes unless significant.\n\nReturn the commit message as a plain text string.",
        diff
    )
}
