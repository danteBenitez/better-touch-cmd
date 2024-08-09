use std::path::Path;
use std::{env, io};
use std::fs::{self, OpenOptions};

fn main() -> io::Result<()> {
    let mut args = env::args();
    args.next();

    let path = args.next().expect("Please provide a path");
    let path = Path::new(&path);

    if path.ends_with("/") {
        fs::create_dir_all(path)?;
        return Ok(())
    }

    if let Some(dirname) = Path::new(&path).parent() {
        fs::create_dir_all(dirname)?;
    }

    let mut opts = OpenOptions::new();
    opts.append(true);
    opts.create(true);

    opts.open(path).expect("File could not be created");

    Ok(())
}

