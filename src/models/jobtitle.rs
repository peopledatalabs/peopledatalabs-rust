use serde::{Deserialize, Serialize};

use crate::{models::common::BaseParams, PDLError};

#[derive(Debug, Serialize, Deserialize)]
pub struct JobTitleBaseParams {
    /// JobTitle that is used as the seed for enrichment
    #[serde(rename = "job_title", default)]
    pub job_title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobTitleParams {
    #[serde(flatten)]
    pub base_params: BaseParams,

    #[serde(flatten)]
    pub job_title_base_params: JobTitleBaseParams,
}

impl JobTitleParams {
    pub fn validate(&self) -> Result<(), PDLError> {
        if self.job_title_base_params.job_title.is_none() {
            return Err(PDLError::ValidationError);
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobTitleResponse {
    status: i32,
    data: JobTitleResult,
}

#[derive(Debug, Serialize, Deserialize)]
struct JobTitleResult {
    cleaned_job_title: String,
    similar_job_titles: Vec<String>,
    relevant_skills: Vec<String>,
}
