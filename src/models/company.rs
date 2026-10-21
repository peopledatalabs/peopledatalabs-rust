use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    models::common::{AdditionalParams, BaseParams},
    PDLError,
};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CompanyParams {
    /// The PDL ID of the company
    #[serde(rename = "pdl_id", skip_serializing_if = "Option::is_none", default)]
    pub pdl_id: Option<String>,
    /// The name of the company
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// A website the company uses
    #[serde(rename = "website", skip_serializing_if = "Option::is_none", default)]
    pub website: Option<String>,
    /// A social profile of the company (linkedin/facebook/twitter/crunchbase)
    #[serde(rename = "profile", skip_serializing_if = "Option::is_none", default)]
    pub profile: Option<String>,
    /// Company stock ticker
    #[serde(rename = "ticker", skip_serializing_if = "Option::is_none", default)]
    pub ticker: Option<String>,
    /// Complete or partial company location
    #[serde(rename = "location", skip_serializing_if = "Option::is_none", default)]
    pub location: Option<Vec<String>>,
    /// Company locality
    #[serde(rename = "locality", skip_serializing_if = "Option::is_none", default)]
    pub locality: Option<String>,
    /// Company region
    #[serde(rename = "region", skip_serializing_if = "Option::is_none", default)]
    pub region: Option<String>,
    /// Company country
    #[serde(rename = "country", skip_serializing_if = "Option::is_none", default)]
    pub country: Option<String>,
    /// Company address
    #[serde(
        rename = "street_address",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub street_address: Option<String>,
    /// Company postal code
    #[serde(
        rename = "postal_code",
        skip_serializing_if = "Option::is_none",
        default
    )]
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
    #[serde(
        rename = "type",
        deserialize_with = "crate::models::common::deserialize_string_or_vec"
    )]
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
pub struct CompanyLocation {
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
    pub is_primary: Option<bool>,
    pub is_active: Option<bool>,
    pub first_seen: Option<String>,
    pub last_seen: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopUsEmployeeMetros {
    pub current_headcount: Option<i32>,
    #[serde(rename = "12_month_growth_rate")]
    pub twelve_month_growth_rate: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopEmployerEntry {
    pub id: Option<String>,
    pub display_name: Option<String>,
    pub count: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentExecHires {
    pub joined_date: Option<String>,
    pub pdl_id: Option<String>,
    pub name: Option<String>,
    pub linkedin_url: Option<String>,
    pub job_title: Option<String>,
    pub job_title_role: Option<String>,
    pub job_title_sub_role: Option<String>,
    pub job_title_class: Option<String>,
    pub job_title_levels: Option<Vec<String>>,
    pub previous_company_id: Option<String>,
    pub previous_company_display_name: Option<String>,
    pub previous_company_job_title: Option<String>,
    pub previous_company_job_title_role: Option<String>,
    pub previous_company_job_title_sub_role: Option<String>,
    pub previous_company_job_title_class: Option<String>,
    pub previous_company_job_title_levels: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentExecDepartures {
    pub departed_date: Option<String>,
    pub pdl_id: Option<String>,
    pub name: Option<String>,
    pub linkedin_url: Option<String>,
    pub job_title: Option<String>,
    pub job_title_role: Option<String>,
    pub job_title_sub_role: Option<String>,
    pub job_title_class: Option<String>,
    pub job_title_levels: Option<Vec<String>>,
    pub new_company_id: Option<String>,
    pub new_company_display_name: Option<String>,
    pub new_company_job_title: Option<String>,
    pub new_company_job_title_role: Option<String>,
    pub new_company_job_title_sub_role: Option<String>,
    pub new_company_job_title_class: Option<String>,
    pub new_company_job_title_levels: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentExecInternalChanges {
    pub new_role_date: Option<String>,
    pub pdl_id: Option<String>,
    pub name: Option<String>,
    pub linkedin_url: Option<String>,
    pub job_title: Option<String>,
    pub job_title_role: Option<String>,
    pub job_title_sub_role: Option<String>,
    pub job_title_class: Option<String>,
    pub job_title_levels: Option<Vec<String>>,
    pub previous_job_title: Option<String>,
    pub previous_job_title_role: Option<String>,
    pub previous_job_title_sub_role: Option<String>,
    pub previous_job_title_class: Option<String>,
    pub previous_job_title_levels: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvestingCompaniesDetail {
    pub investing_company_id: Option<String>,
    pub investing_company_display_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FundingDetails {
    pub funding_round_date: Option<String>,
    pub funding_raised: Option<f64>,
    pub funding_currency: Option<String>,
    pub funding_type: Option<String>,
    #[deprecated(
        since = "5.3.0",
        note = "removal on 2027-02-16 (PDL API v37.1); use `investing_companies_details` instead"
    )]
    pub investing_companies: Option<Vec<String>>,
    pub investing_companies_details: Option<Vec<InvestingCompaniesDetail>>,
    pub investing_individuals: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AffiliatedEntity {
    pub affiliated_id: Option<String>,
    pub relationship: Option<String>,
    pub display_name: Option<String>,
    pub relationship_catalyst: Option<String>,
    pub relationship_status: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub relationship_citations: Option<Vec<String>>,
    pub employee_count: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeGrowthRate12MonthByCountry {
    pub current_headcount: Option<i32>,
    #[serde(rename = "12_month_headcount")]
    pub twelve_month_headcount: Option<i32>,
    #[serde(rename = "12_month_growth_rate")]
    pub twelve_month_growth_rate: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TechnologiesUsed {
    pub product_id: Option<String>,
    pub name: Option<String>,
    pub num_job_posting_mentions: Option<i32>,
    pub job_posting_first_observed: Option<String>,
    pub job_posting_last_observed: Option<String>,
    pub num_resume_mentions: Option<i32>,
    pub resume_first_observed: Option<String>,
    pub resume_last_observed: Option<String>,
    pub inferred_first_used: Option<String>,
    pub inferred_last_used: Option<String>,
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
    pub linkedin_follower_count: Option<i32>,
    pub id: Option<String>,
    pub founded: Option<i32>,
    pub industry: Option<String>,
    pub industry_v2: Option<String>,
    pub gics_sector: Option<String>,
    pub inferred_revenue: Option<String>,
    pub inferred_revenue_integer: Option<i64>,
    pub linkedin_id: Option<String>,
    pub linkedin_url: Option<String>,
    pub linkedin_slug: Option<String>,
    pub facebook_url: Option<String>,
    pub twitter_url: Option<String>,
    pub profiles: Option<Vec<String>>,
    pub website: Option<String>,
    pub ticker: Option<String>,
    pub mic_exchange: Option<String>,
    pub ultimate_parent: Option<String>,
    pub immediate_parent: Option<String>,
    pub ultimate_parent_ticker: Option<String>,
    pub ultimate_parent_mic_exchange: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub summary: Option<String>,
    pub tags: Option<Vec<String>>,
    pub headline: Option<String>,
    pub display_name_history: Option<Vec<String>>,
    pub alternative_names: Option<Vec<String>>,
    pub alternative_domains: Option<Vec<String>>,
    #[deprecated(
        since = "5.3.0",
        note = "removal on 2027-02-16 (PDL API v37.1); use `affiliated_entities[].affiliated_id` instead"
    )]
    pub affiliated_profiles: Option<Vec<String>>,
    pub all_subsidiaries: Option<Vec<String>>,
    pub direct_subsidiaries: Option<Vec<String>>,
    pub affiliated_entities: Option<Vec<AffiliatedEntity>>,
    pub location: Option<Location>,
    pub locations: Option<Vec<CompanyLocation>>,
    pub num_total_locations: Option<i32>,
    pub num_active_locations: Option<i32>,
    pub naics: Option<Vec<Naics>>,
    pub sic: Option<Vec<Sic>>,
    #[serde(
        default,
        deserialize_with = "crate::models::common::deserialize_opt_map_skip_nulls"
    )]
    pub employee_growth_rate: Option<HashMap<String, f32>>,
    #[serde(
        default,
        deserialize_with = "crate::models::common::deserialize_opt_map_skip_nulls"
    )]
    pub employee_churn_rate: Option<HashMap<String, f32>>,
    #[serde(
        default,
        deserialize_with = "crate::models::common::deserialize_opt_map_skip_nulls"
    )]
    pub employee_turnover_rate: Option<HashMap<String, f32>>,
    pub annualized_employee_turnover_rate: Option<Value>,
    pub average_employee_tenure: Option<f32>,
    pub median_employee_tenure: Option<f32>,
    #[serde(
        default,
        deserialize_with = "crate::models::common::deserialize_opt_map_skip_nulls"
    )]
    pub average_tenure_by_role: Option<HashMap<String, f32>>,
    #[serde(
        default,
        deserialize_with = "crate::models::common::deserialize_opt_map_skip_nulls"
    )]
    pub median_tenure_by_role: Option<HashMap<String, f32>>,
    #[serde(
        default,
        deserialize_with = "crate::models::common::deserialize_opt_map_skip_nulls"
    )]
    pub average_tenure_by_level: Option<HashMap<String, f32>>,
    #[serde(
        default,
        deserialize_with = "crate::models::common::deserialize_opt_map_skip_nulls"
    )]
    pub median_tenure_by_level: Option<HashMap<String, f32>>,
    #[serde(
        default,
        deserialize_with = "crate::models::common::deserialize_opt_map_skip_nulls"
    )]
    pub employee_count_by_country: Option<HashMap<String, i32>>,
    #[serde(
        default,
        deserialize_with = "crate::models::common::deserialize_opt_map_skip_nulls"
    )]
    pub employee_count_by_role: Option<HashMap<String, i32>>,
    #[serde(
        default,
        deserialize_with = "crate::models::common::deserialize_opt_map_skip_nulls"
    )]
    pub employee_growth_rate_12_month_by_role: Option<HashMap<String, f32>>,
    pub employee_growth_rate_12_month_by_country:
        Option<HashMap<String, EmployeeGrowthRate12MonthByCountry>>,
    pub top_us_employee_metros: Option<HashMap<String, TopUsEmployeeMetros>>,
    #[serde(
        default,
        deserialize_with = "crate::models::common::deserialize_opt_map_skip_nulls"
    )]
    pub employee_count_by_month: Option<HashMap<String, i32>>,
    #[serde(
        default,
        deserialize_with = "crate::models::common::deserialize_opt_map_skip_nulls"
    )]
    pub gross_additions_by_month: Option<HashMap<String, i32>>,
    #[serde(
        default,
        deserialize_with = "crate::models::common::deserialize_opt_map_skip_nulls"
    )]
    pub gross_departures_by_month: Option<HashMap<String, i32>>,
    #[serde(
        default,
        deserialize_with = "crate::models::common::deserialize_opt_nested_map_skip_nulls"
    )]
    pub employee_count_by_month_by_role: Option<HashMap<String, HashMap<String, i32>>>,
    #[serde(
        default,
        deserialize_with = "crate::models::common::deserialize_opt_nested_map_skip_nulls"
    )]
    pub employee_count_by_month_by_level: Option<HashMap<String, HashMap<String, i32>>>,
    pub active_job_postings: Option<Value>,
    #[serde(
        default,
        deserialize_with = "crate::models::common::deserialize_opt_map_skip_nulls"
    )]
    pub active_job_postings_by_role: Option<HashMap<String, i32>>,
    #[serde(
        default,
        deserialize_with = "crate::models::common::deserialize_opt_map_skip_nulls"
    )]
    pub active_job_postings_by_class: Option<HashMap<String, i32>>,
    #[serde(
        default,
        deserialize_with = "crate::models::common::deserialize_opt_map_skip_nulls"
    )]
    pub active_job_postings_by_sub_role: Option<HashMap<String, i32>>,
    #[serde(
        default,
        deserialize_with = "crate::models::common::deserialize_opt_map_skip_nulls"
    )]
    pub active_job_postings_by_country: Option<HashMap<String, i32>>,
    #[serde(
        default,
        deserialize_with = "crate::models::common::deserialize_opt_map_skip_nulls"
    )]
    pub active_job_postings_by_metro: Option<HashMap<String, i32>>,
    #[serde(
        default,
        deserialize_with = "crate::models::common::deserialize_opt_map_skip_nulls"
    )]
    pub active_job_postings_by_month: Option<HashMap<String, i32>>,
    pub deactivated_job_postings: Option<Value>,
    #[serde(
        default,
        deserialize_with = "crate::models::common::deserialize_opt_map_skip_nulls"
    )]
    pub deactivated_job_postings_by_role: Option<HashMap<String, i32>>,
    #[serde(
        default,
        deserialize_with = "crate::models::common::deserialize_opt_map_skip_nulls"
    )]
    pub deactivated_job_postings_by_class: Option<HashMap<String, i32>>,
    #[serde(
        default,
        deserialize_with = "crate::models::common::deserialize_opt_map_skip_nulls"
    )]
    pub deactivated_job_postings_by_sub_role: Option<HashMap<String, i32>>,
    #[serde(
        default,
        deserialize_with = "crate::models::common::deserialize_opt_map_skip_nulls"
    )]
    pub deactivated_job_postings_by_month: Option<HashMap<String, i32>>,
    pub recent_exec_hires: Option<Vec<RecentExecHires>>,
    pub recent_exec_departures: Option<Vec<RecentExecDepartures>>,
    pub recent_exec_internal_changes: Option<Vec<RecentExecInternalChanges>>,
    pub top_previous_employers: Option<HashMap<String, Vec<TopEmployerEntry>>>,
    pub top_next_employers: Option<HashMap<String, Vec<TopEmployerEntry>>>,
    pub total_funding_raised: Option<f32>,
    pub latest_funding_stage: Option<String>,
    pub last_funding_date: Option<String>,
    pub number_funding_rounds: Option<i32>,
    pub funding_stages: Option<Vec<String>>,
    pub funding_details: Option<Vec<FundingDetails>>,
    pub likelihood: Option<i32>,
    pub technologies_used: Option<Vec<TechnologiesUsed>>,
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
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub raw: Option<Vec<String>>,
    pub score: f32,
    pub fuzzy_match: bool,
}

