use reqwest::blocking as rq;
use reqwest::header;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::time::Duration;

static APP_USER_AGENT: &str = "PDL-RUST-SDK";

static DEFAULT_API_URL: &str = "https://api.peopledatalabs.com/";
static DEFAULT_API_VERSION: &str = "v5";
static DEFAULT_SANDBOX_URL: &str = "https://sandbox.api.peopledatalabs.com/";
static DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub enum PDLError {
    NetworkError(reqwest::Error),
    HTTPError(StatusCode),
    SerializationError,
    ValidationError,
}

impl Display for PDLError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            PDLError::NetworkError(ref e) => e.fmt(f),
            PDLError::HTTPError(ref s) => write!(f, "Invalid HTTP status code: {}", s),
            PDLError::SerializationError => f.write_str("Unable to serialize."),
            PDLError::ValidationError => f.write_str("Unable to validate."),
        }
    }
}

impl Error for PDLError {}

/// This is the struct that allows users to pass optional parameters to the PDLClient.
pub struct PDLCLientOptions {
    pub sandbox: bool,
    pub timeout: Duration,
}

impl PDLCLientOptions {
    pub fn default() -> Self {
        Self {
            sandbox: false,
            timeout: DEFAULT_TIMEOUT,
        }
    }
}

/// This is the struct that creates the connection with PeopleDataLabs.
/// It contain   s the API Key and a client that can be used both sync and async.
#[derive(Clone)]
pub struct PDLClient {
    api_key: String,
    base_url: String,
    api_version: String,
    client: rq::Client,
}

/// Builds client based off of API_KEY and Optional Timeout
fn build_client(api_key: &str, timeout: Option<Duration>) -> rq::Client {
    let mut headers = header::HeaderMap::new();
    let api_key = header::HeaderValue::from_str(api_key).unwrap();
    headers.insert("X-Api-Key", api_key);

    let duration = timeout.unwrap_or(DEFAULT_TIMEOUT);

    rq::Client::builder()
        .default_headers(headers)
        .user_agent(APP_USER_AGENT)
        .timeout(duration)
        .build()
        .expect("Failed to build reqwest client")
}

impl PDLClient {
    /// Make a new People Data Labs client with users API Key and API Version.
    pub fn new(api_key: &str) -> Self {
        let client = build_client(&api_key, None);
        PDLClient {
            api_key: api_key.to_string(),
            base_url: DEFAULT_API_URL.to_string(),
            api_version: DEFAULT_API_VERSION.to_string(),
            client,
        }
    }

    /// Adds the ability to update the version from the default through chaining.
    pub fn version(mut self, version: &str) -> Self {
        self.api_version = version.to_string();
        self
    }

    /// Adds the ability to update the default timeout or access sandbox mode
    /// through chaining.
    pub fn options(mut self, options: PDLCLientOptions) -> Self {
        if options.timeout != DEFAULT_TIMEOUT {
            self.client = build_client(&self.api_key, Some(options.timeout))
        }

        if options.sandbox {
            self.base_url = DEFAULT_SANDBOX_URL.to_string();
        }

        self
    }

    /// Builds the final PDLClient
    pub fn build(self) -> PDLClient {
        PDLClient {
            api_key: self.api_key,
            base_url: self.base_url,
            api_version: self.api_version,
            client: self.client,
        }
    }

    /// Sends a GET method through the PeopleDataLabs API. It takes an endpoint &str and params &str.
    /// It returns a generic response or PDLError.
    pub fn get<T, P>(&self, endpoint: &str, params: P) -> Result<T, PDLError>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        let query_params =
            serde_urlencoded::to_string(params).map_err(|_| PDLError::SerializationError)?;

        let uri = format!(
            "{}{}{}?{}",
            self.base_url, self.api_version, endpoint, query_params
        );

        let resp = self
            .client
            .get(uri)
            .send()
            .map_err(PDLError::NetworkError)?;

        match resp.status() {
            StatusCode::OK => {}
            StatusCode::NOT_FOUND => return Err(PDLError::HTTPError(StatusCode::NOT_FOUND)),
            other => return Err(PDLError::HTTPError(other)),
        }

        resp.json::<T>().map_err(PDLError::NetworkError)
    }

    /// Sends a POST method through the PeopleDataLabs API. It takes an endpoint &str and params &str.
    /// It returns a generic response or PDLError.
    pub fn post<T, P>(&self, endpoint: &str, params: P) -> Result<T, PDLError>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        let json = serde_json::to_value(params).map_err(|_| PDLError::ValidationError)?;

        let uri = format!("{}{}{}", self.base_url, self.api_version, endpoint);

        let resp = self
            .client
            .post(uri)
            .json(&json)
            .send()
            .map_err(PDLError::NetworkError)?;

        match resp.status() {
            StatusCode::OK => {}
            other => return Err(PDLError::HTTPError(other)),
        }

        resp.json::<T>().map_err(PDLError::NetworkError)
    }
}
