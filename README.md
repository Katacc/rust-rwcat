# rust-rwcat

![GitLab pipeline status](https://gitlab.com/Katacc/rust-rwcat/badges/main/pipeline.svg?ref=main)
![GitLab license](https://gitlab.com/Katacc/rust-rwcat/badges/main/license.svg)

`rust-rwcat` is a Rust-written `cat`-styled program that provides commands for reading and appending to files.

---

## Features

- **Read**:
  - Read the entire file.
  - Query specific lines containing a keyword.
- **Write**:
  - Append a string to a file.
  - Automatically creates the file if it doesn't exist.

---

## Installation

Clone the repository and compile the project using Cargo:

```sh
git clone https://gitlab.com/Katacc/rust-rwcat.git
cd rust-rwcat
cargo build --release
```

The executable will be located in the `target/release/` folder. Rename it if desired and add it to your `PATH` for global usage.

---

## Usage

### Read from a file
```sh
rwcat read <file_path> [query]
```
- **Example 1**: Read the entire file:
  ```sh
  rwcat read test.txt
  ```
- **Example 2**: Query lines containing "test":
  ```sh
  rwcat read test.txt test
  ```

### Write to a file
```sh
rwcat write <file_path> <content>
```
- **Example**: Append a string to a file:
  ```sh
  rwcat write test.txt "This string will be appended to the text file"
  ```

> **Note**: If the file does not exist, it will be created automatically.

---

## License

This project is licensed under the GNU Affero General Public License v3. See the [LICENSE](LICENSE) file for details.

---

## Contact

For questions or feedback, please open an issue on the [GitLab repository](https://gitlab.com/Katacc/rust-rwcat).
