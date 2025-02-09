use std::os::unix::fs::PermissionsExt;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc};
use tabwriter::TabWriter;
use std::io::Write;
use std::collections::HashMap;
use std::path::Path;
use std::{env, fs};



fn main() {
    let args: Vec<String> = env::args().collect();
    let dir_path = if args.len() > 1 {
        &args[1]
    } else {
        "."
    };

    let full_path = Path::new(dir_path).canonicalize();
    match full_path {
        Ok(path) => {
            if path.is_dir() {
                let dir_contents = extract_files_from_path(dir_path);
                dir_contents.print();
            } else {
                eprintln!("❌ Error: '{}' is not a directory.", dir_path);
            }
        }
        Err(_) => {
            eprintln!("❌ Error: Directory '{}' does not exist.", dir_path);
        }
    }
}

fn extract_files_from_path(path: &str) -> DirContents {
    let mut files: Vec<FileInfo> = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let metadata = entry.metadata().unwrap();
                let file_type = metadata.file_type();
                let file_name = entry.file_name().into_string().unwrap();
                
                if file_name == "." || file_name == ".." {
                    continue;
                }

                let file_size = human_readable_size(metadata.len());

                if let Ok(modified) = metadata.modified() {
                    let duration = modified.duration_since(UNIX_EPOCH).unwrap();
                    let datetime: DateTime<Utc> = DateTime::<Utc>::from(SystemTime::UNIX_EPOCH + duration);
                    let mod_time = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

                    if file_type.is_dir() {
                        files.push(FileInfo {
                            name: file_name,
                            readable_size: file_size,
                            modified_at: mod_time,
                            kind: FileType::DIRECTORY
                        })
                    } else if file_type.is_file() {
                        if metadata.permissions().mode() & 0o111 != 0 {
                            files.push(FileInfo {
                                name: file_name,
                                readable_size: file_size,
                                modified_at: mod_time,
                                kind: FileType::EXECUTABLE
                            })
                        } else {
                            files.push(FileInfo { 
                                name: file_name, 
                                readable_size: file_size, 
                                modified_at: mod_time,
                                kind: FileType::FILE
                            });
                        }
                    }
                }
            }
        }
    }
    files.sort_by(|a, b| a.kind.cmp(&b.kind).then(a.name.cmp(&b.name)));
    DirContents { files }
}

struct LongestFileInfoFields {
    max_name_len: usize,
    max_size_len: usize,
    max_date_len: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum FileType {
    FILE,
    DIRECTORY,
    EXECUTABLE
}

struct FileInfo {
    name: String,
    readable_size: String,
    modified_at: String,
    kind: FileType
}

struct DirContents {
    files: Vec<FileInfo>
}

impl DirContents {
    fn print(&self) {
        let mut tw = TabWriter::new(std::io::stdout()).padding(4);
        let max_lengths = self.get_longest_field_entries();
        let modified_at_delim: String = std::iter::repeat('-').take(max_lengths.max_date_len).collect();
        let size_of_delim: String = std::iter::repeat('-').take(max_lengths.max_size_len).collect();
        let name_of_delim: String = std::iter::repeat('-').take(max_lengths.max_name_len).collect();
    
        writeln!(tw, "Modified\tSize\tName").unwrap();
        writeln!(tw, "---{}\t{}\t{}", modified_at_delim, size_of_delim, name_of_delim).unwrap();

        for entry in &self.files {
            match entry.kind {
                FileType::DIRECTORY => writeln!(tw, "📁 {}\t{}\t{}/", entry.modified_at, entry.readable_size, entry.name).unwrap(),
                FileType::FILE => writeln!(tw, "{} {}\t{}\t{}", get_file_emoji(&entry.name[..]), entry.modified_at, entry.readable_size, entry.name).unwrap(),
                FileType::EXECUTABLE => writeln!(tw, "⚡ {}\t{}\t{}", entry.modified_at, entry.readable_size, entry.name).unwrap()
            }
        }
        
        tw.flush().unwrap();
    }
    
    fn get_longest_field_entries(&self) -> LongestFileInfoFields {
        let mut max_name_len: usize = 0;
        let mut max_size_len: usize = 0;
        let mut max_date_len: usize = 0;

        let mut update_max_lengths= |files: &Vec<FileInfo>|
            for file in files {
                if file.name.len() > max_name_len {
                    max_name_len = file.name.len();
                }
                if file.readable_size.len() > max_size_len {
                    max_size_len = file.readable_size.len();
                }
                if file.modified_at.len() > max_date_len {
                    max_date_len = file.modified_at.len();
                }
            };
        update_max_lengths(&self.files);
        LongestFileInfoFields {
            max_name_len,
            max_date_len,
            max_size_len
        }
    }
}

fn get_file_emoji(file_name: &str) -> &'static str {
    let mut emoji_map = HashMap::new();

    // Define file extensions and their corresponding emojis
    emoji_map.insert("txt", "📝");
    emoji_map.insert("md", "⬇️");
    emoji_map.insert("rs", "🦀");
    emoji_map.insert("rb", "💎");
    emoji_map.insert("go", "🐹");
    emoji_map.insert("py", "🐍");
    emoji_map.insert("java", "☕");
    emoji_map.insert("zig", "⚡");
    emoji_map.insert("c", "💾");
    emoji_map.insert("cpp", "💾");
    emoji_map.insert("js", "📜");
    emoji_map.insert("html", "🌐");
    emoji_map.insert("css", "🎨");
    emoji_map.insert("json", "📑");
    emoji_map.insert("csv", "📊");
    emoji_map.insert("mp3", "🎵");
    emoji_map.insert("wav", "🎵");
    emoji_map.insert("mp4", "🎬");
    emoji_map.insert("png", "🖼️");
    emoji_map.insert("jpg", "📷");
    emoji_map.insert("jpeg", "📷");
    emoji_map.insert("gif", "🎞️");
    emoji_map.insert("zip", "📦");
    emoji_map.insert("jar", "📦");
    emoji_map.insert("tar", "📦");
    emoji_map.insert("pdf", "📕");
    
    let extension = file_name
        .rsplit_once('.')
        .map(|(_, ext)| ext.to_lowercase())
        .unwrap_or(String::new());    
   
    emoji_map.get(extension.as_str()).unwrap_or(&"📄")
}

fn human_readable_size(size: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < units.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    format!("{:.2} {}", size, units[unit])
}