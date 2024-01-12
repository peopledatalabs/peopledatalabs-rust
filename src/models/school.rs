use serde::{Deserialize, Serialize};

use crate::{
    models::common::{AdditionalParams, BaseParams},
    PDLError,
};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SchoolParams {
    #[serde(rename = "name", default)]
    pub name: Option<String>,

    #[serde(rename = "website", default)]
    pub website: Option<String>,

    #[serde(rename = "profile", default)]
    pub profile: Option<String>,
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
pub struct Location {
    pub name: Option<String>,
    pub locality: Option<String>,
    pub region: Option<String>,
    pub country: Option<String>,
    pub continent: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleanSchoolResponse {
    /// See https://docs.peopledatalabs.com/docs/output-response-cleaner-apis#school-cleaner-api-response for more information
    pub status: i32,
    pub name: Option<String>,
    pub type_: Option<String>,
    pub id: Option<String>,
    pub location: Option<Location>,
    pub linkedin_url: Option<String>,
    pub facebook_url: Option<String>,
    pub twitter_url: Option<String>,
    pub linkedin_id: Option<String>,
    pub website: Option<String>,
    pub domain: Option<String>,
}
