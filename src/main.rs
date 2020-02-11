use chrono::NaiveDateTime;
use filetime::FileTime;
use std::env;
use std::fs::{self};
use std::{io, fmt};
use std::path::{Path, PathBuf};

pub struct FileData {
    modtime: FileTime,
    mtime_seconds: i64,
    path: PathBuf,
    size: u64,
}

impl fmt::Display for FileData {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let date_since_mod = FileData::seconds_to_date(self.mtime_seconds);
        let mut path_string = self.path.to_str().unwrap();
        if path_string.len() > 26 {
            let mut path_string_short = String::from(&path_string[0..26]);
            path_string_short.push_str("...");
            write!(f, "{:<30}\t{:<20}\t{}", path_string_short, date_since_mod, self.size)
        }
        else {
            write!(f, "{:<30}\t{:<20}\t{}", path_string, date_since_mod, self.size)
        }

    }
}

impl FileData {
    fn new() -> FileData {
        FileData {
            modtime: FileTime::zero(),
            mtime_seconds: 0,
            path: PathBuf::new(),
            size: 0,
        }
    }
    fn list_files_dir(&mut self, dir: &Path) -> io::Result<()> {
        println!("PATH\t\tLast Modified\t\tSize");
        for entry in fs::read_dir(dir)? {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    self.path = entry.path();
                    self.modtime = FileTime::from_last_modification_time(&metadata);
                    self.size = metadata.len();
                    self.mtime_seconds = self.modtime.seconds();

                    println!("{}", self);
                }
            }
        }
        Ok(())
    }

    fn seconds_to_date(s: i64) -> NaiveDateTime {
        NaiveDateTime::from_timestamp(s, 0)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut time_struct = FileData::new();

    match time_struct.list_files_dir(Path::new(&args[1])) {
        Ok(()) => println!("Operation ran"),
        Err(e) => println!("Error:\t{}", e),
    }

    println!();
}
