# Look tool

This is a command-line tool written in Rust that searches for files matching a given pattern and displays their sizes in a human-readable format.

## Features

- Search for files using glob patterns.
- Display file sizes in a human-readable format (B, KB, MB, GB, TB).

## Usage

To use this tool, run the following command:

```sh
look <pattern>
```

### Example

Search for all `.jpg` files in the `media` folder and its subfolders:

  ```sh
  look '/media/**/*.jpg'
  ```

The tool will display the size and full path of each file found:

```
1.23 MB - /media/pictures/photo.jpg
456 KB  - /media/pictures/album/another_photo.jpg
```

## Installation

To install the tool, follow these steps:

1. Ensure you have [Rust](https://www.rust-lang.org/) installed.
2. Clone this repository:

   ```sh
   git clone <repository-url>
   ```

3. Navigate to the project directory:

   ```sh
   cd file-search-tool
   ```

4. Build the project:

   ```sh
   cargo build --release
   ```

5. Add binary file to the Path

6. Run the tool:

   ```sh
   look <pattern>
   ```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
