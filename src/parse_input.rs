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