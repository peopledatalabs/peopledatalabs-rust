use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    models::common::{AdditionalParams, BaseParams},
    PDLError,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyParams {
    /// The PDL ID of the company
    #[serde(rename = "pdl_id", default)]
    pub pdl_id: Option<String>,
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
            pdl_id: None,
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
        if self.company_params.pdl_id.is_none()
            && self.company_params.name.is_none()
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
    pub status: i32,
    pub data: Vec<CompanyResponse>,
    /// Scroll value used for pagination
    pub scroll_token: Option<String>,
    /// Number of records matching a given query or sql input.
    pub total: Option<i32>,
    /// Error details
    pub error: Option<CompanyError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyError {
    #[serde(rename = "type")]
    pub error_type: Vec<String>,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Naics {
    pub naics_code: Option<String>,
    pub sector: Option<String>,
    pub sub_sector: Option<String>,
    pub industry_group: Option<String>,
    pub naics_industry: Option<String>,
    pub national_industry: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sic {
    pub sic_code: Option<String>,
    pub major_group: Option<String>,
    pub industry_group: Option<String>,
    pub industry_sector: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub name: Option<String>,
    pub locality: Option<String>,
    pub region: Option<String>,
    pub metro: Option<String>,
    pub country: Option<String>,
    pub continent: Option<String>,
    pub street_address: Option<String>,
    pub address_line_2: Option<String>,
    pub postal_code: Option<String>,
    pub geo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyResponse {
    pub status: Option<i32>,
    pub name: Option<String>,
    pub size: Option<String>,
    pub employee_count: Option<i32>,
    pub id: Option<String>,
    pub founded: Option<i32>,
    pub industry: Option<String>,
    pub naics: Option<Vec<Naics>>,
    pub sic: Option<Vec<Sic>>,
    pub location: Option<Location>,
    pub linkedin_id: Option<String>,
    pub linkedin_url: Option<String>,
    pub facebook_url: Option<String>,
    pub twitter_url: Option<String>,
    pub profiles: Option<Vec<String>>,
    pub website: Option<String>,
    pub ticker: Option<String>,
    pub gics_sector: Option<String>,
    pub mic_exchange: Option<String>,
    pub type_: Option<String>,
    pub summary: Option<String>,
    pub tags: Option<Vec<String>>,
    pub headline: Option<String>,
    pub alternative_names: Option<Vec<String>>,
    pub alternative_domains: Option<Vec<String>>,
    pub affiliated_profiles: Option<Vec<String>>,
    pub employee_count_by_country: Option<HashMap<String, i32>>,
    pub likelihood: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleanCompanyResponse {
    pub status: i32,
    pub name: Option<String>,
    pub size: Option<String>,
    pub id: Option<String>,
    pub founded: Option<i32>,
    pub industry: Option<String>,
    pub location: Option<Location>,
    pub linkedin_url: Option<String>,
    pub linkedin_id: Option<String>,
    pub facebook_url: Option<String>,
    pub twitter_url: Option<String>,
    pub website: Option<String>,
    pub ticker: Option<String>,
    pub type_: Option<String>,
    pub raw: Option<Vec<String>>,
    pub score: f32,
    pub fuzzy_match: bool,
}
