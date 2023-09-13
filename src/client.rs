use reqwest::StatusCode;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::time::Duration;

static DEFAULT_API_URL: &str = "https://api.peopledatalabs.com/";
static DEFAULT_API_VERSION: &str = "v5";
static DEFAULT_SANDBOX_URL: &str = "https://sandbox.api.peopledatalabs.com/";
static DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub enum PDLError {
    NetworkError(reqwest::Error),
    HTTPError(StatusCode),
    ValidationError,
}

impl Display for PDLError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            PDLError::NetworkError(ref e) => e.fmt(f),
            PDLError::HTTPError(ref s) => write!(f, "Invalid HTTP status code: {}", s),
            PDLError::ValidationError => f.write_str("Invalid Parameters"),
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
    client: reqwest::blocking::Client,
}

impl PDLClient {
    /// Make a new People Data Labs client with users API Key and API Version.
    pub fn new(key: &str) -> PDLClient {
        // Sets the default PDLClient
        use reqwest::blocking as rq;

        let builder = rq::ClientBuilder::new();
        let client = builder.timeout(DEFAULT_TIMEOUT).build().unwrap();

        PDLClient {
            api_key: key.to_string(),
            base_url: DEFAULT_API_URL.to_string(),
            api_version: DEFAULT_API_VERSION.to_string(),
            client,
        }
    }

    /// Adds the ability to update the version from the default through chaining.
    pub fn version(mut self, version: &str) -> PDLClient {
        self.api_version = version.to_string();
        self
    }

    /// Adds the ability to update the default timeout or access sandbox mode
    /// through chaining.
    pub fn options(mut self, options: PDLCLientOptions) -> PDLClient {
        if options.timeout != DEFAULT_TIMEOUT {
            use reqwest::blocking as rq;

            let builder = rq::ClientBuilder::new();
            let client = builder.timeout(options.timeout).build().unwrap();
            self.client = client
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
    pub fn get<T>(&self, endpoint: &str, params: &str) -> Result<T, PDLError>
    where
        T: serde::de::DeserializeOwned,
    {
        let uri = format!(
            "{}{}{}?api_key={}&{}",
            self.base_url, self.api_version, endpoint, self.api_key, params
        )
        .to_string();

        dbg!(&uri);

        let resp = self
            .client
            .get(uri)
            .send()
            .map_err(PDLError::NetworkError)
            .unwrap();

        match resp.status() {
            StatusCode::OK => {}
            StatusCode::NOT_FOUND => return Err(PDLError::HTTPError(StatusCode::NOT_FOUND)),
            other => return Err(PDLError::HTTPError(other)),
        }

        let r = resp.json::<T>().unwrap();

        Ok(r)
    }

    /// Sends a POST method through the PeopleDataLabs API. It takes an endpoint &str and params &str.
    /// It returns a generic response or PDLError.
    pub fn post<T>(&self, endpoint: &str, json: serde_json::Value) -> Result<T, PDLError>
    where
        T: serde::de::DeserializeOwned + std::fmt::Debug,
    {
        let uri = format!(
            "{}{}{}?api_key={}",
            self.base_url, self.api_version, endpoint, self.api_key
        )
        .to_string();

        dbg!(&uri);
        dbg!(&json);

        let resp = self
            .client
            .post(uri)
            .json(&json)
            .send()
            .map_err(PDLError::NetworkError)
            .unwrap();

        match resp.status() {
            StatusCode::OK => {}
            other => return Err(PDLError::HTTPError(other)),
        }

        let r = resp.json::<T>().unwrap();

        Ok(r)
    }
}
