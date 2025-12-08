use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use chrono::{DateTime, Local};
use csv::ReaderBuilder;
use log::error;

use crate::{
    convert_args::FileWithPath,
    errors::{PostgresLog, Severity},
    parsers::{LogLine, LogParser},
};

pub struct CsvLogParser {
    csv_reader: csv::Reader<BufReader<File>>,
}

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

impl CsvLogParser {
    pub fn new(file_with_path: FileWithPath) -> Self {
        let reader = BufReader::new(file_with_path.file);
        let csv_reader = ReaderBuilder::new()
            .has_headers(false)
            .flexible(true) // Allow variable number of columns
            .from_reader(reader);

        Self { csv_reader }
    }
}

impl LogParser for CsvLogParser {
    type Iter = Box<dyn Iterator<Item = Result<LogLine>>>;

    fn parse(
        mut self,
        min_severity: i32,
        mask: Option<String>,
        begin: Option<DateTime<Local>>,
        end: Option<DateTime<Local>>,
    ) -> Self::Iter {
        let iter = self.csv_reader.records().filter_map(move |rec| {
            let record = match rec {
                Ok(l) => l,
                Err(err) => return Some(Err(format!("Failed to read! Err: {err}").into())),
            };
            let level: Severity = record[11].to_string().into();
            let log_level_num: i32 = (&level).into();
            if log_level_num < min_severity {
                return None;
            }
            if let Some(some_mask) = &mask {
                if !record[11].starts_with(some_mask) {
                    return None;
                };
            }
            let log_record: PostgresLog = match record.deserialize(None) {
                Ok(rec) => rec,
                Err(e) => {
                    error!("Error deserializing CSV record in file. {}", e);
                    return None;
                }
            };
            if let Some(log_time) = log_record.log_time {
                let log_time_local = log_time.with_timezone(&chrono::Local);
                if let Some(begin) = begin {
                    if log_time_local < begin {
                        return None;
                    }
                }
                if let Some(end) = end {
                    if log_time_local > end {
                        return None;
                    }
                }
            }
            
            Some(Ok(LogLine { pg_log: Some(log_record), raw: "".to_string() }))
        });
        Box::new(iter)
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, path::PathBuf};

    use crate::{errors::Severity, parsers::csv_log_parser::CsvLogParser};

    use super::*;

    #[test]
    fn test_csv_parser() -> Result<()> {
        let path: PathBuf = PathBuf::from("./testdata/csvlog_pg14.csv");
        let file = File::open(path.clone())?;
        let parser = CsvLogParser::new(FileWithPath { file, path });

        let intseverity = (&(Severity::LOG)).into();
        let iter = parser.parse(
            intseverity,
            Some("2025-05-21 13:00:03.127".to_string()),
            None,
            None,
        );
        for line in iter {
            let line = line?;
            println!("{:?}", line);
        }

        Ok(())
    }
}