#[cfg(test)]
mod tests {
    use super::CompanyResponse;

    #[test]
    fn test_company_response_skips_null_map_entries() {
        let resp: CompanyResponse = serde_json::from_value(serde_json::json!({
            "employee_growth_rate_12_month_by_role": {"engineering": 0.12, "trade": null},
            "employee_count_by_role": {"sales": 10, "legal": null},
            "employee_count_by_month_by_role": {"2026-09": {"sales": 3, "legal": null}, "2026-10": null},
            "recent_exec_internal_changes": [{
                "new_role_date": "2026-08",
                "name": "gail cohen",
                "job_title_levels": ["vp"],
                "previous_job_title_levels": ["director"]
            }]
        }))
        .unwrap();

        let by_role = resp.employee_growth_rate_12_month_by_role.unwrap();
        assert_eq!(by_role.len(), 1);
        assert_eq!(by_role["engineering"], 0.12);
        assert_eq!(resp.employee_count_by_role.unwrap().len(), 1);
        let by_month = resp.employee_count_by_month_by_role.unwrap();
        assert_eq!(by_month.len(), 1);
        assert_eq!(by_month["2026-09"].len(), 1);
        assert!(resp.employee_growth_rate.is_none());
        let internal = resp.recent_exec_internal_changes.unwrap();
        assert_eq!(internal[0].name.as_deref(), Some("gail cohen"));
    }
}
