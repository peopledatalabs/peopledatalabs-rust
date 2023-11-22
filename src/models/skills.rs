use serde::{Deserialize, Serialize};

use crate::{models::common::BaseParams, PDLError};

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillBaseParams {
    /// Skill that is used as the seed for enrichment
    #[serde(rename = "skill", default)]
    pub skill: Option<String>,
    /// Setting titlecase to true will titlecase the data in 200 responses
    #[serde(rename = "titlecase", skip_serializing_if = "Option::is_none")]
    pub titlecase: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillParams {
    #[serde(flatten)]
    pub base_params: Option<BaseParams>,

    #[serde(flatten)]
    pub skill_base_params: SkillBaseParams,
}

impl SkillParams {
    pub fn validate(&self) -> Result<(), PDLError> {
        if self.skill_base_params.skill.is_none() {
            return Err(PDLError::ValidationError);
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillResponse {
    pub status: i32,
    pub data: SkillResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillResult {
    pub cleaned_skill: String,
    pub similar_skills: Vec<String>,
    pub relevant_job_titles: Vec<String>,
}
