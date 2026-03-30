# BetterCurl

**Make HTTP feel like a conversation, not a configuration file.**

BetterCurl is a modern, human-friendly HTTP client that combines the simplicity of `curl` with an intuitive syntax and beautiful output. Built in Rust for speed and reliability.

## ✨ Features

- **Intuitive syntax** - `bc https://api.example.com/users` just works
- **Auto method detection** - Body present? POST. No body? GET.
- **JSON shorthand** - `bc POST api.com name=John age=30` auto-serializes to JSON
- **Beautiful output** - Colorized status codes, syntax-highlighted JSON, response timing
- **Built-in auth** - Bearer, Basic, API Key support
- **GraphQL ready** - First-class GraphQL queries with variables
- **File handling** - Easy uploads and downloads with progress
- **Assertions** - Test APIs inline: `--assert status==200 --assert body~success`
- **Dry-run & curl export** - See exactly what will be sent or generate portable curl commands

## 📦 Installation

### Pre-built Binaries

Download the latest binary for your platform from the [Releases page](https://github.com/yourusername/bettercurl/releases).

```bash
# Linux/macOS
chmod +x bettercurl
sudo mv bettercurl /usr/local/bin/

# Windows (WSL)
chmod +x bettercurl.exe
sudo mv bettercurl.exe /usr/local/bin/
```

### Build from Source

Requires Rust 1.70+ and Cargo.

```bash
git clone https://github.com/yourusername/bettercurl.git
cd bettercurl
cargo build --release
sudo cp target/release/bettercurl /usr/local/bin/
```

### Package Managers (Coming Soon)

- **Homebrew** (macOS/Linux): `brew install bettercurl`
- **APT** (Debian/Ubuntu): `apt install bettercurl`
- **YUM/DNF** (RHEL/Fedora): `yum install bettercurl`
- **Scoop** (Windows): `scoop install bettercurl`

## 🚀 Quick Start

### Basic Requests

```bash
# GET request
bc https://httpbin.org/get

# POST with JSON body
bc https://httpbin.org/post --json -- name=Alice age=30 city=NYC

# POST with form data
bc https://httpbin.org/post --form -- username=admin password=secret
```

### Headers & Auth

```bash
# Custom headers
bc https://api.example.com -H "X-API-Key: your-key" -H "Content-Type: application/json"

# Bearer token
bc https://api.example.com/protected --bearer YOUR_TOKEN

# Basic auth
bc https://api.example.com/admin --basic admin:password

# API key header
bc https://api.example.com --api-key YOUR_KEY
```

### Query Parameters

```bash
bc https://api.example.com/search -p q=rust -p limit=10 -p sort=desc
```

### File Upload & Download

```bash
# Upload a file
bc https://httpbin.org/post --upload image.jpg

# Download a file
bc https://example.com/file.zip --download --out file.zip
```

### GraphQL

```bash
bc https://api.example.com/graphql --gql --gql-query '{ users { id name } }'
bc https://api.example.com/graphql --gql --gql-query 'query GetUser($id: Int) { user(id: $id) { name } }' --gql-vars id=123
```

### Assertions & Testing

```bash
# Check status code
bc https://api.example.com/health --assert status==200

# Check response body contains text
bc https://api.example.com/status --assert status==ok --assert version~1.0

# Fails exit code if any assertion fails - perfect for CI/CD
```

### Dry Run & Curl Export

```bash
# See what would be sent without making request
bc https://api.example.com --dry-run --json -- name=test

# Generate equivalent curl command
bc https://api.example.com --curl -H "Authorization: Bearer TOKEN"
```

### Pretty Output

```bash
# Colorized, pretty-printed JSON (default for JSON responses)
bc https://api.example.com/data --pretty

# Raw output (no formatting)
bc https://api.example.com/data --raw
```

## 🔧 Options

| Option | Description |
|--------|-------------|
| `-X, --request <METHOD>` | HTTP method (GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS) |
| `-H, --header <HEADER>` | Add custom header (format: `Key:Value`) |
| `-p, --params <PARAM>` | Add query parameter (format: `key=value`) |
| `-d, --body <BODY>` | Raw request body |
| `--json` | Send JSON body (auto-detects `key=value` args) |
| `--form` | Send form-encoded data |
| `--bearer <TOKEN>` | Bearer authentication |
| `--basic <USER:PASS>` | Basic authentication |
| `--api-key <KEY>` | API key (adds `X-API-Key` header) |
| `--download, --out <FILE>` | Download response to file |
| `--upload <FILE>` | Upload file as multipart/form-data |
| `--pretty, --raw` | Control response formatting |
| `--dry-run` | Show request without sending |
| `--curl` | Output curl command instead of executing |
| `--verbose` | Show timing breakdown |
| `--assert <COND>` | Assert response condition (e.g., `status==200`, `body~text`) |
| `--timeout <SECONDS>` | Request timeout |
| `-f, --follow` | Follow redirects |

## 📋 TODO / Roadmap

- [ ] Collections - Save and reuse request templates
- [ ] Environments - Manage variables for different deployment stages
- [ ] Sessions - Cookie persistence across requests
- [ ] WebSocket & SSE support
- [ ] File streaming for large downloads
- [ ] Interactive mode (REPL)
- [ ] Shell completions (bash, zsh, fish)
- [ ] Plugin system for custom output formatters
- [ ] Mock server mode for local development
- [ ] Diff mode for comparing responses
- [ ] Syntax highlighting for responses (full AST-based)
- [ ] Import/export Postman collections & OpenAPI specs
- [ ] CI/CD integration with structured test reports
- [ ] Configuration file (~/.config/bettercurl/config.yaml)
- [ ] Progress bars for large file transfers

See [plan.md](plan.md) for the complete feature vision.

## 📄 License

MIT License © 2025 BetterCurl Contributors

## 🙏 Credits

Inspired by the excellent work of [httpie](https://httpie.io/), [xh](https://github.com/ducaale/xh), and [gh](https://cli.github.com/).
