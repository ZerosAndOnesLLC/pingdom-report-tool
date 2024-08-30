# Pingdom Uptime Calculator

## Overview
The Pingdom Uptime Calculator is a Rust-based command-line tool designed to calculate and report uptime statistics for Pingdom checks over a specified date range. This tool leverages asynchronous programming to efficiently process multiple checks concurrently, providing fast and accurate uptime calculations.

Created by Ron McCorkle

## Features
- Calculates uptime percentages and downtime minutes for all Pingdom checks
- Supports custom date ranges for calculations
- Utilizes asynchronous programming for improved performance
- Processes multiple checks concurrently (up to 10 at a time)
- Reads Pingdom API credentials from environment variables or a .env file
- Provides a user-friendly command-line interface with usage instructions

## Prerequisites
- Rust programming language (latest stable version)
- Pingdom account with API access

## Setup
1. Clone the repository:
   ```
   git clone https://github.com/yourusername/pingdom-uptime-calculator.git
   cd pingdom-uptime-calculator
   ```

2. Set up your Pingdom API credentials:
   You have two options:

   a. Create a `.env` file in the project root:
      ```
      PINGDOM_API_KEY=your_api_key_here
      PINGDOM_API_URL=https://api.pingdom.com/api/3.1
      ```

   b. Set environment variables directly in your shell:
      ```
      export PINGDOM_API_KEY=your_api_key_here
      export PINGDOM_API_URL=https://api.pingdom.com/api/3.1
      ```

3. Build the project:
   ```
   cargo build --release
   ```

## Usage
After compiling, you can run the tool directly without using `cargo run`. The compiled binary will be in the `target/release` directory.

1. If you're in the project root, you can run:
   ```
   ./target/release/pingdom-uptime-calculator --start-date MM/DD/YYYY --end-date MM/DD/YYYY
   ```

2. Alternatively, you can move the binary to a directory in your PATH and run it from anywhere:
   ```
   pingdom-uptime-calculator --start-date MM/DD/YYYY --end-date MM/DD/YYYY
   ```

Example:
```
pingdom-uptime-calculator --start-date 01/01/2024 --end-date 12/31/2024
```

This will calculate the uptime for all your Pingdom checks from January 1, 2024, to December 31, 2024.

If you prefer to run it with cargo during development, you can still use:
```
cargo run -- --start-date MM/DD/YYYY --end-date MM/DD/YYYY
```

## Output
The tool will display the uptime statistics for each check in the following format:
```
Check Name, Uptime Percentage%, Downtime Minutes
```

Example output:
```
My Website, 99.900%, 525 mins
API Server, 99.950%, 262 mins
Database Cluster, 99.999%, 52 mins
```

## Notes
- The tool uses a small delay (200ms) between API requests to avoid rate limiting. Adjust this in the code if necessary.
- Ensure your Pingdom API key has the necessary permissions to access check information and performance summaries.
- If you're using the `.env` file, make sure it's in the same directory as the binary when running the compiled version.

## Dependencies
- reqwest: HTTP client for making API requests
- serde and serde_json: For JSON serialization and deserialization
- chrono: For date and time handling
- tokio: Asynchronous runtime
- clap: For parsing command-line arguments
- dotenv: For loading environment variables from a .env file
- futures: For concurrent processing of API requests

## Contributing
Contributions to improve the Pingdom Uptime Calculator are welcome. Please feel free to submit issues or pull requests.

## License
[Specify the license here, e.g., MIT, Apache 2.0, etc.]

## Credits
Created by Ron McCorkle

For any questions or support, please contact [provide contact information if appropriate].
