# FitLog

FitLog is a simple CLI tool to calculate and log your Body Mass Index (BMI). The app saves daily records of your BMI in an SQLite database, enabling you to track your health metrics over time.

## Features

- Calculates BMI based on user input for height and weight
- Classifies BMI into standard health categories
- Stores daily BMI records in an SQLite database, allowing you to track changes over time
- Color-coded console output for enhanced readability

### Installation on Ubuntu/Debian

To install the required dependencies, run:

```bash
sudo apt update
sudo apt install sqlite3 sqlite3-dev build-essential
```

1. **Clone the repository**:

```bash
git clone https://github.com/slash071/fitlog.git
cd fitlog
```

2. **Build the project and run**:

```bash
cargo build --release
cargo run
```

## Contributing

Feel free to open issues or submit pull requests with improvements. Contributions are welcome!

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE](./LICENSE) file for more details
