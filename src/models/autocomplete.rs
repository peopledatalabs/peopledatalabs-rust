use serde::{Deserialize, Serialize};

use crate::{models::common::BaseParams, PDLError};

#[derive(Debug, Serialize, Deserialize)]
pub struct AutocompleteBaseParams {
    /// Field that autocomplete will be calculated for
    #[serde(rename = "field", default)]
    pub field: String,
    /// Text that is used as the seed for autocompletion
    #[serde(rename = "text", default)]
    pub text: Option<String>,
    /// Setting titlecase to true will titlecase the data in 200 responses
    #[serde(rename = "titlecase", skip_serializing_if = "Option::is_none")]
    pub titlecase: Option<bool>,
    /// Setting beta to true will enable the beta autocomplete endpoint
    #[serde(rename = "beta", skip_serializing_if = "Option::is_none")]
    pub beta: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutocompleteParams {
    #[serde(flatten)]
    pub base_params: Option<BaseParams>,
    #[serde(flatten)]
    pub autocomplete_base_params: AutocompleteBaseParams,
}

impl AutocompleteParams {
    pub fn validate(&self) -> Result<(), PDLError> {
        if self.autocomplete_base_params.field.is_empty() {
            return Err(PDLError::ValidationError);
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutocompleteResponse {
    /// See https://docs.peopledatalabs.com/docs/output-response-autocomplete-api for more information
    pub status: i32,
    pub data: Option<Vec<AutocompleteResult>>,
    pub fields: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub website: Option<String>,
    pub location_name: Option<String>,
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutocompleteResult {
    /// The plain text name of this Autocomplete API suggestion.
    /// The prefix of this field will match the value of the text input parameter.
    pub name: Option<String>,
    /// The number of records in our Person Dataset for this Autocomplete API suggestion.
    /// This field is used for sorting elements in the data array.
    pub count: Option<i32>,
    /// A set of additional fields returned for each result in the data array.
    /// The metadata fields depend on the field input parameter
    pub meta: Option<Meta>,
}
