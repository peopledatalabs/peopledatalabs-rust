use reqwest::blocking::Body;
use reqwest::StatusCode;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

static API_URL: &str = "https://api.peopledatalabs.com/";

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

/// This is the struct that creates the connection with PeopleDataLabs.
/// It contain   s the API Key and a client that can be used both sync and async.
#[derive(Clone)]
pub struct PDLClient {
    api_key: String,
    url: String,
    api_version: String,
    client: reqwest::blocking::Client,
}

impl PDLClient {
    /// Make a new People Data Labs client with users API Key and API Version.
    pub fn new(key: &str, version: &str) -> PDLClient {
        use reqwest::blocking as rq;

        let builder = rq::ClientBuilder::new();
        let client = builder.build().unwrap();

        PDLClient {
            api_key: key.to_string(),
            url: API_URL.to_string(),
            api_version: version.to_string(),
            client,
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
            self.url, self.api_version, endpoint, self.api_key, params
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
    pub fn post<T>(&self, endpoint: &str, params: &str, body: Body) -> Result<T, PDLError>
    where
        T: serde::de::DeserializeOwned,
    {
        let uri = format!(
            "{}{}{}?api_key={}&{}",
            self.url, self.api_version, endpoint, self.api_key, params
        )
        .to_string();

        let resp = self
            .client
            .post(uri)
            .body(body)
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
