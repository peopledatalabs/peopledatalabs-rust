use serde::{Deserialize, Serialize};

use crate::{
    models::common::{AdditionalParams, BaseParams},
    PDLError,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyParams {
    /// The name of the company
    #[serde(rename = "name", default)]
    pub name: Option<String>,
    /// A website the company uses
    #[serde(rename = "website", default)]
    pub website: Option<String>,
    /// A social profile of the company (linkedin/facebook/twitter/crunchbase)
    #[serde(rename = "profile", default)]
    pub profile: Option<String>,
    /// Company stock ticker
    #[serde(rename = "ticker", default)]
    pub ticker: Option<String>,
    /// Complete or partial company location
    #[serde(rename = "location", default)]
    pub location: Option<Vec<String>>,
    /// Company locality
    #[serde(rename = "locality", default)]
    pub locality: Option<String>,
    /// Company region
    #[serde(rename = "region", default)]
    pub region: Option<String>,
    /// Company country
    #[serde(rename = "country", default)]
    pub country: Option<String>,
    /// Company address
    #[serde(rename = "street_address", default)]
    pub street_address: Option<String>,
    /// Company postal code
    #[serde(rename = "postal_code", default)]
    pub postal_code: Option<String>,
}

impl Default for CompanyParams {
    fn default() -> Self {
        Self {
            name: None,
            website: None,
            profile: None,
            ticker: None,
            location: None,
            locality: None,
            region: None,
            country: None,
            street_address: None,
            postal_code: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnrichCompanyParams {
    #[serde(flatten)]
    pub base_params: BaseParams,

    #[serde(flatten)]
    pub company_params: CompanyParams,

    #[serde(flatten)]
    pub additional_params: Option<AdditionalParams>,
}

impl EnrichCompanyParams {
    pub fn validate(&self) -> Result<(), PDLError> {
        if self.company_params.name.is_none()
            && self.company_params.ticker.is_none()
            && self.company_params.website.is_none()
            && self.company_params.profile.is_none()
        {
            return Err(PDLError::ValidationError);
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnrichCompanyResponse {
    status: i32,
    likelihood: i32,
    company: Company,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleanCompanyParams {
    /// The name of the company
    #[serde(rename = "name", default)]
    pub name: Option<String>,
    /// A website the company uses
    #[serde(rename = "website", default)]
    pub website: Option<String>,
    /// A social profile of the company (linkedin/facebook/twitter/crunchbase)
    #[serde(rename = "profile", default)]
    pub profile: Option<String>,
}

impl Default for CleanCompanyParams {
    fn default() -> Self {
        Self {
            name: None,
            website: None,
            profile: None,
        }
    }
}

impl CleanCompanyParams {
    pub fn validate(&self) -> Result<(), PDLError> {
        if self.name.is_none() && self.website.is_none() && self.profile.is_none() {
            return Err(PDLError::ValidationError);
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleanCompanyResponse {
    status: i32,
    company: Company,
    fuzzy_match: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchCompanyResponse {
    status: i32,
    data: Vec<Company>,
    /// Scroll value used for pagination
    scroll_token: String,
    /// Number of records matching a given query or sql input.
    total: i32,
    /// Error details
    error: CompanyError,
}

#[derive(Debug, Serialize, Deserialize)]
struct CompanyError {
    #[serde(rename = "type")]
    error_type: Vec<String>,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Company {
    /// The name of the company
    #[serde(rename = "name", default)]
    pub name: Option<String>,
    /// A website the company uses
    #[serde(rename = "website", default)]
    pub website: Option<String>,
    /// A social profile of the company (linkedin/facebook/twitter/crunchbase)
    #[serde(rename = "profile", default)]
    pub profile: Option<String>,
    /// Company stock ticker
    #[serde(rename = "ticker", default)]
    pub ticker: Option<String>,
    /// Complete or partial company location
    #[serde(rename = "location", default)]
    pub location: Option<Vec<String>>,
    /// Company locality
    #[serde(rename = "locality", default)]
    pub locality: Option<String>,
    /// Company region
    #[serde(rename = "region", default)]
    pub region: Option<String>,
    /// Company country
    #[serde(rename = "country", default)]
    pub country: Option<String>,
    /// Company address
    #[serde(rename = "street_address", default)]
    pub street_address: Option<String>,
    /// Company postal code
    #[serde(rename = "postal_code", default)]
    pub postal_code: Option<String>,
}
