# Rust Server Boilerplate

[![CI](https://github.com/yourusername/rust-server-boilerplate/actions/workflows/ci.yml/badge.svg)](https://github.com/yourusername/rust-server-boilerplate/actions/workflows/ci.yml)

A minimalist, production-ready Rust HTTP server boilerplate using Axum framework with built-in health checks and modern development tooling.

## Features

- 🚀 Built with [Axum](https://github.com/tokio-rs/axum) - A modern, fast Rust web framework
- ✅ Health check endpoint
- 🔧 Environment variable configuration
- 📝 Comprehensive test coverage
- 🛠️ Development tools (linting, formatting)
- 🔒 Security audit workflow
- 📦 Cargo-based build system

## Quick Start

### Prerequisites

- Rust (latest stable)
- Cargo

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/rust-server-boilerplate.git
cd rust-server-boilerplate
```

2. Build the project:
```bash
make build
```

### Running the Server

Start the server using:
```bash
make run
```

The server will start on `http://127.0.0.1:3000` by default.

## Configuration

The server can be configured using environment variables:

- `PORT`: Server port (default: 3000)

You can create a `.env` file in the project root:
```env
PORT=8080
```

## Development

### Running Tests

Run the test suite:
```bash
make test
```

### Linting and Formatting

Check code style and run lints:
```bash
make lint
```

### All Checks

Run all checks (linting and tests):
```bash
make check
```

## Project Structure

```
.
├── src/
│   ├── handlers/     # Request handlers
│   ├── router/       # Routing configuration
│   ├── lib.rs        # Library code
│   └── main.rs       # Application entry point
├── .github/          # GitHub Actions workflows
├── Cargo.toml        # Dependencies and project metadata
└── Makefile         # Build automation
```

## API Endpoints

### Health Check
- `GET /health`
- Returns "OK" with 200 status code
- Used for monitoring and load balancer checks

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) - The web framework used
- [Tokio](https://tokio.rs/) - The async runtime
