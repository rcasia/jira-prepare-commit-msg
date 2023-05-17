
pub fn get_branch_name() -> String {
    let branch_name : String = std::process::Command::new("git")
        .args(&["branch", "--show-current"])
        .output()
        .expect("Failed to execute command")
        .stdout
        .iter()
        .map(|&c| c as char)
        .collect();

    branch_name
}
