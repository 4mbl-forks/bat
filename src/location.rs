use std::path::{Path, PathBuf};

use crate::line_range::LineRange;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    fn parse(raw: &str) -> Option<Self> {
        let (start, end) = if let Some((a, b)) = raw.split_once('-') {
            (parse_bound(a, 1)?, parse_bound(b, usize::MAX)?)
        } else {
            let n = raw.parse::<usize>().ok().filter(|n| *n > 0)?;
            (n, n)
        };
        (start <= end).then_some(Self { start, end })
    }
}

fn parse_bound(raw: &str, default: usize) -> Option<usize> {
    if raw.is_empty() {
        Some(default)
    } else {
        raw.parse().ok().filter(|n| *n > 0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Location {
    pub lines: Span,
    pub columns: Span,
}

impl Location {
    pub fn line_range(self) -> LineRange {
        LineRange::new(self.lines.start, self.lines.end)
    }
}

pub fn parse_path(path: &Path) -> Option<(PathBuf, Location)> {
    if path.exists() {
        return None;
    }
    let raw = path.to_str()?;
    // Work backwards so colons in filenames and drive prefixes remain intact.
    for (index, _) in raw.match_indices(':').rev() {
        let prefix = &raw[..index];
        if prefix.is_empty() {
            continue;
        }
        let suffix = &raw[index + 1..];
        let (lines, columns) = if let Some((line, column)) = suffix.split_once(':') {
            (
                if line.is_empty() {
                    Span {
                        start: 1,
                        end: usize::MAX,
                    }
                } else {
                    Span::parse(line)?
                },
                Span::parse(column)?,
            )
        } else if suffix.is_empty() {
            continue;
        } else {
            (
                Span::parse(suffix)?,
                Span {
                    start: 1,
                    end: usize::MAX,
                },
            )
        };
        if Path::new(prefix).is_file() {
            return Some((PathBuf::from(prefix), Location { lines, columns }));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span_syntax() {
        assert_eq!(Span::parse("12").unwrap(), Span { start: 12, end: 12 });
        assert_eq!(
            Span::parse("12-").unwrap(),
            Span {
                start: 12,
                end: usize::MAX
            }
        );
        assert_eq!(Span::parse("-20").unwrap(), Span { start: 1, end: 20 });
        assert_eq!(
            Span::parse("-").unwrap(),
            Span {
                start: 1,
                end: usize::MAX
            }
        );
        for invalid in ["0", "0-2", "3-2", "x", "", "1--2"] {
            assert!(Span::parse(invalid).is_none(), "{invalid}");
        }
    }

    #[test]
    fn path_syntax_and_existing_filename() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("file:part.md");
        std::fs::write(&file, "hello").unwrap();
        assert!(parse_path(&file).is_none());
        let (parsed, location) =
            parse_path(&PathBuf::from(format!("{}:12-20:5-10", file.display()))).unwrap();
        assert_eq!(parsed, file);
        assert_eq!(location.lines, Span { start: 12, end: 20 });
        assert_eq!(location.columns, Span { start: 5, end: 10 });
        let (_, location) = parse_path(&PathBuf::from(format!("{}::5", file.display()))).unwrap();
        assert_eq!(
            location.lines,
            Span {
                start: 1,
                end: usize::MAX
            }
        );
        assert_eq!(location.columns, Span { start: 5, end: 5 });
    }
}
