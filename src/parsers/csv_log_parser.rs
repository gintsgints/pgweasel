use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{convert_args::FileWithPath, parsers::LogParser};

pub struct CsvLogParser {
    reader: BufReader<File>,
}

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

impl CsvLogParser {
    pub fn new(file_with_path: FileWithPath) -> Self {
        Self {
            reader: BufReader::new(file_with_path.file),
        }
    }
}

impl LogParser for CsvLogParser {
    type Iter = Box<dyn Iterator<Item = Result<String>>>;

    fn parse(self, mask: String) -> Self::Iter {
        let iter = self.reader.lines().filter_map(move |lin| {
            let line = match lin {
                Ok(l) => l,
                Err(err) => return Some(Err(format!("Failed to read! Err: {err}").into())),
            };
            if !line.contains(&mask) {
                return None;
            };
            Some(Ok(line))
        });
        Box::new(iter)
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, path::PathBuf};

    use crate::parsers::csv_log_parser::CsvLogParser;

    use super::*;

    #[test]
    fn test_csv_parser() -> Result<()> {
        let path: PathBuf = PathBuf::from("./testdata/csvlog_pg14.csv");
        let file = File::open(path.clone())?;
        let parser = CsvLogParser::new(FileWithPath { file, path });

        let iter = parser.parse("2025-05-21 13:00:03.127".to_string());
        for line in iter {
            let line = line?;
            println!("{}", line);
        }

        Ok(())
    }
}
