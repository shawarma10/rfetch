use clap::Parser;

#[derive(Parser, Clone)]
#[command(version, about, long_about = None)]
pub struct Flags {
    /// Sets URL
    pub url: String,

    /// Sets HTTP Method
    #[arg(short, long, default_value = "GET")]
    pub method: String,

    /// Sets HTTP Request Body
    #[arg(short, long)]
    pub body: Option<String>,

    /// Output the response body to a file
    #[arg(short, long)]
    pub output: Option<String>,

    /// Append a HTTP Header with expected format 'key:value'
    #[arg(short = 'H', long = "header")]
    pub headers: Vec<String>,

    /// Sets a query parameter with expected format 'key:value' or 'key=value'
    #[arg(short = 'Q', long = "query-param")]
    pub query_params: Vec<String>,

    /// Sets 'application/json' header and checks for valid JSON syntax
    #[arg(short, long, default_value_t = false)]
    pub json: bool,

    /// Uses Rust's pretty printing (#:?)
    #[arg(long, default_value_t = false)]
    pub debug: bool,

    /// Only prints the body
    #[arg(long, default_value_t = false)]
    pub body_only: bool,
}

impl Flags {
    pub fn extract_method(&self) -> anyhow::Result<reqwest::Method> {
        match self.method.to_lowercase().as_str() {
            "post" => Ok(reqwest::Method::POST),
            "get" => Ok(reqwest::Method::GET),
            "put" => Ok(reqwest::Method::PUT),
            "delete" => Ok(reqwest::Method::DELETE),
            "head" => Ok(reqwest::Method::HEAD),
            "options" => Ok(reqwest::Method::OPTIONS),
            "trace" => Ok(reqwest::Method::TRACE),
            "connect" => Ok(reqwest::Method::CONNECT),
            "patch" => Ok(reqwest::Method::PATCH),
            _ => anyhow::bail!("Invalid method."),
        }
    }
}
