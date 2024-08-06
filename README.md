<div align="center">
  <a href="https://github.com/shunsock/timezone_translator">
    <img alt="tzt" src="image/txt_icon.jpg">
  </a>
</div>

<h2 align="center">
  tzt - Timezone Translator
</h2>
<p align="center">
  simple command-line utility that converts a given time from one timezone to another.
</p>

<p align="center">
  <a href="./LICENSE">
    <img alt="LICENSE" src="https://img.shields.io/badge/license-MIT-blue.svg?maxAge=43200"></a>
  <a href="https://www.rust-lang.org/">
    <img alt="rust" src="https://img.shields.io/badge/logo-rust-blue?logo=rust"></a>
  <a href="https://github.com/shunsock/timezone_translator/actions/workflows/rust.yml">
    <img alt="workflow" src="https://github.com/shunsock/timezone_translator/actions/workflows/rust.yml/badge.svg"></a>
</p>

## Features
- Convert a given time from one timezone to another.
- Supports multiple timezones.
  - if you want to see the list of supported timezones, read following url.
  - https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html

## Usage
You can use the following command to see the help message.

```bash
$tzt --help
translate time from one timezone to another

Usage: tzt [OPTIONS] --time <TIME>

Options:
  -T, --time <TIME>
          Time in the format YYYY-MM-DD HH:MM:SS (you can omit HH:MM:SS) or YYYY-MM-DDTHH:MM:SS
  -f, --from <FROM_TIMEZONE>
          The original timezone (e.g. America/New_York) @see https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html [default: Your_Local_Timezone]
  -t, --to <TO_TIMEZONE>
          The target timezone (e.g. Asia/Tokyo) @see https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html# [default: Your_Local_Timezone]
  -a, --ambiguous-time-strategy <STRATEGY>
          Strategy to use for ambiguous times (earliest, latest) [default: earliest]
  -h, --help
          Print help
  -V, --version
          Print version
```

## Dependencies
This project requires the following dependencies:

- `Cargo`: The Rust package manager and build tool.
- `Make`: A build automation tool that simplifies the build process.

## Getting Started
### Install (cargo)
You can install the binary with cargo.

```bash
cargo install tzt
```

### Install (binary)
To install the binary, you can use the following command.

```bash
sudo curl -L -o \
	/usr/local/bin/tzt \
	https://github.com/shunsock/timezone_translator/releases/download/v0.1.0/timezone_translator &&\
  sudo chmod +x /usr/local/bin/tzt
```

### Install (from source)
You can also build and install the binary from source.
To build and install the project, you can use the `install` target in the Makefile.

```bash
git clone https://github.com/shunsock/timezone_translator.git
cd timezone_translator
make install
```

After installing the binary, you can run it from the command line:

```bash
$ tzt -T "2024-01-01 12:00:00" -f "America/New_York" -t "UTC"
2024-01-01 17:00:00 UTC
```

### Uninstalling
You can uninstall the binary with cargo command.

```bash
cargo uninstall tzt
```

To remove the installed binary, use the `uninstall` command
if you installed by curl or built from source.

```bash
make uninstall
```

## Ambiguous Time Strategy
There are two strategies for ambiguous times: `earliest` and `latest` to handle ambiguous times.

Ambiguous times occur when the clocks are set back for daylight saving time (DST). When DST starts, the clock forwards by one hour, and when DST ends, the clock moves back by one hour. This means that there is one hour that occurs twice in the fall when the clock moves back. The `earliest` strategy uses the first occurrence of the time, and the `latest` strategy uses the second occurrence of the time.

tzt use earliest strategy for ambiguous times by default. 
```bash
$ tzt --time '2024-11-03 01:30:00' --from 'America/New_York' --to 'UTC'
2024-11-03 05:30:00 UTC
```

If you want to use latest strategy, you can use `ambiguous-time-strategy` (`-a`) option.
```bash
$ tzt --time '2024-11-03 01:30:00' --from 'America/New_York' --to 'UTC' --ambiguous-time-strategy 'latest'
2024-11-03 06:30:00 UTC
```

## Error Handling
tzt output Validation Error when `tzt validator` finds invalid value.

this is an example of an invalid time format. you can see all valid time formats by using `tzt --help`.
```bash
$ tzt --time '2024-01-' --from 'America/New_York' --to 'UTC'
Invalid time format found: 2024-01- (expected: YYYY-MM-DD hh:mm:ss)
```

this is an example of an invalid timezone. you can check all valid inputs by looking `https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html` because, tzt uses `chrono-tz` library internally.
```bash
$ tzt --time '2024-03-10 02:30:00' --from 'America/New_York' --to 'NOT EXIST'
Invalid timezone found: NOT EXIST. @see https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html
```

`tzt translator` can handle the case where the output time and timezone do not exist.
```bash
$ tzt --time '2024-03-10 02:30:00' --from 'America/New_York' --to 'America/Los_Angeles'
Translation Error: Output time and timezone does not exist. Please check DST rules.
```

## LICENSE
This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
