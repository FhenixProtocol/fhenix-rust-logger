# fhenix-rust-logger

A simple, configurable logging library for Rust applications within Fhenix. This library provides a thin wrapper around `tracing` and `tracing-subscriber`.

## Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
fhenix-logger = { git = "https://github.com/fhenixprotocol/fhenix-rust-log.git", version = "0.1" }
```

## Usage

### Basic Usage

For quick setup with default configuration:

```rust
use logger::init_default_logger;

fn main() -> anyhow::Result<()> {
    // Initialize logger with service name
    init_default_logger("my-service")?;

    // Use standard log macros
    log::info!("Application started");
    log::debug!("Debug information");
    log::error!("Something went wrong: {}", error);

    Ok(())
}
```

### Custom Configuration

For more control over the logger behavior:

```rust
use logger::{init_logger, LoggerConfig, LogLevel};

fn main() -> anyhow::Result<()> {
    let config = LoggerConfig {
        level: LogLevel::Debug,
        show_file: false,
        show_line_number: true,
        ..Default::default()
    };

    init_logger("my-service", &config)?;

    // Your application code here

    Ok(())
}
```

## Configuration Options

The `LoggerConfig` struct provides the following options:

| Option             | Type       | Default | Description                            |
| ------------------ | ---------- | ------- | -------------------------------------- |
| `level`            | `LogLevel` | `Info`  | The log level filter                   |
| `show_thread_id`   | `bool`     | `true`  | Show thread IDs in log output          |
| `show_thread_name` | `bool`     | `true`  | Show thread names in log output        |
| `show_file`        | `bool`     | `true`  | Show source file paths in log output   |
| `show_line_number` | `bool`     | `true`  | Show line numbers in log output        |
| `show_target`      | `bool`     | `true`  | Show target module names in log output |

## Log Levels

Available log levels (from most to least severe):

- `Error` - Error conditions
- `Warn` - Warning conditions
- `Info` - Informational messages (default)
- `Debug` - Debug-level messages
- `Trace` - Trace-level messages

## Environment Variable Configuration

This library supports the standard `RUST_LOG` environment variable for configuring log levels. When set, `RUST_LOG` will override the level specified in your code.

### Examples

```bash
# Set all logs to debug level
RUST_LOG=debug cargo run

# Set specific module to trace level, everything else to info
RUST_LOG=info,my_module=trace cargo run

# Complex filtering
RUST_LOG=warn,my_service=info,my_service::critical_component=debug cargo run
```

The syntax follows the standard format:

- A single level applies to all modules
- `module_name=level` sets a specific level for a module
- Multiple directives can be separated by commas
