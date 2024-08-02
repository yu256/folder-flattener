# Folder-Flattener

`Folder-Flattener` is a tool that moves the contents of a specified directory up one level and deletes the original directory. It also allows you to add this functionality to the Windows context menu for easier access.4

## Features

- **Flatten Directories**: Moves all files and subdirectories within the specified directory to its parent directory and deletes the original directory.
- **Context Menu Integration**: Adds a "Folder-Flattener" option to the Windows right-click menu, allowing you to easily perform this action on selected folders.

## Installation

### Adding to Context Menu

1. Ensure Rust is installed on your system.
2. Clone or download this project.
3. Navigate to the project directory and build the project with the following command:

    ```bash
    cargo build --release
    ```

4. Once the build is complete, add the context menu entry by running:

    ```bash
    target/release/folder-flattener --install
    ```

### Removing from Context Menu

To remove the context menu entry, run:

```bash
target/release/folder-flattener --uninstall
