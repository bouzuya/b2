use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::str::FromStr as _;

use anyhow::Context as _;

use crate::Config;

#[derive(clap::Args)]
pub struct Args {
    /// Date to list, equivalent to --since {date} --until {date+1d}
    #[clap()]
    pub date: Option<String>,

    /// Limit the number of results
    #[clap(long, default_value_t = 20)]
    pub limit: usize,

    /// Sort the results
    #[clap(default_value_t = Order::Desc, long, value_enum)]
    pub order: Order,

    /// List entries since this id (inclusive)
    #[clap(long)]
    pub since: Option<String>,

    /// List entries until this id (exclusive)
    #[clap(long)]
    pub until: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, clap::ValueEnum)]
pub enum Order {
    Asc,
    Desc,
}

pub fn execute(
    Args {
        date,
        limit,
        order,
        since,
        until,
    }: Args,
) -> anyhow::Result<()> {
    let (since, until) = match date {
        Some(date) => {
            if since.is_some() || until.is_some() {
                anyhow::bail!("date and since/until are mutually exclusive");
            }

            let date = chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d")?;
            let since = IdLike::from_str(&format!("{}T000000Z", date.format("%Y%m%d")))?;
            let until = IdLike::from_str(&format!(
                "{}T000000Z",
                (date + chrono::Days::new(1)).format("%Y%m%d")
            ))?;
            Ok::<_, anyhow::Error>((Some(since), Some(until)))
        }
        None => {
            let since = since.as_deref().map(IdLike::from_str).transpose()?;
            let until = until.as_deref().map(IdLike::from_str).transpose()?;
            Ok((since, until))
        }
    }?;

    let config = Config::load()?;
    let dir = PathBuf::from(config.data_dir());

    let mut lines = vec![];
    // TODO: Add start position to EntryWalker::new
    let walker = EntryWalker::new(dir.join("flow"), order);
    for entry in walker {
        let entry = entry?;
        if let Some(since) = since {
            if let Some(file_name) = entry.as_path().file_name() {
                if file_name.to_str().context("file_name is not UTF-8")?
                    < since.to_string().as_str()
                {
                    continue;
                }
            }
        }
        if let Some(until) = until {
            if let Some(file_name) = entry.as_path().file_name() {
                if file_name.to_str().context("file_name is not UTF-8")?
                    >= until.to_string().as_str()
                {
                    continue;
                }
            }
        }
        lines.push(path_buf_to_line(entry.as_path())?);
        if lines.len() >= limit {
            break;
        }
    }

    for line in lines {
        println!("{}", line);
    }
    Ok(())
}

/// A string that looks like an id
///
/// e.g. 20231001T123456Z
/// e.g. 20231001T123456+0900
/// e.g. 20231001T123456-0900
#[derive(Clone, Copy)]
pub struct IdLike(chrono::DateTime<chrono::Utc>);

impl std::fmt::Display for IdLike {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // +0000 -> Z
        write!(f, "{}", self.0.naive_utc().format("%Y%m%dT%H%M%SZ"))
    }
}

impl std::str::FromStr for IdLike {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // YYYYMMDDTHHMMSS+0900
        chrono::DateTime::parse_from_str(s, "%Y%m%dT%H%M%S%z")
            .or_else(|_| {
                // YYYYMMDDTHHMMSSZ
                let mut iter = s.chars();
                match iter.next_back() {
                    Some('Z') => Ok(chrono::DateTime::parse_from_str(
                        format!("{}+0000", iter.as_str()).as_str(),
                        "%Y%m%dT%H%M%S%z",
                    )?),
                    _ => Err(anyhow::anyhow!("invalid date format: {}", s)),
                }
            })
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .map(IdLike)
            .map_err(|_| anyhow::anyhow!("invalid date format: {}", s))
    }
}

struct EntryWalker {
    iter: walkdir::IntoIter,
}

impl EntryWalker {
    fn new(root_dir: PathBuf, order: Order) -> Self {
        let walk_dir = walkdir::WalkDir::new(root_dir).max_depth(4);
        let walk_dir = match order {
            Order::Asc => walk_dir.sort_by_key(|it| it.path().to_path_buf()),
            Order::Desc => walk_dir.sort_by_key(|it| std::cmp::Reverse(it.path().to_path_buf())),
        };
        let iter = walk_dir.into_iter();
        Self { iter }
    }
}

