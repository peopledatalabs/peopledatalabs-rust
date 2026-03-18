use serde::{Deserialize, Serialize};

use crate::{
    models::common::BaseParams,
    PDLError,
};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ChangelogPersonParams {
    #[serde(flatten)]
    pub base_params: Option<BaseParams>,

    /// The release version of the person data to compare against
    #[serde(rename = "origin_version", skip_serializing_if = "Option::is_none")]
    pub origin_version: Option<String>,

    /// The release version of the person data to compare with
    #[serde(rename = "current_version", skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,

    /// A list of person IDs to compare
    #[serde(rename = "ids", skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,

    /// The type of changes to include (e.g. "added", "updated", "deleted", "merged")
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    /// A list of fields to filter changes by
    #[serde(rename = "fields_updated", skip_serializing_if = "Option::is_none")]
    pub fields_updated: Option<Vec<String>>,

    /// An offset key for paginating between batches
    #[serde(rename = "scroll_token", skip_serializing_if = "Option::is_none")]
    pub scroll_token: Option<String>,
}

impl ChangelogPersonParams {
    pub fn validate(&self) -> Result<(), PDLError> {
        if self.origin_version.is_none() || self.current_version.is_none() {
            return Err(PDLError::ValidationError);
        }
        if self.ids.is_none() && self.type_.is_none() {
            return Err(PDLError::ValidationError);
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangelogPersonResponse {
    pub status: i32,
    pub error: Option<ChangelogError>,
    pub data: Option<ChangelogData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangelogError {
    #[serde(rename = "type")]
    pub type_: Option<Vec<String>>,
    pub message: Option<String>,
    pub valid_versions: Option<Vec<String>>,
    pub valid_types: Option<Vec<String>>,
    pub valid_fields_updated: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangelogData {
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub origin_version: Option<String>,
    pub current_version: Option<String>,
    pub scroll_token: Option<String>,
    pub updated: Option<Vec<ChangelogUpdated>>,
    pub added: Option<Vec<ChangelogRecord>>,
    pub deleted: Option<Vec<ChangelogRecord>>,
    pub merged: Option<Vec<ChangelogMerged>>,
    pub opted_out: Option<Vec<ChangelogRecord>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangelogRecord {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangelogUpdated {
    pub id: String,
    pub additional_metadata: Option<ChangelogUpdatedMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangelogUpdatedMetadata {
    pub fields_updated: Option<Vec<String>>,
    pub contains: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangelogMerged {
    pub id: String,
    pub additional_metadata: Option<ChangelogMergedMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangelogMergedMetadata {
    pub to: Option<Vec<String>>,
}
