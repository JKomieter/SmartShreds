# SmartShreds 
### (in progress)
SmartShreds is a Rust-based desktop application designed to help educators and students efficiently manage and organize their files. Leveraging AI capabilities, SmartShreds identifies and highlights relevant file paths, providing suggestions on file organization and storage optimization.

## Future Key Features

- **AI-Driven File Path Identification**: Utilize AI to determine the relevance of file paths based on metadata.
- **Cross-Platform Support**: Compatible with Windows, macOS, and Linux.
- **User-Friendly Interface**: An intuitive interface with detailed insights and recommendations.
- **Efficient Storage Optimization**: Optimize storage by identifying and suggesting files that can be deleted or archived.

## Technologies
- GTK4
- Rust
- OpenAI
- NodeJS


## State of Project
![Screenshot 2024-07-17 at 10 38 04 PM](https://github.com/user-attachments/assets/90a15f34-fdec-4c13-a662-d0c41bdf5d51)


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

## Application usage

### Home Screen:

- **Displays various file types and the space they occupy.**
Provides options to view recent files and suggested actions.
Analyze File Paths:

- **Select file paths to analyze.**
Get AI-driven suggestions on relevance and organization.
Optimize Storage:

- **Identify and remove duplicate or unnecessary files.**
Organize files efficiently based on AI recommendations.


### How It Works

- **File Path Analysis:**
The application sends a list of file paths and metadata to the backend service.
The backend service uses the OpenAI API to analyze the relevance of each file path.

- **AI Integration:**
The backend service processes the data and returns relevant file paths and recommendations.
The Rust client displays these recommendations to the user.

## Contributing

Contributions are welcome! Please read our [Contributing Guidelines](CONTRIBUTING.md) for more details.

## License

This project is licensed under the Apache License. See the [LICENSE](LICENSE) file for more details.

