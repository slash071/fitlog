# FitLog

FitLog is a simple CLI tool for calculating and logging your Body Mass Index (BMI).Designed specifically for adults (ages 20 and above), the app saves daily BMI records in an SQLite database, enabling you to track your health metrics over time.

## Features

- Calculates BMI based on user input for height and weight
- Classifies BMI into standard health categories
- Stores daily BMI records in an SQLite database, allowing you to track changes over time
- Color-coded console output for enhanced readability

### Installation on Ubuntu/Debian

To install the required dependencies, run:

```bash
sudo apt update
sudo apt install sqlite3 libsqlite3-dev build-essential
```

1. **Clone the repository**:

```bash
git clone https://github.com/slash071/fitlog.git
cd fitlog
```

2. **Build the project and run**:

```bash
cargo build --release
```

The binary will be located under `target/release/fitlog`. You can add it to your PATH to access it globally from anywhere, or run it directly with Cargo:

```bash
cargo run --release
```

**Note**: If you find the welcome message redundant, you can simply comment out the `display_welcome` function call in the `main.rs` file and recompile the app.

## Customization

- **Remove the welcome message**: If you find the welcome message redundant, simply comment out the `display_welcome` function call in the `main.rs` file and recompile the app.
- **Set height permanently**: If your height doesn’t change often and you want to set it permanently, you can modify the `user` struct in `main.rs`. Change the `height` field to a fixed value (e.g., `180.0` for 180 cm). The height must be in centimeters.

## Contributing

Contributions are welcome! Feel free to [open issues](https://github.com/slash071/fitlog/issues) or [submit pull requests](https://github.com/slash071/fitlog/pulls) with improvements.

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE](./LICENSE) file for more details
