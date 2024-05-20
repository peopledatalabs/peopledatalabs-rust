use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    models::common::{AdditionalParams, BaseParams},
    PDLError,
};

#[derive(Debug, Serialize, Deserialize, Default)]
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

impl CompanyParams {
    // Validation function
    pub fn validate(&self) -> Result<(), PDLError> {
        // Check if at least one field is present
        if self.pdl_id.is_some()
            || self.name.is_some()
            || self.website.is_some()
            || self.profile.is_some()
            || self.ticker.is_some()
            || self.location.is_some()
            || self.locality.is_some()
            || self.region.is_some()
            || self.country.is_some()
            || self.street_address.is_some()
            || self.postal_code.is_some()
        {
            return Ok(());
        }

        Err(PDLError::ValidationError)
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

#[derive(Debug, Deserialize, Serialize)]
pub struct BulkEnrichCompanyParams {
    pub requests: Vec<BulkEnrichSingleCompanyParams>,
}

impl BulkEnrichCompanyParams {
    pub fn validate(&self) -> Result<(), PDLError> {
        for request in &self.requests {
            request.validate()?
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BulkEnrichSingleCompanyParams {
    pub params: CompanyParams,
}

impl BulkEnrichSingleCompanyParams {
    pub fn validate(&self) -> Result<(), PDLError> {
        self.params.validate()?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
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
pub struct TopUsEmployeeMetros {
    pub current_head_count: Option<i32>,
    pub twelve_moth_growth_rate: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentExecHires {
    pub joined_date: Option<String>,
    pub pdl_id: Option<String>,
    pub job_title: Option<String>,
    pub job_title_role: Option<String>,
    pub job_title_sub_role: Option<String>,
    pub job_title_levels: Option<Vec<String>>,
    pub previous_company_id: Option<String>,
    pub previous_company_job_title: Option<String>,
    pub previous_company_job_title_role: Option<String>,
    pub previous_company_job_title_sub_role: Option<String>,
    pub previous_company_job_title_levels: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentExecDepartures {
    pub departed_date: Option<String>,
    pub pdl_id: Option<String>,
    pub job_title: Option<String>,
    pub job_title_role: Option<String>,
    pub job_title_sub_role: Option<String>,
    pub job_title_levels: Option<Vec<String>>,
    pub new_company_id: Option<String>,
    pub new_company_job_title: Option<String>,
    pub new_company_job_title_role: Option<String>,
    pub new_company_job_title_sub_role: Option<String>,
    pub new_company_job_title_levels: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FundingDetails {
    pub funding_round_date: Option<String>,
    pub funding_raised: Option<f64>,
    pub funding_currency: Option<String>,
    pub funding_type: Option<String>,
    pub investing_companies: Option<Vec<String>>,
    pub investing_individuals: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyResponse {
    /// See https://docs.peopledatalabs.com/docs/example-company-record for more information.
    pub status: Option<i32>,
    pub dataset_version: Option<String>,
    pub display_name: Option<String>,
    pub name: Option<String>,
    pub size: Option<String>,
    pub employee_count: Option<i32>,
    pub linkedin_employee_count: Option<i32>,
    pub id: Option<String>,
    pub founded: Option<i32>,
    pub industry: Option<String>,
    pub linkedin_id: Option<String>,
    pub linkedin_url: Option<String>,
    pub linkedin_slug: Option<String>,
    pub facebook_url: Option<String>,
    pub twitter_url: Option<String>,
    pub profiles: Option<Vec<String>>,
    pub website: Option<String>,
    pub ticker: Option<String>,
    pub type_: Option<String>,
    pub summary: Option<String>,
    pub tags: Option<Vec<String>>,
    pub headline: Option<String>,
    pub display_name_history: Option<Vec<String>>,
    pub alternative_names: Option<Vec<String>>,
    pub alternative_domains: Option<Vec<String>>,
    pub affiliated_profiles: Option<Vec<String>>,
    pub location: Option<Location>,
    pub naics: Option<Vec<Naics>>,
    pub sic: Option<Vec<Sic>>,
    pub employee_growth_rate: Option<HashMap<String, f32>>,
    pub employee_churn_rate: Option<HashMap<String, f32>>,
    pub average_employee_tenure: Option<f32>,
    pub average_tenure_by_role: Option<HashMap<String, f32>>,
    pub average_tenure_by_level: Option<HashMap<String, f32>>,
    pub employee_count_by_country: Option<HashMap<String, i32>>,
    pub top_us_employee_metro: Option<Vec<TopUsEmployeeMetros>>,
    pub employee_count_by_month: Option<HashMap<String, i32>>,
    pub gross_additions_by_month: Option<HashMap<String, i32>>,
    pub gross_departures_by_month: Option<HashMap<String, i32>>,
    pub employee_count_by_month_by_role: Option<HashMap<String, HashMap<String, i32>>>,
    pub employee_count_by_month_by_level: Option<HashMap<String, HashMap<String, i32>>>,
    pub recent_exec_hires: Option<Vec<RecentExecHires>>,
    pub recent_exec_departures: Option<Vec<RecentExecDepartures>>,
    pub top_previous_employers_by_role: Option<HashMap<String, HashMap<String, i32>>>,
    pub top_next_employers_by_role: Option<HashMap<String, HashMap<String, i32>>>,
    pub total_funding_raised: Option<f32>,
    pub latest_funding_stage: Option<String>,
    pub latest_funding_date: Option<String>,
    pub number_funding_rounds: Option<i32>,
    pub funding_stages: Option<Vec<String>>,
    pub funding_details: Option<Vec<FundingDetails>>,
    pub likelihood: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BulkCompanyEnrichResponse {
    pub data: Option<Vec<CompanyResponse>>,
    pub status: i32,
    pub likelihood: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleanCompanyResponse {
    /// See https://docs.peopledatalabs.com/docs/output-response-cleaner-apis#company-cleaner-api-response for more information.
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
