use std::env;
use std::fs;
use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;

fn flatten_folder(dir: &Path) -> std::io::Result<()> {
    let parent_dir = dir.parent().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "Parent directory not found")
    })?;

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        let file_name = path.file_name().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid file name")
        })?;

        let new_path = parent_dir.join(file_name);
        fs::rename(&path, &new_path)?;
    }

    fs::remove_dir(dir)?;

    Ok(())
}

fn add_context_menu() -> std::io::Result<()> {
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

fn remove_context_menu() -> std::io::Result<()> {
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
