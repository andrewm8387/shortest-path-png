pub(crate) fn parse(args: Vec<String>) -> Option<String> {
    //validate user input
    if args.len() != 2 {
        eprintln!("Usage: shortest-path-png <file_path>");
        return None;
    }
    let file_path = &args[1];
    if !file_path.ends_with(".png") {
        eprintln!("file type must be png");
        return None;
    }

    Some(file_path.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_args() {
        let args = vec!["shortest-path-png".to_string()];
        assert_eq!(parse(args), None);
    }

    #[test]
    fn test_wrong_file_type() {
        let args = vec!["shortest-path-png".to_string(), "test.jpg".to_string()];
        assert_eq!(parse(args), None);
    }

    #[test]
    fn test_correct_file_type() {
        let args = vec!["shortest-path-png".to_string(), "test.png".to_string()];
        assert_eq!(parse(args), Some("test.png".to_string()));
    }

    #[test]
    fn test_correct_file_type_with_path() {
        let args = vec!["shortest-path-png".to_string(), "/home/user/test.png".to_string()];
        assert_eq!(parse(args), Some("/home/user/test.png".to_string()));
    }

    #[test]
    fn test_correct_file_type_with_path_and_spaces() {
        let args = vec!["shortest-path-png".to_string(), "/home/user/test file.png".to_string()];
        assert_eq!(parse(args), Some("/home/user/test file.png".to_string()));
    }

    #[test]
    fn test_correct_file_type_with_path_and_spaces_and_png() {
        let args = vec!["shortest-path-png".to_string(), "/home/user/test file.png.png".to_string()];
        assert_eq!(parse(args), Some("/home/user/test file.png.png".to_string()));
    }
}