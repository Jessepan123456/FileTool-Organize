# File Organize
A command-line tool for scanning and organizing files into categorized folders

## Features
- Scan any user-provided directory
- Detects files type
- Group Files based on types
- Automatically creates folders based on what is needed
- Moves files into categorized folders
- Handles invalid paths and missing folders/files
- Added a custom folder user input
- Rollback system for previous modification
- Folder system based on extension or filename

### Support:
- Images (png, jpg, jpeg, gif)
- Videos (mp4, mov)
- Music (mp3, wav)
- Documents (pdf, docx)
- Code (rs, js, java)
- Archives (zip, rar)
- Unknown files
- Can also support any custom input

## How to Run
1. Clone the repository
2. cargo run
3. Enter 1-4 option
- 1. Default Folder
- 2. Custom Folder
- 3. Rollback
- 4. Search by folder

## What I Learned
- Rust file API (read_dir, rename, create_dir_all)
- Path and PathBuf
- Using HashMap to store information
- Persistent data using JSON
- Lambda for custom inputs

## Future Improvement
- Recursive Search
- Multiple Rollback history
- Duplicate detection
- Save Custom Category in JSON
- File Stats
