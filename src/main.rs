use std::fs::{read_dir, ReadDir};
use std::io;
use std::io::Write;
use std::path::Path;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "ls", about)]
struct Opt {
    #[structopt(short = "a")]
    all: bool,
    #[structopt(short = "A")]
    almost_all: bool,
}

const DEFAULT_TERM_LEN: usize = 100;

fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    ls(&opt)
}

fn ls(opt: &Opt) -> io::Result<()> {
    let path = Path::new(".");
    let dir: ReadDir = read_dir(path)?;

    let mut names: Vec<_> = dir
        .filter_map(|dir_entry| dir_entry.ok())
        .map(|dir_entry| {
            let mut name = dir_entry.file_name().to_string_lossy().to_string();
            if let Ok(metadata) = dir_entry.metadata() {
                if metadata.is_dir() {
                    name.push('\\');
                }
            }
            name
        })
        .collect();
    if !opt.all && !opt.almost_all {
        names = names
            .into_iter()
            .filter(|name| !name.starts_with('.'))
            .collect();
    } else if opt.all {
        names.push(String::from("."));
        names.push(String::from(".."));
    }
    let term_size = DEFAULT_TERM_LEN;
    names.sort();
    let max_length = names.iter().map(String::len).max().unwrap_or(1);
    let columns = term_size / (max_length + 1);

    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    let filler = String::from_utf8(vec![b' '; max_length + 1])
        .expect("an array of b' ' should be valid UTF-8");
    for chunk in names.chunks(columns) {
        for element in chunk {
            write!(handle, "{}", element)?;
            if columns > 1 {
                write!(handle, "{}", &filler[0..(max_length - element.len() + 2)])?;
            }
        }
        writeln!(handle)?;
    }

    Ok(())
}
