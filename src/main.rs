use chrono::{DateTime, Utc};
use clap::Parser;
use owo_colors::OwoColorize;
use serde::Serialize;
use std::{
    fs,
    io::Error,
    path::{self, Path, PathBuf},
};
use strum::Display;
use tabled::{
    Table, Tabled,
    settings::{
        Color, Style,
        object::{Columns, Rows},
    },
};

#[derive(Debug, Display, Serialize)]
enum EntryType {
    File,
    Dir,
}

#[derive(Debug, Tabled, Serialize)]
struct FileEntry {
    #[tabled{rename = "Name"}]
    name: String,
    #[tabled{rename = "Type"}]
    e_type: EntryType,
    #[tabled{rename = "SizeBytes"}]
    len_bytes: u64,
    #[tabled{rename = "Modified"}]
    modified: String,
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = "amenls - List files in a directory.")]
struct Cli {
    path: Option<PathBuf>, // 目录路径
    #[arg(short, long)]
    json: bool, // 是否输出JSON格式
}

fn main() {
    let cli = Cli::parse();
    let path = cli.path.unwrap_or(PathBuf::from("."));
    if let Ok(does_exist) = fs::exists(&path) {
        if does_exist {
            if cli.json {
                if let Ok(files) = get_file(&path) {
                    println!(
                        "{}",
                        serde_json::to_string(&files).unwrap_or("cannot print json".to_string())
                    );
                } else {
                    println!("{}", "Error reading directory.".red());
                }
            } else {
                print_table(path);
            }
        } else {
            println!("{}", "Directory does not exist.".red());
        }
    } else {
        println!("{}", "Error reading directory.".red());
    }
}

fn get_file(path: &Path) -> Result<Vec<FileEntry>, Error> {
    let mut data = Vec::new();
    let read_dir = fs::read_dir(path)?;
    for entry in read_dir {
        let entry = entry?;
        map_data(&entry, &mut data);
    }

    Ok(data)
}

fn map_data(entry: &fs::DirEntry, data: &mut Vec<FileEntry>) {
    let meta = fs::metadata(&entry.path());
    match meta {
        Ok(meta) => {
            data.push(FileEntry {
                name: entry
                    .file_name()
                    .into_string()
                    .unwrap_or("unknown name".into()),
                e_type: if meta.is_dir() {
                    EntryType::Dir
                } else {
                    EntryType::File
                },
                len_bytes: meta.len(),
                modified: if let Ok(modi) = meta.modified() {
                    let date: DateTime<Utc> = modi.into();
                    format!("{}", date.format("%a %b %e %Y"))
                } else {
                    String::default()
                },
            });
        }
        Err(err) => {
            println!("Error reading file metadata: {:?}", err);
        }
    };
}

fn print_table(path: PathBuf) {
    let files = get_file(&path);
    if let Ok(files) = files {
        let mut table = Table::new(files);
        table.with(Style::rounded()); // 修改表格样式为圆角框
        table.modify(Columns::first(), Color::FG_BRIGHT_CYAN); // 修改第一列颜色为亮青
        table.modify(Columns::one(2), Color::FG_BRIGHT_MAGENTA); // 修改第三列颜色为亮品红色
        table.modify(Columns::one(3), Color::FG_BRIGHT_YELLOW); // 修改第四列颜色为亮黄色
        table.modify(Rows::first(), Color::FG_BRIGHT_GREEN); // 修改第一行颜色为亮绿色
        println!("{}", table);
    } else {
        println!("{}", "Error reading directory.".red());
    }
}
