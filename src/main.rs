use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use winreg::enums::*;
use winreg::RegKey;

fn get_unique_file_name(parent_dir: &Path, file_path: &Path) -> PathBuf {
    let extension = file_path.extension().map(|ext| {
        let mut extension = OsString::new();
        extension.push(".");
        extension.push(ext);
        extension
    });

    let filename_base = file_path
        .file_stem()
        .map_or_else(OsString::new, OsString::from);

    std::iter::once(parent_dir.join(file_path.file_name().unwrap_or_default()))
        .chain((1..).map(|count| {
            let mut filename = filename_base.clone();
            filename.push(format!("({count})"));
            if let Some(ref ext) = extension {
                filename.push(ext);
            }
            parent_dir.join(filename)
        }))
        .find(|new_path| !new_path.exists())
        .unwrap()
}

fn flatten_folder(dir: &Path) -> io::Result<()> {
    let parent_dir = dir
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Parent directory not found"))?;

    // 同名フォルダ対策のため、一時フォルダ名を作成
    let temp_dir = parent_dir.join("__temp_flatten_folder");
    if temp_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "Temporary folder already exists",
        ));
    }
    fs::create_dir(&temp_dir)?;

    let files: HashMap<PathBuf, PathBuf> = fs::read_dir(dir)?
        .map(|entry| {
            let path = entry?.path();
            let file_name = path
                .file_name()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid file name"))?;

            let temp_path = temp_dir.join(file_name);

            Ok::<_, io::Error>((path, temp_path))
        })
        .collect::<Result<HashMap<_, _>, _>>()?;

    for (current_path, temp_path) in &files {
        fs::rename(current_path, temp_path)?;
    }

    fs::remove_dir(dir)?;

    for temp_path in files.values() {
        let new_path = get_unique_file_name(parent_dir, temp_path);
        fs::rename(temp_path, new_path)?;
    }

    fs::remove_dir(temp_dir)?;

    Ok(())
}

fn add_context_menu() -> io::Result<()> {
    let (key, _) = RegKey::predef(HKEY_CURRENT_USER)
        .create_subkey("Software\\Classes\\Directory\\shell\\FlattenFolders")?;
    key.set_value("", &"Flatten Folders")?;
    let (command_key, _) = key.create_subkey("command")?;
    let command = format!(
        "\"{}\" \"%1\"",
        env::current_exe()?
            .to_str()
            .expect("Invalid Unicode character found.")
    );
    command_key.set_value("", &command)
}

fn remove_context_menu() -> io::Result<()> {
    RegKey::predef(HKEY_CURRENT_USER)
        .delete_subkey_all("Software\\Classes\\Directory\\shell\\FlattenFolders")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("--install") => match add_context_menu() {
            Ok(()) => println!("Context menu added successfully."),
            Err(e) => eprintln!("Error adding context menu: {e}"),
        },
        Some("--uninstall") => match remove_context_menu() {
            Ok(()) => println!("Context menu removed successfully."),
            Err(e) => eprintln!("Error removing context menu: {}", e),
        },
        Some(dir) => {
            if let Err(e) = flatten_folder(Path::new(dir)) {
                eprintln!("Error: {e}");
            }
        }
        None => {
            eprintln!("Usage:");
            eprintln!("  {} --install", args[0]);
            eprintln!("  {} --uninstall", args[0]);
            eprintln!("  {} <directory>", args[0]);
        }
    }
}
