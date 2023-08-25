use std::collections::HashMap;

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
    pub base_params: Option<BaseParams>,

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
pub struct SearchCompanyResponse {
    status: i32,
    data: Vec<CompanyResponse>,
    /// Scroll value used for pagination
    scroll_token: Option<String>,
    /// Number of records matching a given query or sql input.
    total: Option<i32>,
    /// Error details
    error: Option<CompanyError>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CompanyError {
    #[serde(rename = "type")]
    error_type: Vec<String>,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Naics {
    naics_code: Option<String>,
    sector: Option<String>,
    sub_sector: Option<String>,
    industry_group: Option<String>,
    naics_industry: Option<String>,
    national_industry: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Sic {
    sic_code: Option<String>,
    major_group: Option<String>,
    industry_group: Option<String>,
    industry_sector: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Location {
    name: Option<String>,
    locality: Option<String>,
    region: Option<String>,
    metro: Option<String>,
    country: Option<String>,
    continent: Option<String>,
    street_address: Option<String>,
    address_line_2: Option<String>,
    postal_code: Option<String>,
    geo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyResponse {
    status: Option<i32>,
    name: Option<String>,
    size: Option<String>,
    employee_count: Option<i32>,
    id: Option<String>,
    founded: Option<i32>,
    industry: Option<String>,
    naics: Option<Vec<Naics>>,
    sic: Option<Vec<Sic>>,
    location: Option<Location>,
    linkedin_id: Option<String>,
    linkedin_url: Option<String>,
    facebook_url: Option<String>,
    twitter_url: Option<String>,
    profiles: Option<Vec<String>>,
    website: Option<String>,
    ticker: Option<String>,
    gics_sector: Option<String>,
    mic_exchange: Option<String>,
    type_: Option<String>,
    summary: Option<String>,
    tags: Option<Vec<String>>,
    headline: Option<String>,
    alternative_names: Option<Vec<String>>,
    alternative_domains: Option<Vec<String>>,
    affiliated_profiles: Option<Vec<String>>,
    employee_count_by_country: Option<HashMap<String, i32>>,
    likelihood: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleanCompanyResponse {
    status: i32,
    name: Option<String>,
    size: Option<String>,
    id: Option<String>,
    founded: Option<i32>,
    industry: Option<String>,
    location: Option<Location>,
    linkedin_url: Option<String>,
    linkedin_id: Option<String>,
    facebook_url: Option<String>,
    twitter_url: Option<String>,
    website: Option<String>,
    ticker: Option<String>,
    type_: Option<String>,
    raw: Option<Vec<String>>,
    score: f32,
    fuzzy_match: bool,
}
