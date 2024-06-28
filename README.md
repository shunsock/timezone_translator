# Time Zone Translator (TZT)
## Overview
This project is a simple command-line utility that converts a given time from one timezone to another.

## Features
- Convert a given time from one timezone to another.
- Supports multiple timezones.
  - if you want to see the list of supported timezones, read following url.
  - https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html

## Usage
The timezone_converter binary takes two arguments: the time to convert and the target timezone.

```bash
tzt --help
Converts time between time zones

Usage: tzt --time <TIME> --from <FROM_TIMEZONE> --to <TO_TIMEZONE>

Options:
  -T, --time <TIME>           Time in the format YYYY-MM-DD HH:MM:SS (you can omit HH:MM:SS) or YYYY-MM-DDTHH:MM:SS
  -f, --from <FROM_TIMEZONE>  The original timezone (e.g. America/New_York) @see https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html
  -t, --to <TO_TIMEZONE>      The target timezone (e.g. Asia/Tokyo) @see https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html#
  -h, --help                  Print help
  -V, --version               Print version
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

### Running the Command
After installing the binary, you can run it from the command line:

```bash
$ tzt -T "2024-01-01 12:00:00" -f "America/New_York" -t "UTC"
2024-01-01 17:00:00 UTC
```

### Uninstalling
To remove the installed binary, use the `uninstall` command:

```bash
make uninstall
```