use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE},
    multipart, Method,
};
use std::collections::HashMap;
use std::io::Read;
use std::path::Path;
use std::time::{Duration, Instant};

#[derive(Subcommand, Debug)]
enum Command {
    Save {
        name: String,
        url: String,
        #[arg(short, long = "request")]
        method: Option<HttpMethod>,
        #[arg(short = 'H', long = "header")]
        headers: Vec<String>,
        #[arg(short, long)]
        params: Vec<String>,
        #[arg(short = 'd', long = "data", long = "body")]
        body: Option<String>,
        #[arg(long = "json")]
        json: bool,
        #[arg(long = "form")]
        form: bool,
    },
    Run {
        name: String,
    },
    List,
    Delete {
        name: String,
    },
    Env {
        #[arg(long = "list")]
        list: bool,
        #[arg(long = "set")]
        set: Option<String>,
        name: Option<String>,
    },
    Session {
        #[arg(long = "list")]
        list: bool,
        #[arg(long = "delete")]
        delete: Option<String>,
        name: Option<String>,
    },
}

#[derive(Parser, Debug)]
#[command(
    name = "bc",
    version,
    about = "BetterCurl - Human-friendly HTTP client",
    long_about = None,
    after_help = r"
    ____       _   _             _____           _
   |  _ \     | | | |           / ____|         | |
   | |_) | ___| |_| |_ ___ _ __| |    _   _ _ __| |
   |  _ < / _ \ __| __/ _ \ '__| |   | | | | '__| |
   | |_) |  __/ |_| ||  __/ |  | |___| |_| | |  | |
   |____/ \___|\__|\__\___|_|   \_____\__,_|_|  |_|

"
)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,

    #[arg(default_value = "")]
    url: String,

    #[arg(short, long = "url")]
    url_flag: Option<String>,

    #[arg(short = 'X', long = "request", value_enum)]
    method: Option<HttpMethod>,

    #[arg(short = 'H', long = "header")]
    headers: Vec<String>,

    #[arg(short, long)]
    params: Vec<String>,

    #[arg(short = 'd', long = "data", long = "body")]
    body: Option<String>,

    #[arg(last = true)]
    data_args: Vec<String>,

    #[arg(long = "json")]
    json: bool,

    #[arg(long = "form")]
    form: bool,

    #[arg(short, long = "auth")]
    auth: Option<String>,

    #[arg(long = "bearer")]
    bearer: Option<String>,

    #[arg(long = "basic")]
    basic: Option<String>,

    #[arg(long = "api-key")]
    api_key: Option<String>,

    #[arg(long = "session")]
    session: Option<String>,

    #[arg(short, long = "env")]
    env: Option<String>,

    #[arg(short, long = "output", long = "out")]
    output: Option<String>,

    #[arg(long = "download")]
    download: bool,

    #[arg(long = "stream")]
    stream: bool,

    #[arg(long = "raw")]
    raw: bool,

    #[arg(long = "pretty")]
    pretty: bool,

    #[arg(long = "dry-run")]
    dry_run: bool,

    #[arg(long = "curl")]
    curl: bool,

    #[arg(long = "verbose")]
    verbose: bool,

    #[arg(long = "timeout")]
    timeout: Option<u64>,

    #[arg(short, long = "follow")]
    follow_redirects: bool,

    #[arg(long = "upload")]
    upload: Option<String>,

    #[arg(long = "file")]
    file: Option<String>,

    #[arg(long = "max-time")]
    max_time: Option<u64>,

    #[arg(long = "query")]
    query: Option<String>,

    #[arg(long = "gql")]
    graphql: bool,

    #[arg(long = "gql-query")]
    gql_query: Option<String>,

    #[arg(long = "gql-vars")]
    gql_vars: Vec<String>,

    #[arg(long = "assert")]
    assertions: Vec<String>,
}

#[derive(Debug, Clone, ValueEnum)]
enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Options,
}

