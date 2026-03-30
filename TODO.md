# BetterCurl Roadmap

This document tracks the development progress and planned features for BetterCurl.

## ✅ Implemented (v0.1.0)

- [x] Basic HTTP methods (GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS)
- [x] Intuitive CLI interface (no need for `-X GET` etc)
- [x] Auto method detection from body presence
- [x] Key=value syntax for headers (`-H "Key:Value"`) and query params (`-p key=value`)
- [x] JSON body shorthand (`--json -- name=value age=30`)
- [x] Form data encoding (`--form -- key=value`)
- [x] Beautiful colored output (status codes, headers)
- [x] JSON pretty-print (`--pretty` default)
- [x] Raw output mode (`--raw`)
- [x] Authentication:
  - [x] Bearer tokens (`--bearer TOKEN` or `--auth bearer:TOKEN`)
  - [x] Basic auth (`--basic user:pass` or `--auth basic:user:pass`)
  - [x] API Key (`--api-key KEY` or `--auth api-key:KEY`)
- [x] GraphQL support (`--gql`, `--gql-query`, `--gql-vars`)
- [x] File upload (`--upload file.txt` or `--file file.txt`)
- [x] File download (`--download --out file.zip`)
- [x] Assertions for API testing (`--assert status==200`, `--assert body~pattern`)
- [x] CI-friendly exit codes (1 on assertion failure)
- [x] Dry-run mode (`--dry-run`)
- [x] Curl command generation (`--curl`)
- [x] Verbose mode with timing (`--verbose`)
- [x] URL query parameters on command line (`-p key=value`)
- [x] ASCII logo in help output
- [x] GitHub Actions CI/CD for automated builds
- [x] Release automation workflow

## 🚧 In Progress / Partial

- [ ] **Streaming** - `--stream` flag exists but not fully implemented
- [ ] **Verbose timing** - Shows mock values currently, needs real DNS/TCP/TLS/TTFB metrics

## ❌ Planned / Future Releases

### Collections & Workflows (v0.2.0)
- [ ] Request collections: `bc save mysession https://api...` and `bc run mysession`
- [ ] `bc list` to show saved collections
- [ ] `bc delete <name>` to remove saved requests
- [ ] YAML/JSON file format for collections (version control friendly)
- [ ] Import/export Postman collections
- [ ] Import/export OpenAPI specs

### Environments & Sessions (v0.3.0)
- [ ] Environment variables: `.bcenv` files for different stages (local/staging/prod)
- [ ] `bc --env prod https://...` auto-substitutes base URLs, tokens, etc.
- [ ] `.bcenv.local` auto-gitignored for secrets
- [ ] Session persistence: cookies, headers, auth across multiple requests
- [ ] Session history and replay
- [ ] `bc session list`, `bc session delete <name>`

### Advanced Features (v0.4.0)
- [ ] WebSocket support: `bc ws wss://echo.websocket.org`
- [ ] Server-Sent Events (SSE): `bc sse https://api/stream`
- [ ] Diff mode: `bc GET /v1 --diff GET /v2` side-by-side comparison
- [ ] Syntax highlighting (full AST-based, not just colors)
- [ ] Interactive REPL mode: `bc interactive`
- [ ] Shell completions (bash, zsh, fish)
- [ ] Progress bars for large uploads/downloads

### Chaining & Scripting (v0.5.0)
- [ ] Pipe support: `bc GET /token | bc POST /data --body @stdin`
- [ ] Variable extraction: `bc GET /login --extract token=$.data.token`
- [ ] Chain mode: define multi-step flows in YAML

### Mocking & Testing (v0.6.0)
- [ ] Mock server: `bc mock start --port 3001` from a collection
- [ ] Record mode: `bc --record proxies real traffic` to create mocks
- [ ] Test suites: `bc test suite.yaml` with structured reporting
- [ ] JUnit/XML output for CI integration
- [ ] Load testing / concurrency basics

### Extensibility & Plugins (v0.7.0)
- [ ] Plugin system: `bc plugin install bc-aws-sigv4`
- [ ] Hook scripts (pre/post request) in shell or JS
- [ ] Custom output formatters
- [ ] Custom authentication schemes

### DX Polish (v0.8.0)
- [ ] Configuration file: `~/.config/bettercurl/config.yaml`
- [ ] Per-project config: `.bcrc`
- [ ] Set defaults: `bc config set default.timeout 30s`
- [ ] Command aliasing
- [ ] History file (like bash history) for past requests
- [ ] Fuzzel/fzf integration for picking requests
- [ ] JSONPath/jq integration for body extraction (`--jq .data.items`)

### Package Distribution
- [ ] Publish to crates.io (for `cargo install bettercurl`)
- [ ] Homebrew tap/formula
- [ ] APT repository (Debian/Ubuntu)
- [ ] YUM/DNF repository (RHEL/Fedora/CentOS)
- [ ] Scoop bucket (Windows)
- [ ] Snap package
- [ ] Chocolatey package (Windows)

## 🐛 Known Issues

1. `--verbose` timing shows hardcoded mock values, not real measurements (GitHub Issue: #1)
2. Collections, sessions, and environments are stubbed but not implemented
3. No streaming support for large downloads
4. File upload limited to single file via `--upload`/`--file`

## 📈 Metrics & Goals

- **Binary size**: <10MB (✅ Currently ~7MB stripped)
- **Build time**: <30s on CI (✅ ~4-5s)
- **Performance**: 2x faster than curl for simple requests (benchmarking needed)
- **Compatibility**: 100% compatible with curl for common use cases

---

## Contributing

We welcome contributions! Please check the issues page and the roadmap above for areas that need help.
