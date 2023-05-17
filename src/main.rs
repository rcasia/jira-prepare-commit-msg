mod extract_ticket_from_branch;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // only accept 'message' or nothing as argument[2]
    if let Some(commit_source) = args.get(2) {
        if commit_source != "message" {
            println!("Type of the commit should be 'message'");
            return;
        }
    }
    
    for idx in 0..args.len() {
        println!("arg[{}]: {}", idx, args[idx]);
    }
}


#[cfg(test)]
mod tests {

}
