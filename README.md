# SmartShreds 
### (in progress)


SmartShreds is a Rust-based desktop application that enhances file management by detecting duplicate files through content and semantic similarity analysis. It uses hashing algorithms and NLP libraries to suggest merging, repurposing, and highlighting differences between similar files. Aimed at optimizing storage and improving content organization with AI-powered tools, SmartShreds is a sophisticated solution for managing digital clutter.

## Future Key Features

- **Content-based Duplicate Detection**: Uses hashing algorithms to detect duplicates.
- **Semantic Similarity Analysis**: Utilizes Natural Language Processing (NLP) libraries for content analysis.
- **Actionable Suggestions**:
  - Merge files with highly similar content.
  - Repurpose content for different formats based on semantic analysis.
  - Highlight key differences between similar files.

## Technologies
- GTK4
- Rust
- Plotter rs
- Linfa


## State of Project
<img width="1117" alt="Screenshot 2024-07-17 at 1 02 09 AM" src="https://github.com/user-attachments/assets/f763e50b-6bac-439d-a372-d40f9c7afdd3">



## Installation

```bash
# Clone the repository
git clone https://github.com/JKomieter/SmartShreds.git

# Navigate to the project directory
cd smartshreds

# Build the project
cargo build --release
```

## Usage

```bash
# Run the application
cargo run --release

```

## Contributing

Contributions are welcome! Please read our [Contributing Guidelines](CONTRIBUTING.md) for more details.

## License

This project is licensed under the Apache License. See the [LICENSE](LICENSE) file for more details.

## Additional Features (Future Enhancements)

- **Duplicate File Finder**: Continue with the current functionality of finding duplicate files, enhanced by comparing file contents to ensure they are truly duplicates.
- **File Type Analyzer**: Add functionality to analyze and categorize files based on their type (e.g., images, videos, documents).
- **Storage Analyzer**: Analyze storage usage, calculating the percentage used by each file type.
- **File History Tracker**: Track changes to files over time, including creation, modification, and deletion.
- **File Recovery**: Implement a feature to recover deleted files, if possible.
- **File Compression**: Add a feature to compress rarely used files to save storage space.
- **File Encryption**: Add a feature to encrypt sensitive files for security.
- **Testing and Documentation**: Write comprehensive tests for the application and document all features thoroughly.
