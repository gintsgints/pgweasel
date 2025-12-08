mod csv_log_parser;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

// #[derive(Debug)]
// pub struct LogLine {
//     pub severity: Severity,
//     pub raw: String,
// }

/// Trait for all parsers: produce an iterator over filtered log lines.
pub trait LogParser {
    type Iter: Iterator<Item = Result<String>>;

    fn parse(self, mask: String) -> Self::Iter;
}
