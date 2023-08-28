use serde::{Deserialize, Serialize};

use crate::{
    models::common::{AdditionalParams, BaseParams},
    PDLError,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationParams {
    /// The raw location to process
    #[serde(rename = "location", default)]
    pub location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleanLocationParams {
    #[serde(flatten)]
    pub base_params: Option<BaseParams>,

    #[serde(flatten)]
    pub location_params: LocationParams,

    #[serde(flatten)]
    pub additional_params: Option<AdditionalParams>,
}

impl CleanLocationParams {
    pub fn validate(&self) -> Result<(), PDLError> {
        if self.location_params.location.is_none() {
            return Err(PDLError::ValidationError);
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleanLocationResponse {
    status: i32,
    location: String,
}
