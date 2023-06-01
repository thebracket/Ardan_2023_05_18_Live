use std::path::Path;

fn main() {
    let my_file = Path::new("mytile.txt");
    // This yields a Result type of String or an error
    let contents = std::fs::read_to_string(my_file);
    // Let's just handle the error by printing it out
    match contents {
        Ok(contents) => println!("File contents: {contents}"),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => println!("File not found"),
            std::io::ErrorKind::PermissionDenied => println!("Permission denied"),
            _ => println!("ERROR: {e:#?}"),
        },
    }
}