# Express MVC Project Automation Tool

This Rust program automates the creation and management of an Express.js backend application with an MVC (Model-View-Controller) architecture. It provides a convenient way to initialize and reset project files based on given configurations.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Configuration File](#configuration-file)
- [Features](#features)
- [Contributing](#contributing)
- [License](#license)

## Installation

1. **Clone the repository:**
   ```sh
   git clone https://github.com/yourusername/express-mvc-automation.git
   cd express-mvc-automation
   ```

2. **Build the project:**
   Make sure you have Rust installed. If not, install it from [rust-lang.org](https://www.rust-lang.org/).

   ```sh
   cargo build --release
   ```

3. **Run the executable:**
   The compiled executable will be in the `target/release` directory.

   ```sh
   ./target/release/express_mvc_automation --path /path/to/config.json --workdir /path/to/workdir
   ```

## Usage

1. **Prepare a configuration file:**
   Create a JSON configuration file that describes the modules you want to include in your project.

2. **Run the program with the necessary arguments:**
   - `--path`: Path to the configuration JSON file.
   - `--workdir`: Working directory where the project files will be created.

   ```sh
   ./express_mvc_automation --path /path/to/config.json --workdir /path/to/workdir
   ```

3. **Reset the project (optional):**
   If you want to start from scratch and remove existing `Controllers` and `Routes` folder content, the program will prompt you for confirmation.

4. **Check the generated files:**
   The program will generate the necessary files and directories in the specified working directory.

## Configuration File

The configuration file should be a JSON file with the structure defining your project modules. Example:

```json
[
    {
        "module_name": "user",
        "crud": true
    },
    {
        "module_name": "product",
        "crud": true
    }
]
```

- `module_name`: The name of the module.
- `crud`: Boolean indicating if CRUD operations should be included.

## Features

- **Automated File Structure Creation:** Creates necessary directories and files based on the configuration.
- **Reset Option:** Option to remove existing files and start fresh.
- **Customizable Content:** Generates content for controllers, routes, models, middleware, and other project files.
- **Configuration Driven:** Easily configurable through a JSON file.

## Contributing

Contributions are welcome! Please follow these steps to contribute:

1. **Fork the repository**
2. **Create a new branch** (`git checkout -b feature-branch`)
3. **Commit your changes** (`git commit -m 'Add some feature'`)
4. **Push to the branch** (`git push origin feature-branch`)
5. **Open a Pull Request**

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
