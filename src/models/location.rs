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
    pub status: i32,
    pub name: Option<String>,
    pub locality: Option<String>,
    pub region: Option<String>,
    pub metro: Option<String>,
    pub subregion: Option<String>,
    pub country: Option<String>,
    pub continent: Option<String>,
    pub type_: Option<String>,
    pub geo: Option<String>,

}
