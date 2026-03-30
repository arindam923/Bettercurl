🚀 BetterCurl — Project Plan
Core Philosophy

"Make HTTP feel like a conversation, not a configuration file."

Inspired by tools like httpie, xh, and gh, but going further — BetterCurl should be the tool developers actually reach for first.

📦 Feature List
1. Human-Readable Syntax

Intuitive argument parsing — bc GET https://api.example.com instead of curl -X GET
Auto-detect method from context (body present → POST, no body → GET)
Key=value syntax for headers, query params, body — Authorization:Bearer token foo=bar
JSON body shorthand — bc POST api.com name=John age=30 auto-serializes to JSON

2. Beautiful Output

Syntax-highlighted JSON/XML/HTML responses in the terminal
Colored status codes (green 2xx, yellow 3xx, red 4xx/5xx)
Formatted headers section, response time, and size display
Pretty-print mode by default, with --raw flag to disable
Diff mode — compare two responses side by side

3. Authentication Made Easy

Built-in auth modes: --auth bearer:TOKEN, --auth basic:user:pass, --auth api-key:KEY
OAuth 2.0 flow support — browser-based token fetch with --auth oauth2
Credential store — save named credentials: bc auth save prod --bearer TOKEN

4. Sessions & State

Named sessions that persist cookies, headers, auth: bc --session prod GET /users
Session history — replay any past request from a session
Cookie jar auto-management

5. Request Collections (like Postman, but CLI)

Save requests: bc save login POST /auth --body '...'
Run saved requests: bc run login
Collections as YAML/JSON files — version controllable, team-shareable
Import/export Postman collections and OpenAPI specs

6. Environment Management

.bcenv files for environment variables (local, staging, prod)
bc --env prod GET /users auto-swaps base URLs, tokens, etc.
.bcenv.local gitignore pattern built-in to protect secrets

7. Chaining & Scripting

Pipe support — bc GET /token | bc POST /data --body @stdin
Variable extraction — bc GET /login --extract token=$.data.token saves to session
Chain mode — define multi-step flows in YAML and run with bc chain auth-flow.yaml

8. WebSocket & SSE Support

bc ws wss://echo.websocket.org — interactive WebSocket session
bc sse https://api.example.com/stream — pretty-print Server-Sent Events live

9. GraphQL First-Class Support

bc gql POST /graphql --query "{ users { id name } }"
Variables support: --gql-vars id=1
Introspection helper: bc gql introspect /graphql

10. Mocking & Offline Mode

bc mock start --port 3001 — spin up a local mock server from a collection
Record mode — bc --record proxies real traffic and saves to a mock file

11. Testing & Assertions

Inline assertions: bc GET /health --assert status==200 --assert body.status==ok
CI-friendly exit codes based on assertion results
bc test suite.yaml — run a test suite of chained requests with pass/fail reporting

12. Plugins & Extensibility

Plugin system: bc plugin install bc-aws-sigv4
Hook scripts (pre/post request) in shell or JS
Custom output formatters

13. Developer Experience (DX) Polish

Interactive mode — bc interactive for a REPL-style HTTP console
Shell completions for zsh, bash, fish out of the box
--dry-run — shows the exact request that would be sent (great for debugging)
--curl flag — outputs the equivalent curl command for portability
--verbose with timing waterfall (DNS, TCP, TLS, TTFB, total)
Progress bar for large file uploads/downloads

14. File Handling

Upload: bc POST /upload file@./photo.jpg (multipart auto-detected)
Download with progress: bc GET /file.zip --out ./file.zip
Stream large responses with --stream

15. Config & Defaults

Global config at ~/.config/bettercurl/config.yaml
Per-project config at .bcrc (auto-loaded)
Set defaults: bc config set default.timeout 30s


🛠 Suggested Tech Stack
ConcernRecommendationLanguageRust (fast, single binary) or GoHTTPreqwest (Rust) or net/http (Go)Output formattingsyntect for syntax highlightingConfigTOML/YAML via serdeCLI parsingclap (Rust) or cobra (Go)DistributionHomebrew tap, cargo install, apt/yum
