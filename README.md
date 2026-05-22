# File Organize
A command-line tool for scanning and organizing files into categorized folders

## Features
- Scan any user-provided directory
- Detects files type
- Group Files based on types
- Automatically creates folders based on what is needed
- Moves files into categorized folders
- Handles invalid paths and missing folders/files
### Support:
- Images (png, jpg, jpeg, gif)
- Videos (mp4, mov)
- Music (mp3, wav)
- Documents (pdf, docx)
- Code (rs, js, java)
- Archives (zip, rar)
- Unknown files 

## How to Run
1. Clone the repository
2. cargo run
3. Enter the folder to scan
4. Enter the folder location to move to.


## What I Learned
- Rust file API (read_dir, rename, create_dir_all)
- Path and PathBuf
- Using HashMap to store information

## Future Improvement
- More folder categories
- Dry-run (preview changes before moving)
- Undo system
- Summary report
