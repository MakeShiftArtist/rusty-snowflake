# rusty-snowflake
[![Crates.io Version](https://img.shields.io/crates/v/rusty-snowflake)](https://crates.io/crates/rusty-snowflake)
[![Crates.io License](https://img.shields.io/crates/l/rusty-snowflake)](https://github.com/MakeShiftArtist/rusty-snowflake/blob/master/LICENSE)
[![Tests](https://github.com/MakeShiftArtist/rusty-snowflake/actions/workflows/test_code.yml/badge.svg)](https://github.com/MakeShiftArtist/rusty-snowflake/actions/workflows/test_code.yml)
[![docs.rs](https://img.shields.io/docsrs/rusty-snowflake)](https://docs.rs/rusty-snowflake)

This library is a Snowflake ID generator and parser written entirely in Rust.

## Features

-   Generate unique snowflake IDs based on timestamp, worker ID, and sequence number.
-   Parse snowflake IDs to retrieve timestamp, worker ID, and sequence number.

## Installation

Add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
rusty-snowflake = "0.1.0"
```

Or run the following in your project directory

```bash
cargo add rusty-snowflake
```

## Usage

```rust
use rusty_snowflake::Snowflake;

fn main() {
    // Create a new snowflake generator with custom worker ID of 123
    let mut snowflake = Snowflake::new(123);

    // Generate a new snowflake ID
    let id = generator.next();
    println!("Generated snowflake id: {}", id);

    // Parse the snowflake ID
    let parsed_snowflake = Snowflake::parse(id);
    println!("Parsed snowflake: {:?}", parsed_snowflake);
}
```

## Contributions

Contributions and feedback are welcome! If you encounter any issues or have suggestions for improvements, please open an issue or submit a pull request on the [GitHub repository](https://github.com/MakeShiftArtist/rusty-snowflake).

## License

This project is licensed under the [MIT License](LICENSE).