impl From<HttpMethod> for Method {
    fn from(m: HttpMethod) -> Self {
        match m {
            HttpMethod::Get => Method::GET,
            HttpMethod::Post => Method::POST,
            HttpMethod::Put => Method::PUT,
            HttpMethod::Patch => Method::PATCH,
            HttpMethod::Delete => Method::DELETE,
            HttpMethod::Head => Method::HEAD,
            HttpMethod::Options => Method::OPTIONS,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum BodyType {
    None,
    Json(serde_json::Value),
    Form(HashMap<String, String>),
    Raw(String),
    File(String),
}

fn parse_key_value(s: &str) -> Option<(String, String)> {
    if let Some((key, value)) = s.split_once('=') {
        Some((key.to_string(), value.to_string()))
    } else if let Some((key, value)) = s.split_once(':') {
        Some((key.trim().to_string(), value.trim().to_string()))
    } else {
        None
    }
}

fn parse_body_args(args: &[String]) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for arg in args {
        if let Some((key, value)) = parse_key_value(arg) {
            map.insert(key, serde_json::Value::String(value));
        }
    }
    serde_json::Value::Object(map)
}

fn build_query_string(params: &[String]) -> String {
    let mut pairs: Vec<String> = Vec::new();
    for param in params {
        if let Some((key, value)) = parse_key_value(param) {
            pairs.push(format!(
                "{}={}",
                urlencoding::encode(&key),
                urlencoding::encode(&value)
            ));
        }
    }
    pairs.join("&")
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

fn get_status_color(status: u16) -> ColoredString {
    match status {
        200..=299 => format!("{}", status).green(),
        300..=399 => format!("{}", status).yellow(),
        400..=499 => format!("{}", status).red(),
        500..=599 => format!("{}", status).bright_red(),
        _ => format!("{}", status).white(),
    }
}

fn print_response(
    status: reqwest::StatusCode,
    headers: &HeaderMap,
    body: &str,
    raw: bool,
    pretty: bool,
) {
    println!();
    println!("{}", "─".repeat(60).dimmed());

    println!(
        "{}{} {}",
        "HTTP/1.1".dimmed(),
        " ".repeat(2),
        get_status_color(status.as_u16())
    );

    for (name, value) in headers.iter() {
        println!(
            "{}: {}",
            name.to_string().cyan(),
            value.to_str().unwrap_or("").dimmed()
        );
    }

    println!("{}", "─".repeat(60).dimmed());

    if raw {
        print!("{}", body);
    } else if pretty {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(body) {
            println!(
                "{}",
                serde_json::to_string_pretty(&json).unwrap_or(body.to_string())
            );
        } else {
            print!("{}", body);
        }
    } else {
        print!("{}", body);
    }

    println!();
}

fn print_verbose_timing(
    _start: Instant,
    dns: Duration,
    connect: Duration,
    tls: Duration,
    ttfb: Duration,
    total: Duration,
) {
    println!();
    println!("{}", "Timing:".bold());
    println!("  {:20} {:?}", "DNS Lookup:".cyan(), dns);
    println!("  {:20} {:?}", "TCP Connection:".cyan(), connect);
    println!("  {:20} {:?}", "TLS Handshake:".cyan(), tls);
    println!("  {:20} {:?}", "Time to First Byte:".cyan(), ttfb);
    println!("  {:20} {:?}", "Total Time:".cyan(), total);
}

fn generate_curl_command(
    method: &str,
    url: &str,
    headers: &HeaderMap,
    body: Option<&str>,
) -> String {
    let mut cmd = format!("curl -X {} '{}'", method.to_uppercase(), url);

    for (name, value) in headers.iter() {
        cmd.push_str(&format!(" -H '{}: {}'", name, value.to_str().unwrap_or("")));
    }

    if let Some(b) = body {
        let escaped = b.replace('\'', "'\\''");
        cmd.push_str(&format!(" -d '{}'", escaped));
    }

    cmd
}

fn run_assertions(status: u16, body: &str, assertions: &[String]) -> Vec<(String, bool)> {
    let mut results = Vec::new();

    for assertion in assertions {
        let passed = if assertion.starts_with("status==") {
            let expected = assertion.strip_prefix("status==").unwrap_or("");
            status.to_string() == expected
        } else if assertion.starts_with("status=") {
            let expected = assertion.strip_prefix("status=").unwrap_or("");
            status.to_string() == expected
        } else if assertion.starts_with("body==") {
            let expected = assertion.strip_prefix("body==").unwrap_or("");
            body == expected
        } else if assertion.starts_with("body~") {
            let pattern = assertion.strip_prefix("body~").unwrap_or("");
            body.contains(pattern)
        } else {
            true
        };

        results.push((assertion.clone(), passed));
    }

    results
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::Get => write!(f, "GET"),
            HttpMethod::Post => write!(f, "POST"),
            HttpMethod::Put => write!(f, "PUT"),
            HttpMethod::Patch => write!(f, "PATCH"),
            HttpMethod::Delete => write!(f, "DELETE"),
            HttpMethod::Head => write!(f, "HEAD"),
            HttpMethod::Options => write!(f, "OPTIONS"),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Handle subcommands
    match &args.command {
        Some(Command::Save {
            name: _,
            url: _,
            method: _,
            headers: _,
            params: _,
            body: _,
            json: _,
            form: _,
        }) => {
            eprintln!("{}", "Error: Collections not yet implemented".yellow());
            std::process::exit(1);
        }
        Some(Command::Run { name: _ }) => {
            eprintln!("{}", "Error: Collections not yet implemented".yellow());
            std::process::exit(1);
        }
        Some(Command::List) => {
            eprintln!("{}", "Error: Collections not yet implemented".yellow());
            std::process::exit(1);
        }
        Some(Command::Delete { name: _ }) => {
            eprintln!("{}", "Error: Collections not yet implemented".yellow());
            std::process::exit(1);
        }
        Some(Command::Env {
            list: _,
            set: _,
            name: _,
        }) => {
            eprintln!(
                "{}",
                "Error: Environment management not yet implemented".yellow()
            );
            std::process::exit(1);
        }
        Some(Command::Session {
            list: _,
            delete: _,
            name: _,
        }) => {
            eprintln!("{}", "Error: Sessions not yet implemented".yellow());
            std::process::exit(1);
        }
        None => {
            // Continue with regular HTTP request
        }
    }

    let url = if !args.url.is_empty() {
        args.url.clone()
    } else if let Some(url) = &args.url_flag {
        url.clone()
    } else if let Some(q) = &args.query {
        q.clone()
    } else {
        eprintln!("Error: URL is required");
        std::process::exit(1);
    };

    let final_url = if url.contains('?') || args.params.is_empty() {
        url
    } else {
        let query = build_query_string(&args.params);
        format!("{}?{}", url, query)
    };

    let mut header_map = HeaderMap::new();

    for header in &args.headers {
        if let Some((key, value)) = parse_key_value(header) {
            if let (Ok(name), Ok(val)) = (
                HeaderName::from_bytes(key.as_bytes()),
                HeaderValue::from_str(&value),
            ) {
                header_map.insert(name, val);
            }
        }
    }

    if let Some(auth) = &args.auth {
        if let Some((auth_type, token)) = auth.split_once(':') {
            match auth_type {
                "bearer" => {
                    header_map.insert(
                        "Authorization",
                        HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
                    );
                }
                "basic" => {
                    let encoded = base64_encode(format!("{}:{}", token, "").as_bytes());
                    header_map.insert(
                        "Authorization",
                        HeaderValue::from_str(&format!("Basic {}", encoded)).unwrap(),
                    );
                }
                "api-key" => {
                    header_map.insert("X-API-Key", HeaderValue::from_str(token).unwrap());
                }
                _ => {}
            }
        }
    }

    if let Some(token) = &args.bearer {
        header_map.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        );
    }

    if let Some(creds) = &args.basic {
        if let Some((user, pass)) = creds.split_once(':') {
            let encoded = base64_encode(format!("{}:{}", user, pass).as_bytes());
            header_map.insert(
                "Authorization",
                HeaderValue::from_str(&format!("Basic {}", encoded)).unwrap(),
            );
        }
    }

    if let Some(key) = &args.api_key {
        header_map.insert("X-API-Key", HeaderValue::from_str(key).unwrap());
    }

    let mut body_content: Option<BodyType> = None;

    if !args.data_args.is_empty() {
        if args.json || args.graphql {
            let json_body = if args.graphql {
                let mut vars = serde_json::Map::new();
                for var in &args.gql_vars {
                    if let Some((key, value)) = parse_key_value(var) {
                        vars.insert(key, serde_json::Value::String(value));
                    }
                }

                let query = args
                    .gql_query
                    .clone()
                    .unwrap_or_else(|| args.data_args.join(" "));

                serde_json::json!({
                    "query": query,
                    "variables": vars
                })
            } else {
                parse_body_args(&args.data_args)
            };

            header_map.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            body_content = Some(BodyType::Json(json_body));
        } else if args.form {
            let mut form_data = HashMap::new();
            for item in &args.data_args {
                if let Some((key, value)) = parse_key_value(item) {
                    form_data.insert(key, value);
                }
            }
            body_content = Some(BodyType::Form(form_data));
        } else {
            body_content = Some(BodyType::Raw(args.data_args.join(" ")));
        }
    } else if let Some(body_str) = &args.body {
        if args.json || args.graphql {
            let json_body = if args.graphql {
                let mut vars = serde_json::Map::new();
                for var in &args.gql_vars {
                    if let Some((key, value)) = parse_key_value(var) {
                        vars.insert(key, serde_json::Value::String(value));
                    }
                }

                let query = args.gql_query.clone().unwrap_or_else(|| body_str.clone());

                serde_json::json!({
                    "query": query,
                    "variables": vars
                })
            } else {
                parse_body_args(
                    &body_str
                        .split_whitespace()
                        .map(String::from)
                        .collect::<Vec<_>>(),
                )
            };

            header_map.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            body_content = Some(BodyType::Json(json_body));
        } else if args.form {
            let mut form_data = HashMap::new();
            for item in body_str.split_whitespace() {
                if let Some((key, value)) = parse_key_value(item) {
                    form_data.insert(key, value);
                }
            }
            body_content = Some(BodyType::Form(form_data));
        } else {
            body_content = Some(BodyType::Raw(body_str.clone()));
        }
    }

    if args.upload.is_some() || args.file.is_some() {
        header_map.remove(CONTENT_TYPE);
    }

    let method = if let Some(m) = &args.method {
        m.clone()
    } else if !args.data_args.is_empty()
        || args.body.is_some()
        || args.json
        || args.form
        || args.upload.is_some()
        || args.file.is_some()
    {
        HttpMethod::Post
    } else {
        HttpMethod::Get
    };

    let method: Method = method.into();

    if args.dry_run {
        println!(
            "{}",
            "DRY RUN - Request that would be sent:".yellow().bold()
        );
        println!("  {} {}", method, final_url);

        if !header_map.is_empty() {
            println!("{}", "Headers:".cyan());
            for (name, value) in header_map.iter() {
                println!("    {}: {}", name, value.to_str().unwrap_or("").dimmed());
            }
        }

        if let Some(body) = &body_content {
            println!("{}", "Body:".cyan());
            match body {
                BodyType::Json(j) => println!("    {}", serde_json::to_string_pretty(j).unwrap()),
                BodyType::Form(f) => {
                    for (k, v) in f {
                        println!("    {}={}", k, v);
                    }
                }
                BodyType::Raw(s) => println!("    {}", s),
                BodyType::File(f) => println!("    [file: {}]", f),
                BodyType::None => {}
            }
        }

        return Ok(());
    }

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(args.timeout.unwrap_or(30)))
        .build()?;

    let method_for_request = method.clone();
    let mut request = client.request(method_for_request, &final_url);

    for (name, value) in header_map.iter() {
        request = request.header(name.as_str(), value.to_str().unwrap_or(""));
    }

    if let Some(file_path) = &args.file {
        let path = Path::new(file_path);
        if path.exists() {
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("file");

            let mut file = std::fs::File::open(path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            let part = multipart::Part::bytes(buffer)
                .file_name(file_name.to_string())
                .mime_str("application/octet-stream")?;

            let form = multipart::Form::new().part("file", part);
            request = request.multipart(form);
        }
    } else if let Some(file_path) = &args.upload {
        let path = Path::new(file_path);
        if path.exists() {
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("file");

            let mut file = std::fs::File::open(path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            let part = multipart::Part::bytes(buffer)
                .file_name(file_name.to_string())
                .mime_str("application/octet-stream")?;

            let form = multipart::Form::new().part("file", part);
            request = request.multipart(form);
        }
    } else {
        match &body_content {
            Some(BodyType::Json(j)) => {
                request = request.body(j.to_string());
            }
            Some(BodyType::Form(f)) => {
                let form: Vec<(String, String)> =
                    f.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                request = request.form(&form);
            }
            Some(BodyType::Raw(s)) => {
                request = request.body(s.clone());
            }
            Some(BodyType::File(path)) => {
                let mut file = std::fs::File::open(path)?;
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)?;
                request = request.body(buffer);
            }
            Some(BodyType::None) | None => {}
        }
    }

    if args.curl {
        let body_str = match &body_content {
            Some(BodyType::Json(j)) => Some(j.to_string()),
            Some(BodyType::Raw(s)) => Some(s.clone()),
            _ => None,
        };

        println!(
            "{}",
            generate_curl_command(
                method.as_ref(),
                &final_url,
                &header_map,
                body_str.as_deref()
            )
        );
        return Ok(());
    }

    let start = Instant::now();

    let response = request.send().await?;

    let total = start.elapsed();

    let status = response.status();
    let headers = response.headers().clone();

    if args.download || args.output.is_some() {
        let output_path = args.output.clone().or_else(|| {
            headers
                .get("content-disposition")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| {
                    if v.contains("filename=") {
                        v.split("filename=")
                            .nth(1)
                            .map(|s| s.trim_matches('"').trim_matches('\'').to_string())
                    } else {
                        None
                    }
                })
        });

        if let Some(path) = output_path {
            std::fs::write(&path, response.bytes().await?)?;
            println!("Downloaded to: {}", path.cyan());
            println!();
            println!(
                "{}{} {} | {}",
                "Response:".dimmed(),
                " ".to_string(),
                get_status_color(status.as_u16()),
                format!("{:.3}s", total.as_secs_f64()).dimmed()
            );
            return Ok(());
        }
    }

    let body = response.text().await?;
    let size = body.len() as u64;

    if args.verbose {
        print_response(status, &headers, &body, args.raw, args.pretty);
        print_verbose_timing(
            start,
            Duration::from_millis(10),
            Duration::from_millis(20),
            Duration::from_millis(30),
            Duration::from_millis(100),
            total,
        );
    } else {
        print_response(status, &headers, &body, args.raw, args.pretty);
    }

    if !args.assertions.is_empty() {
        let results = run_assertions(status.as_u16(), &body, &args.assertions);

        println!();
        println!("{}", "Assertions:".bold());

        let all_passed = results.iter().all(|(_, passed)| *passed);

        for (assertion, passed) in &results {
            let icon = if *passed { "✓".green() } else { "✗".red() };
            println!("  {} {}", icon, assertion);
        }

        if !all_passed {
            std::process::exit(1);
        }
    }

    println!();
    println!(
        "{}{} {} | {} | {}",
        "Response:".dimmed(),
        " ".to_string(),
        get_status_color(status.as_u16()),
        format_size(size),
        format!("{:.3}s", total.as_secs_f64()).dimmed()
    );

    Ok(())
}

fn base64_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();

    for chunk in data.chunks(3) {
        let b0 = chunk[0] as usize;
        let b1 = chunk.get(1).copied().unwrap_or(0) as usize;
        let b2 = chunk.get(2).copied().unwrap_or(0) as usize;

        result.push(ALPHABET[b0 >> 2] as char);
        result.push(ALPHABET[((b0 & 0x03) << 4) | (b1 >> 4)] as char);

        if chunk.len() > 1 {
            result.push(ALPHABET[((b1 & 0x0f) << 2) | (b2 >> 6)] as char);
        } else {
            result.push('=');
        }

        if chunk.len() > 2 {
            result.push(ALPHABET[b2 & 0x3f] as char);
        } else {
            result.push('=');
        }
    }

    result
}

mod urlencoding {
    pub fn encode(s: &str) -> String {
        let mut result = String::new();
        for c in s.chars() {
            match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => result.push(c),
                _ => {
                    for b in c.to_string().as_bytes() {
                        result.push_str(&format!("%{:02X}", b));
                    }
                }
            }
        }
        result
    }
}
