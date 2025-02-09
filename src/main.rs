use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc};
use tabwriter::TabWriter;
use std::io::Write;
use std::collections::HashMap;
use std::path::Path;
use std::env;


fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Determine the directory to use
    let dir_path = if args.len() > 1 {
        &args[1]  // Use provided path
    } else {
        "."  // Default to current directory
    };

    // Resolve the absolute path
    let full_path = Path::new(dir_path).canonicalize();

    match full_path {
        Ok(path) => {
            if path.is_dir() {
                let (
                    files, 
                    directories, 
                    executables
                ) = extract_files_from_path(dir_path);
                print(&directories, &files, &executables);
            } else {
                eprintln!("❌ Error: '{}' is not a directory.", dir_path);
            }
        }
        Err(_) => {
            eprintln!("❌ Error: Directory '{}' does not exist.", dir_path);
        }
    }
}

fn extract_files_from_path(path: &str) -> (Vec<(String, String, String)>, Vec<(String, String, String)>, Vec<(String, String, String)>){
    let mut files = Vec::new();
    let mut directories = Vec::new();
    let mut executables = Vec::new();

    // Read directory entries
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let metadata = entry.metadata().unwrap();
                let file_type = metadata.file_type();
                let file_name = entry.file_name().into_string().unwrap();
                
                // Skip "." and ".."
                if file_name == "." || file_name == ".." {
                    continue;
                }

                // Get file size in human-readable format
                let file_size = human_readable_size(metadata.len());

                // Get last modification time
                if let Ok(modified) = metadata.modified() {
                    let duration = modified.duration_since(UNIX_EPOCH).unwrap();
                    let datetime: DateTime<Utc> = DateTime::<Utc>::from(SystemTime::UNIX_EPOCH + duration);
                    let mod_time = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

                    // Categorize items
                    if file_type.is_dir() {
                        directories.push((file_name, file_size, mod_time));
                    } else if file_type.is_file() {
                        if metadata.permissions().mode() & 0o111 != 0 {
                            executables.push((file_name, file_size, mod_time));
                        } else {
                            files.push((file_name, file_size, mod_time));
                        }
                    }
                }
            }
        }
    }
    files.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
    directories.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
    executables.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
    return (files, directories, executables)
}

fn print(directories: &Vec<(String, String, String)>, files: &Vec<(String, String, String)>, executables: &Vec<(String, String, String)>) {
    let mut tw = TabWriter::new(std::io::stdout()).padding(4);
    let (date_len, size_len, name_len) = get_longest_entry(directories, files, executables);
    let modified_delim: String = std::iter::repeat('-').take(date_len).collect();
    let size_delim: String = std::iter::repeat('-').take(size_len).collect();
    let name_delim: String = std::iter::repeat('-').take(name_len).collect();
    
    writeln!(tw, "Modified\tSize\tName").unwrap();
    writeln!(tw, "---{}\t{}\t{}", modified_delim, size_delim, name_delim).unwrap();
    
    for (name, size, date) in directories {
        writeln!(tw, "📁 {}\t{}\t{}/", date, size, name).unwrap();
    }

    for (name, size, date) in files {
        writeln!(tw, "{} {}\t{}\t{}", get_file_emoji(name), date, size, name).unwrap();
    }

    for (name, size, date) in executables {
        writeln!(tw, "⚡ {}\t{}\t{}", date, size, name).unwrap();
    }
    tw.flush().unwrap();
}

fn get_longest_entry(directories: &Vec<(String, String, String)>, files: &Vec<(String, String, String)>, executables: &Vec<(String, String, String)>) -> (usize, usize, usize) {
    let mut max_name_len: usize = 0;
    let mut max_size_len: usize = 0;
    let mut max_date_len: usize = 0;

    for (name, size, date) in directories {
        if name.len() > max_name_len {
            max_name_len = name.len();
        }
        if size.len() > max_size_len {
            max_size_len = size.len();
        }
        if date.len() > max_date_len {
            max_date_len = date.len();
        }
    }

    for (name, size, date) in files {
        if name.len() > max_name_len {
            max_name_len = name.len();
        }
        if size.len() > max_size_len {
            max_size_len = size.len();
        }
        if date.len() > max_date_len {
            max_date_len = date.len();
        }
    }

    for (name, size, date) in executables {
        if name.len() > max_name_len {
            max_name_len = name.len();
        }
        if size.len() > max_size_len {
            max_size_len = size.len();
        }
        if date.len() > max_date_len {
            max_date_len = date.len();
        }
    }
    return (max_date_len, max_size_len, max_name_len)
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