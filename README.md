# w (where)

This simple Rust program displays the current Git branch and working directory in the terminal. It uses ANSI escape codes for color formatting.

## Usage

1. Clone the repository:

   ```bash
   git clone https://github.com/kad1rr/w.git
   ```
   
2. Navigate to the project directory:
    ````bash
    cd w
    ````
   
3. Build and run the program:
    ```bash
    cargo build # --release
   cargo run
    ```
   
## Requirements
- Rust
- Lazy Static (lazy_static)

## Features
- Displays the current Git branch.
- Highlights the working directory path with colors.
- Customizable symbols and colors.

## Configuration

```rust
// main.rs

// ...

let symbol = "→".to_string();

// Customize these symbols
```

```rust
// colors.rs

pub struct Colors {}

impl Colors {
    pub fn yellow_bg(message: &str) -> String {
        format!("\x1b[43;30m{}\x1b[0m", message)
    }

    pub fn yellow(message: &str) -> String {
        format!("\x1b[33m{}\x1b[0m", message)
    }

    pub fn blue_bg(message: &str) -> String {
        format!("\x1b[44m{}\x1b[0m", message)
    }

    pub fn blue(message: &str) -> String {
        format!("\x1b[34m{}\x1b[0m", message)
    }
}

// Customize these colors
```

