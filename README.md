# Session Availability Checker

## Description
This is a beginner Rust project aimed at scraping a website to check when a session is available after it is sold out. This project serves as an introduction to Rust programming language and web scraping techniques.

## Features
- Scrapes a specified website to check session availability.
- Notifies the user when a session becomes available after being sold out.

## Installation
1. Make sure you have Rust installed. If not, you can download it from [here](https://www.rust-lang.org/tools/install).
2. Clone this repository:
    ```
    git clone [https://github.com/pillows/nyurban-scraper](https://github.com/pillows/nyurban-scraper)
    ```
3. Navigate to the project directory:
    ```
    cd nyurban-scraper
    ```
4. Build the project:
    ```
    cargo build --release
    ```
5. Run the project:
    ```
    cargo run
    ```

## Usage
- Modify the `config.toml` file to specify the URL of the website to be scraped and any other parameters.
- Run the project using `cargo run`.
- The program will periodically check the specified website for session availability.
- When a session becomes available after being sold out, the program will notify the user.

## Contributing
Contributions are welcome! If you find any bugs or have suggestions for improvements, please open an issue or submit a pull request.

## License
TBD
