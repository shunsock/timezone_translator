# timezone_converter
## Overview
This project is a simple command-line utility that converts a given time from one timezone to another.

## Features
- Convert a given time from one timezone to another.
- Supports multiple timezones.

## Usage
The timezone_converter binary takes two arguments: the time to convert and the target timezone.

```bash
$ tzconv -T "2024-01-01 12:00:00" -f "America/New_York" -t "UTC"
2024-01-01 17:00:00 UTC
```

## Dependencies
This project requires the following dependencies:

- `Cargo`: The Rust package manager and build tool.
- `Make`: A build automation tool that simplifies the build process.

## Getting Started
This project uses a Makefile for managing build and installation tasks. Here are some quick steps to get started:

1. Clone the repository to your local machine.
2. Navigate to the project root directory where the Makefile is located.

### Building and Installing

To build and install the project, you can use the `install` target in the Makefile.

```bash
make install
```

This command performs the following tasks:

1. Compiles the project with `cargo build --release`.
2. Copy the compiled binary from `target/release/timezone_converter` to `/usr/local/bin/tzconv`.

### Running the Command
After installing the binary, you can run it from the command line:

```bash
$ tzconv -T "2024-01-01 12:00:00" -f "America/New_York" -t "UTC"
2024-01-01 17:00:00 UTC
```

### Uninstalling
To remove the installed binary, use the uninstall target:

```bash
make uninstall
```

This command removes the `tzconv` binary from `/usr/local/bin`.

Please replace the actual commands and paths with your specific project context.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.