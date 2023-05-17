mod extract_ticket_from_branch;
mod read_file;

use std::env;
use std::print;

use read_file::read_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // expect arguments to have at least size 2
    if args.len() < 2 {
        println!("Not enough arguments");
        return;
    }

    // only accept 'message' or nothing as argument[2]
    if let Some(commit_source) = args.get(2) {
        if commit_source != "message" {
            println!("Type of the commit should be 'message'");
            return;
        }
    }
    
    let branch_name = get_branch_name();
    let commit_filename = args.get(1).expect("No commit filename given");
    let commit_message = read_file(commit_filename);

    print!("Commit message: {}", commit_message);
    print!("Branch name: {}", branch_name);

}

fn get_branch_name() -> String {
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
