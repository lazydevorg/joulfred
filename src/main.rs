use chrono::{Local};
use std::borrow::{BorrowMut};
use std::env::current_dir;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{PathBuf};

const NEW_LINE: u8 = 10;

struct EntryHandler {
    file: File,
}

impl EntryHandler {
    fn new(base_path: &str, journal_name: &str) -> EntryHandler {
        let path = Self::file_path(base_path, journal_name);
        Self::add_new_line(&path);
        let file = Self::file(path);
        EntryHandler { file }
    }

    fn add_new_line(path: &PathBuf) -> bool {
        OpenOptions::new()
            .read(true)
            .append(true)
            .open(&path)
            .as_mut()
            .map(|file| {
                file.seek(SeekFrom::End(-1)).unwrap();
                let mut char = [0; 1];
                file.read(&mut char).unwrap();
                if char != [NEW_LINE] {
                    file.write(&[NEW_LINE]).unwrap();
                    return true
                }
                false
            })
            .unwrap_or(false)
    }

    fn file_path(base_path: &str, journal_name: &str) -> PathBuf {
        let today = Local::today();
        let year = today.format("%Y").to_string();
        let month = today.format("%m").to_string();
        let day = today.format("%d.txt").to_string();
        PathBuf::from(base_path)
            .join(journal_name)
            .join(year)
            .join(month)
            .join(day)
    }

    fn file(path: PathBuf) -> File {
        let dir = path.parent().unwrap();
        create_dir_all(dir).unwrap();
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap()
    }

    fn write_entry(&mut self, entry: &str) {
        let now = Local::now();
        let line = format!("{}: {}\n", now, entry);
        let file = self.file.borrow_mut();
        file.write_all(line.as_ref()).unwrap();
    }
}

fn main() {
    let base_path = current_dir().map(|p| p.join("tmp")).unwrap();
    let base_path = base_path.as_path().to_str().unwrap();
    let journal_name = "pilot";
    let mut fh = EntryHandler::new(base_path, journal_name);
    fh.write_entry("Second pilot line #lines");
}
