# rust-rwcat

My rust written cat styled program that has read and write append commands


## Usage

```sh
rwcat read test.txt           # Just read the whole file
rwcat read test.txt test      # Query "test" from test.txt, return lines that has "test" in them
rwcat write test.txt "This string will be appended to the text file"
```

The program generates the missing file if you write to a file that does not exist.

## Compiling

Compile using cargo
```sh
git clone https://gitlab.com/Katacc/rust-rwcat.git
cd rwcat
cargo build --release
```
The executable can be found in `target/release/` folder, rename it to whatever you wish and add it to your PATH to use it anywhere.
