use serde::{Deserialize, Serialize};

use crate::{models::common::BaseParams, PDLError};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemoteWorkPolicy {
    Remote,
    Onsite,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SalaryPeriod {
    Year,
    Month,
    Week,
    Day,
    Hour,
}

/// JobPostingSearchParams covers both the Elasticsearch query form (set
/// `query`) and the field-based parameter form (leave `query` unset and
/// populate any of the filter fields). The PDL API decides which form to
/// honor based on the body it receives.
///
/// `is_active` is `Option<bool>` so a `None` is omitted from the request
/// while an explicit `Some(false)` is still sent — it is an opt-in filter.
/// `scroll_token` is the opaque base64 cursor returned by the previous page;
/// pass it back unchanged.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct JobPostingSearchBaseParams {
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<serde_json::Value>,
    #[serde(rename = "scroll_token", skip_serializing_if = "Option::is_none")]
    pub scroll_token: Option<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "first_seen_min", skip_serializing_if = "Option::is_none")]
    pub first_seen_min: Option<String>,
    #[serde(rename = "first_seen_max", skip_serializing_if = "Option::is_none")]
    pub first_seen_max: Option<String>,
    #[serde(rename = "deactivated_date_min", skip_serializing_if = "Option::is_none")]
    pub deactivated_date_min: Option<String>,
    #[serde(rename = "deactivated_date_max", skip_serializing_if = "Option::is_none")]
    pub deactivated_date_max: Option<String>,

    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "title_class", skip_serializing_if = "Option::is_none")]
    pub title_class: Option<String>,
    #[serde(rename = "title_role", skip_serializing_if = "Option::is_none")]
    pub title_role: Option<String>,
    #[serde(rename = "title_sub_role", skip_serializing_if = "Option::is_none")]
    pub title_sub_role: Option<String>,
    #[serde(rename = "title_levels", skip_serializing_if = "Option::is_none")]
    pub title_levels: Option<String>,

    #[serde(rename = "company_id", skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[serde(rename = "company_name", skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(rename = "company_industry", skip_serializing_if = "Option::is_none")]
    pub company_industry: Option<String>,
    #[serde(rename = "company_industry_v2", skip_serializing_if = "Option::is_none")]
    pub company_industry_v2: Option<String>,
    #[serde(rename = "company_website", skip_serializing_if = "Option::is_none")]
    pub company_website: Option<String>,
    #[serde(rename = "company_profile", skip_serializing_if = "Option::is_none")]
    pub company_profile: Option<String>,

    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "salary_range_min", skip_serializing_if = "Option::is_none")]
    pub salary_range_min: Option<i32>,
    #[serde(rename = "salary_range_max", skip_serializing_if = "Option::is_none")]
    pub salary_range_max: Option<i32>,
    #[serde(rename = "salary_currency", skip_serializing_if = "Option::is_none")]
    pub salary_currency: Option<String>,
    #[serde(rename = "salary_period", skip_serializing_if = "Option::is_none")]
    pub salary_period: Option<SalaryPeriod>,

    #[serde(rename = "remote_work_policy", skip_serializing_if = "Option::is_none")]
    pub remote_work_policy: Option<RemoteWorkPolicy>,
    #[serde(rename = "inferred_skills", skip_serializing_if = "Option::is_none")]
    pub inferred_skills: Option<String>,
    #[serde(rename = "last_verified_min", skip_serializing_if = "Option::is_none")]
    pub last_verified_min: Option<String>,
    #[serde(rename = "last_verified_max", skip_serializing_if = "Option::is_none")]
    pub last_verified_max: Option<String>,

    #[serde(rename = "is_active", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct JobPostingSearchParams {
    #[serde(flatten)]
    pub base_params: Option<BaseParams>,
    #[serde(flatten)]
    pub search_base_params: JobPostingSearchBaseParams,
}

impl JobPostingSearchParams {
    pub fn validate(&self) -> Result<(), PDLError> {
        if let Some(base) = &self.base_params {
            if let Some(size) = base.size {
                if !(1..=100).contains(&size) {
                    return Err(PDLError::ValidationError);
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobPosting {
    pub id: Option<String>,
    pub title: Option<String>,
    pub title_class: Option<String>,
    pub title_role: Option<String>,
    pub title_sub_role: Option<String>,
    pub title_levels: Option<Vec<String>>,
    pub company_id: Option<String>,
    pub company_name: Option<String>,
    pub company_industry: Option<String>,
    pub company_industry_v2: Option<String>,
    pub company_website: Option<String>,
    pub company_profile: Option<String>,
    pub location: Option<String>,
    pub description: Option<String>,
    pub urls: Option<Vec<String>>,
    pub first_seen: Option<String>,
    pub last_verified: Option<String>,
    pub deactivated_date: Option<String>,
    pub is_active: Option<bool>,
    pub remote_work_policy: Option<String>,
    pub salary_range_min: Option<i32>,
    pub salary_range_max: Option<i32>,
    pub salary_currency: Option<String>,
    pub salary_period: Option<String>,
    pub inferred_skills: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchJobPostingResponse {
    pub status: i32,
    pub data: Option<Vec<JobPosting>>,
    pub total: Option<i32>,
    pub scroll_token: Option<String>,
}
