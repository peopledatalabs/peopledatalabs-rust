use serde::{Deserialize, Serialize};

use crate::{
    models::common::{AdditionalParams, BaseParams},
    PDLError,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SchoolParams {
    #[serde(rename = "name", default)]
    pub name: Option<String>,

    #[serde(rename = "website", default)]
    pub website: Option<String>,

    #[serde(rename = "profile", default)]
    pub profile: Option<String>,
}

impl Default for SchoolParams {
    fn default() -> Self {
        Self {
            name: None,
            website: None,
            profile: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleanSchoolParams {
    #[serde(flatten)]
    pub base_params: Option<BaseParams>,

    #[serde(flatten)]
    pub school_params: SchoolParams,

    #[serde(flatten)]
    pub additional_params: Option<AdditionalParams>,
}

impl CleanSchoolParams {
    pub fn validate(&self) -> Result<(), PDLError> {
        if self.school_params.name.is_none()
            && self.school_params.website.is_none()
            && self.school_params.profile.is_none()
        {
            return Err(PDLError::ValidationError);
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleanSchoolResponse {
    status: i32,
    school: School,
}

#[derive(Debug, Serialize, Deserialize)]
struct School {
    #[serde(rename = "name", default)]
    name: Option<String>,

    #[serde(rename = "website", default)]
    website: Option<String>,

    #[serde(rename = "profile", default)]
    profile: Option<String>,
}
