mod extract_ticket_from_branch;
mod read_file;
mod branch_name;

use std::env;

use read_file::read_file;
use branch_name::get_branch_name;
use extract_ticket_from_branch::extract_ticket_from_branch_name;

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
    let ticket_number = extract_ticket_from_branch_name(&branch_name);
    let jira_commit_message = format!("[{}] {}", ticket_number, commit_message);

    write_to_file(&commit_filename, &jira_commit_message);

    println!("[{}] {}", ticket_number, commit_message);

}

fn write_to_file(filename: &str, content: &str) -> () {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(filename).expect("Unable to create file");
    file.write_all(content.as_bytes()).expect("Unable to write data");
}
