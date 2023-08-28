use serde::{Deserialize, Serialize};

use crate::{models::common::BaseParams, PDLError};

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillBaseParams {
    /// Skill that is used as the seed for enrichment
    #[serde(rename = "skill", default)]
    pub skill: Option<String>,
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
    status: i32,
    data: SkillResult,
}

#[derive(Debug, Serialize, Deserialize)]
struct SkillResult {
    cleaned_skill: String,
    similar_skills: Vec<String>,
    relevant_job_titles: Vec<String>,
}
