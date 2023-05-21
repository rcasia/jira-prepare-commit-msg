use regex::Regex;

pub fn extract_ticket_from_branch_name(branch_name: &str) -> Result<&str, ()> {

    let re = Regex::new(r"[a-z]*/?([A-Z]+-\d+)(?:-.*)?").unwrap();


    match re.captures(branch_name) {
        Some(caps) => {
            Ok(caps.get(1).unwrap().as_str())
        },
        None => {
            println!("No ticket found in branch name: {}", branch_name);
            Err(())
        }
    }

}

#[cfg(test)]
mod tests {

    use super::extract_ticket_from_branch_name;

    #[test]
    fn test_extracts_ticket_from_branch_name() {
        // given
        // list of posible branch names
        let branch_names = vec![
            "TEST-1234",
            "feature/TEST-1234",
            "feature/TEST-1234-foo",
            "hotfix/TEST-1234-foo",
            "release/TEST-1234-foo",
            "TEST-1234/homepage",
            "TEST-1234-foo",
        ];
        
        for branch_name in branch_names {
            // when
            let result = extract_ticket_from_branch_name(branch_name);

            // then
            assert!(result.is_ok());
            let actual = result.unwrap();
            assert_eq!(actual, "TEST-1234");
        }
    }

    #[test]
    fn test_should_not_panic_when_no_ticket_found() {
        // given
        let branch_name = "chill-branch-name";

        // when
        // let actual = std::panic::catch_unwind(|| extract_ticket_from_branch_name(branch_name));
        let result = extract_ticket_from_branch_name(branch_name);

        // then
        assert!(result.is_err());
    }

}
