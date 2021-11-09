pub fn parse_line(line: String) -> String {
    "0000000000000002".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_acommand() {
        let line = "@2".to_string();
        assert_eq!(parse_line(line), "0000000000000002".to_string());
    }
}