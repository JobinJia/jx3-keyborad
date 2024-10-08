// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use tauri::command;
use serde::Serialize;
use std::fs;
use std::path::Path;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use serde::Deserialize;

#[derive(Serialize)]
struct FileEntry {
    id: u64,                      // 唯一标识符
    name: String,
    is_dir: bool,
    #[serde(skip_serializing_if = "Option::is_none")]  // 如果为 None 则跳过序列化
    children: Option<Vec<FileEntry>>, // 递归结构
}

impl FileEntry {
    fn new(name: String, is_dir: bool, path: &Path, children: Option<Vec<FileEntry>>) -> Self {
        let id = generate_id(&name, path);
        FileEntry {
            id,
            name,
            is_dir,
            children,
        }
    }
}

// 生成唯一的 ID
fn generate_id(name: &str, path: &Path) -> u64 {
    let mut hasher = DefaultHasher::new();
    (name, path).hash(&mut hasher);
    hasher.finish()
}

#[command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn read_directory(path: &Path, depth: usize) -> Result<Vec<FileEntry>, String> {
    let mut entries = vec![];

    for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let metadata = entry.metadata().map_err(|e| e.to_string())?;

        if metadata.is_dir() {
            let dir_name = entry.file_name().into_string().unwrap_or_default();

            if dir_name == "userpreferences" {
                continue;
            }

            let subdir = read_directory(&entry.path(), depth + 1)?;

            if (depth == 3 || depth == 2 || depth == 1) && subdir.is_empty() {
                continue;
            }

            let children = if subdir.is_empty() {
                None
            } else {
                Some(subdir)
            };

            let dir_entry = FileEntry::new(dir_name, true, &entry.path(), children);
            entries.push(dir_entry);
        }
    }

    Ok(entries)
}

#[command]
fn list_directory_contents(path: &str) -> Result<Vec<FileEntry>, String> {
    let path = std::path::Path::new(path);
    if !path.is_dir() {
        return Err("提供的路径不是一个目录".to_string());
    }

    read_directory(path, 1) // 从深度 1 开始
}


#[derive(Deserialize)]
struct CopyParams {
    source_path: String,
    target_path: String,
}

#[command]
fn cp_source_to_target(params: CopyParams) -> bool {
    let source_path = Path::new(&params.source_path);
    let target_path = Path::new(&params.target_path);

    // 处理过程中的错误将返回 false
    if target_path.exists() {
        if let Err(_) = fs::remove_dir_all(&target_path) {
            return false;
        }
        if let Err(_) = fs::create_dir_all(&target_path) {
            return false;
        }
    } else {
        if let Err(_) = fs::create_dir_all(&target_path) {
            return false;
        }
    }

    // 尝试复制目录
    if copy_dir_all(&source_path, &target_path).is_err() {
        return false;
    }

    // 如果一切顺利，返回 true
    true
}

// 递归复制目录
fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), String> {
    if src.is_dir() {
        fs::create_dir_all(dst).map_err(|e| e.to_string())?;
        for entry in fs::read_dir(src).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let file_type = entry.file_type().map_err(|e| e.to_string())?;
            let new_dst = dst.join(entry.file_name());
            if file_type.is_dir() {
                copy_dir_all(&entry.path(), &new_dst)?;
            } else {
                fs::copy(entry.path(), new_dst).map_err(|e| e.to_string())?;
            }
        }
    } else {
        return Err(format!("Source path {} is not a directory", src.display()));
    }
    Ok(())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![greet, list_directory_contents, cp_source_to_target])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
