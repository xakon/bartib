use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::str::FromStr;

use crate::activity;

pub enum LineStatus {
    Unchanged,
    Changed,
}

// a line in a bartib file
pub struct Line {
    // the plaintext of the line as it has been read from the file
    // we save this to be able write untouched lines back to file without chaning them
    plaintext: String,
    // the result of parsing this line to a activity
    pub activity: Result<activity::Activity, activity::ActivityError>,
    // the status of this activity
    status: LineStatus,
}

impl Line {
    // creates a new line struct from plaintext
    pub fn new(plaintext: &str) -> Line {
        Line {
            plaintext: plaintext.trim().to_string(),
            activity: activity::Activity::from_str(plaintext),
            status: LineStatus::Unchanged,
        }
    }

    // creates a new line from an existing activity
    pub fn for_activity(activity: activity::Activity) -> Line {
        Line {
            plaintext: "".to_string(),
            activity: Ok(activity),
            status: LineStatus::Changed,
        }
    }

    // sets the status of the line to changed
    pub fn set_changed(&mut self) {
        self.status = LineStatus::Changed;
    }
}

// reads the content of a file to a vector of lines
pub fn get_file_content(file_name: &str) -> Vec<Line> {
    let file_handler = File::open(file_name).unwrap();
    let reader = BufReader::new(file_handler);

    reader
        .lines()
        .filter_map(|line_result| line_result.ok())
        .map(|line| Line::new(&line))
        .collect()
}

// writes a vector of lines into a file
pub fn write_to_file(file_name: &str, file_content: &[Line]) -> Result<(), io::Error> {
    let file_handler = get_bartib_file_writable(file_name)?;

    for line in file_content {
        match line.status {
            LineStatus::Unchanged => writeln!(&file_handler, "{}", line.plaintext)?,
            LineStatus::Changed => write!(&file_handler, "{}", line.activity.as_ref().unwrap())?,
        }
    }

    Ok(())
}

// create a write handle to a file
fn get_bartib_file_writable(file_name: &str) -> Result<File, io::Error> {
    OpenOptions::new().create(true).write(true).open(file_name)
}
