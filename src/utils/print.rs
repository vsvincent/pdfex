pub fn print_paths_in_current_dir() -> Result<(), std::io::Error> {
    let paths = std::fs::read_dir(".")?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect::<Vec<_>>();
    println!("Files in the current directory:");
    for path in paths {
        println!("{}", path.display());
    }
    Ok(())
}
