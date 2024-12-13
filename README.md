
# 🔒 macOS Security Logs Collector

A high-performance, security-focused Rust application for collecting, analyzing, and monitoring macOS system security logs with built-in safeguards and rate limiting.

## 🌟 Key Features

- **Secure Log Collection**
  - Unified logs collection with granular filtering
  - Audit logs monitoring with structured output
  - Built-in permission verification
  - Rate limiting to prevent resource exhaustion

- **Performance & Safety**
  - Asynchronous operations using Tokio
  - Memory-safe implementation
  - Resource usage monitoring
  - Graceful error handling

- **Flexible Output**
  - JSON format for programmatic processing
  - Human-readable text output
  - Structured logging support
  - Sanitized output handling

## 📋 Prerequisites

- Rust 1.56.0 or later
- macOS 10.15 or later
- Administrative privileges for log access
- XCode Command Line Tools

## 🚀 Quick Start

1. Clone the repository:
```bash
git clone https://github.com/yourusername/macos_security_logs
cd macos_security_logs
```

2. Build the project:
```bash
cargo build --release
```

3. Run with basic configuration:
```bash
sudo ./target/release/macos_security_logs --max-entries 1000 --format json
```

## 💻 Usage Examples

### Basic Log Collection
```bash
# Collect last 500 security logs in JSON format
sudo ./target/release/macos_security_logs --max-entries 500 --format json

# Output in human-readable text format
sudo ./target/release/macos_security_logs --format text

# Monitor real-time security events
sudo ./target/release/macos_security_logs --watch
```

### Advanced Usage
```bash
# Custom predicate filtering
sudo ./target/release/macos_security_logs --predicate "subsystem == 'com.apple.security' AND category == 'authentication'"

# Export to file
sudo ./target/release/macos_security_logs --output security_audit.json

# Specify time range
sudo ./target/release/macos_security_logs --since "2024-01-01" --until "2024-12-31"
```

## 🛡️ Security Features

- **Permission Management**
  - Strict privilege checking
  - Least privilege principle enforcement
  - Secure credential handling

- **Resource Protection**
  - Configurable rate limiting
  - Memory usage controls
  - CPU utilization monitoring

- **Data Safety**
  - Input validation
  - Output sanitization
  - Secure error handling

## 🔧 Configuration

Configuration can be provided via command-line arguments or a config file:

```toml
# config.toml
max_entries = 1000
format = "json"
rate_limit = 100
output_file = "security_logs.json"
```

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/enhancement`)
3. Commit your changes (`git commit -am 'Add new feature'`)
4. Push to the branch (`git push origin feature/enhancement`)
5. Open a Pull Request

## 📝 Log Format Examples

### JSON Output
```json
{
  "timestamp": "2024-03-21T10:15:30Z",
  "subsystem": "com.apple.security",
  "level": "Notice",
  "message": "Security policy updated",
  "process_id": 1234,
  "thread_id": 5678
}
```

### Text Output
```
[2024-03-21 10:15:30] [Notice] com.apple.security: Security policy updated
```

## ⚠️ Error Handling

The application handles various error scenarios gracefully:
- Permission denied errors
- Rate limit exceeded
- Invalid input parameters
- System resource constraints
- File access issues

## 🔍 Testing

Run the test suite:
```bash
# Run unit tests
cargo test

# Run integration tests (requires sudo)
sudo cargo test --test '*'
```

## 📦 Dependencies

- tokio: Async runtime
- serde: Serialization
- chrono: Time handling
- clap: CLI argument parsing
- tracing: Logging infrastructure

## 📜 License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## 🔗 Related Projects

- [macOS Security Tools](https://github.com/topics/macos-security)
- [System Monitoring](https://github.com/topics/system-monitoring)
- [Log Analysis](https://github.com/topics/log-analysis)

## 📚 Documentation

Full documentation is available in the [docs](./docs) directory.

## ☎️ Support

For bug reports and feature requests, please use the [issue tracker](https://github.com/yourusername/macos_security_logs/issues).

## 🙏 Acknowledgments

- Apple's Security Documentation
- Rust Security Working Group
- macOS Security Community
