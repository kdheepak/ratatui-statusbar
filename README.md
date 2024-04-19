# ratatui-statusbar

ratatui-statusbar is a status bar library for [Ratatui](https://github.com/ratatui-org/ratatui) crate for Rust.

## Getting Started

To get started, first add it to your Cargo.toml dependencies:

```
cargo add ratatui-statusbar
```

Creating a new status bar is simple. Here's a quick example:

```rust
use status_bar::StatusBar;

let status_bar = StatusBar::new(3)
    .section(0, "Left content")?;
    .section(1, "Center content")?;
    .section(2, "Right content")?;
```

## Contributing

Please feel free to fork the repository, make your changes, and submit a pull request.

## License

See [LICENSE](LICENSE) for details.
