use std::io;
use std::fs::{self};
use std::path::Path;
use std::env;
use filetime::FileTime;
use chrono::{NaiveDateTime};

struct FileData<'a> {
    modtime: FileTime,
    path: &'a Path,
    size: u32,

}

fn list_files(dir: &Path) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            if let Ok(entry) = entry{
                if let Ok(metadata) = entry.metadata() {
                    let time_struct = FileTime::from_last_modification_time(&metadata);
                    let mtime = time_struct.seconds();
                    let date_since_mod = NaiveDateTime::from_timestamp(mtime, 0);

                    println!("{:?}\t{}\t{:?}", entry.path().to_str().unwrap(), date_since_mod, metadata.len());
                }
            }

        }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match list_files(Path::new(&args[1])) {
        Ok(()) => println!("Operation ran"),
        Err(e) => println!("Error:\t{}", e),
    }

    println!();


}