impl Iterator for EntryWalker {
    type Item = std::io::Result<PathBuf>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(entry) = self.iter.next() {
            match entry {
                Err(err) => {
                    return Some(Err(err.into_io_error().unwrap_or_else(|| {
                        std::io::Error::new(std::io::ErrorKind::Other, "loop")
                    })))
                }
                Ok(entry) => {
                    if entry.depth() != 4 {
                        continue;
                    }
                    match entry.path().extension() {
                        Some(extension) if extension == "md" => {
                            return Some(Ok(entry.into_path()));
                        }
                        Some(_) | None => continue,
                    }
                }
            }
        }
        None
    }
}

fn path_buf_to_line(path: &Path) -> anyhow::Result<String> {
    let id = path
        .file_stem()
        .context("no file_stem")?
        .to_str()
        .context("not UTF-8")?
        .to_string();

    let mut file = File::open(&path)?;
    let mut buf = [0; 1024];
    let len = Read::read(&mut file, &mut buf)?;
    let s = String::from_utf8_lossy(&buf[0..len]);
    let s = s.trim_end_matches(char::REPLACEMENT_CHARACTER);
    let s = s.replace('\n', " ");
    let s = s.chars().take(30).collect::<String>();
    // YYYYMMDDTHHMMSSZ
    // 12345678901234567...
    // 78 - 17 = 61
    // 61 / 2 ~= 30
    Ok(format!("{} {}", id, s))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use super::*;

    #[test]
    fn test_entry_walker() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let root_dir = temp_dir.path().join("root");
        let d1 = root_dir.join("2024").join("12").join("31");
        std::fs::create_dir_all(&d1)?;
        std::fs::write(d1.join("20241231T000001Z.md"), "Entry1")?;
        let d2 = root_dir.join("2025").join("01").join("01");
        std::fs::create_dir_all(&d2)?;
        std::fs::write(d2.join("20250101T000002Z.md"), "Entry2")?;
        std::fs::write(d2.join("20250101T000003Z.md"), "Entry3")?;
        let d3 = root_dir.join("2025").join("01").join("02");
        std::fs::create_dir_all(&d3)?;
        std::fs::write(d3.join("20250102T000004Z.md"), "Entry4")?;
        std::fs::write(d3.join("20250102T000005Z.md"), "Entry5")?;
        let d4 = root_dir.join("2025").join("02").join("01");
        std::fs::create_dir_all(&d4)?;
        std::fs::write(d4.join("20250201T000006Z.md"), "Entry6")?;

        let paths = [
            root_dir
                .join("2025")
                .join("02")
                .join("01")
                .join("20250201T000006Z.md"),
            root_dir
                .join("2025")
                .join("01")
                .join("02")
                .join("20250102T000005Z.md"),
            root_dir
                .join("2025")
                .join("01")
                .join("02")
                .join("20250102T000004Z.md"),
            root_dir
                .join("2025")
                .join("01")
                .join("01")
                .join("20250101T000003Z.md"),
            root_dir
                .join("2025")
                .join("01")
                .join("01")
                .join("20250101T000002Z.md"),
            root_dir
                .join("2024")
                .join("12")
                .join("31")
                .join("20241231T000001Z.md"),
        ];
        let mut walker = EntryWalker::new(root_dir.clone(), Order::Desc);
        for p in paths.iter().cloned() {
            assert_eq!(walker.next().transpose()?, Some(p));
        }
        assert_eq!(walker.next().transpose()?, None);
        let mut walker = EntryWalker::new(root_dir.clone(), Order::Asc);
        for p in paths.iter().rev().cloned() {
            assert_eq!(walker.next().transpose()?, Some(p));
        }
        assert_eq!(walker.next().transpose()?, None);
        Ok(())
    }

    #[test]
    fn test_id_like() -> anyhow::Result<()> {
        assert_eq!(
            IdLike::from_str("20231001T123456+0900")?.to_string(),
            "20231001T033456Z"
        );
        assert_eq!(
            IdLike::from_str("20231001T123456Z")?.to_string(),
            "20231001T123456Z"
        );
        Ok(())
    }
}
