use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use std::{env, fs};

fn replace_in_file(file_path: &PathBuf, old: &str, new: &str) -> io::Result<()> {
    let mut contents = String::new();
    File::open(file_path)?.read_to_string(&mut contents)?;
    let new_contents = contents.replace(old, new);
    if contents != new_contents {
        println!("Updating {}", file_path.display());
        fs::write(file_path, new_contents)?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let pkg_name = env::var("CARGO_PKG_NAME").unwrap();
    replace_in_file(
        &"../Cargo.toml".into(),
        &format!("# \"{pkg_name}\""),
        &format!("\"{pkg_name}\""),
    )?;

    replace_in_file(&"./Cargo.toml".into(), "\n[workspace]", "")?;

    Ok(())
}
