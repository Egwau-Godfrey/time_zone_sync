# Timezone Converter

A Rust library for handling timezone conversions and retrieving timezone information with ease. This library wraps around `chrono` and `chrono-tz` to provide a simple interface for working with timezones.

## Features

- Convert times between any supported timezone
- Get current time in different timezones
- Retrieve timezone information (name, offset, DST status)
- Calculate time differences between timezones
- Error handling for invalid timezones and conversion failures

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
timezone-converter = "0.1.0"
chrono = "0.4"
chrono-tz = "0.8"
``` 