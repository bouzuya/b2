use std::collections::BTreeSet;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use anyhow::Context as _;
use chrono::Utc;

use crate::Config;

pub struct Args {
    pub date: Option<String>,
}

pub fn execute(Args { date }: Args) -> anyhow::Result<()> {
    let config = Config::load()?;
    let dir = PathBuf::from(config.data_dir());
    let date_string = date.unwrap_or_else(today);
    let (yyyy, mm, dd) = {
        let s = date_string.split('-').collect::<Vec<&str>>();
        anyhow::ensure!(s.len() == 3, "date is not YYYY-MM-DD");
        (s[0], s[1], s[2])
    };
    let today_dir = dir.join("flow").join(yyyy).join(mm).join(dd);

    let mut lines = BTreeSet::new();
    if today_dir.exists() {
        for dir_entry in fs::read_dir(today_dir)? {
            let dir_entry = dir_entry?;
            let path = dir_entry.path();
            if let Some(extension) = path.extension() {
                if extension == "md" {
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
                    lines.insert(format!("{} {}", id, s));
                }
            }
        }
    }

    for line in lines {
        println!("{}", line);
    }
    Ok(())
}

fn today() -> String {
    Utc::now().format("%Y-%m-%d").to_string()
}
