use regex::Regex;

pub fn extract_ticket_from_branch_name(branch_name: &str) -> &str {

    let re = Regex::new(r"[a-z]*/?([A-Z]+-\d+)(?:-.*)?").unwrap();

    let ticket = re.captures(branch_name)
        .expect(format!("No jira ticket found for {}", branch_name).as_str())
        .get(1)
        .unwrap()
        .as_str();

    return ticket
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
            let actual = extract_ticket_from_branch_name(branch_name);

            // then
            assert_eq!(actual, "TEST-1234");
        }
    }

    #[test]
    fn test_panics_when_no_ticket_found() {
        // given
        let branch_name = "feature/TEST-foo";

        // when
        let actual = std::panic::catch_unwind(|| extract_ticket_from_branch_name(branch_name));

        // then
        assert!(actual.is_err());
        assert!(actual.unwrap_err().downcast_ref::<String>().unwrap().contains("No jira ticket found for"));
    }

}
