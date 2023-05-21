mod extract_ticket_from_branch;
mod read_file;
mod branch_name;
mod write_file;

use std::env;

use read_file::read_file;
use branch_name::get_branch_name;
use extract_ticket_from_branch::extract_ticket_from_branch_name;
use write_file::write_to_file;

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
    // let ticket_number = extract_ticket_from_branch_name(&branch_name);
    //

    let jira_commit_message = match extract_ticket_from_branch_name(&branch_name) {
        Ok(ticket_number) => {
            format!("[{}] {}", ticket_number, commit_message)
        },
        Err(_) => {
            println!("No ticket found in branch name: {}", branch_name);
            commit_message
        }
    };

    write_to_file(&commit_filename, &jira_commit_message);

    println!("Commit message: {}", jira_commit_message);

}

