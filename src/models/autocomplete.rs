use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{models::common::BaseParams, PDLError};

#[derive(Debug, Serialize, Deserialize)]
pub struct AutocompleteBaseParams {
    /// Field that autocomplete will be calculated for
    #[serde(rename = "field", default)]
    field: String,
    /// Text that is used as the seed for autocompletion
    #[serde(rename = "text", default)]
    text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutocompleteParams {
    #[serde(flatten)]
    base_params: BaseParams,
    #[serde(flatten)]
    autocomplete_base_params: AutocompleteBaseParams,
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
    status: i32,
    data: Vec<AutocompleteResult>,
    fields: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AutocompleteResult {
    /// The plain text name of this Autocomplete API suggestion.
    /// The prefix of this field will match the value of the text input parameter.
    name: String,
    /// The number of records in our Person Dataset for this Autocomplete API suggestion.
    /// This field is used for sorting elements in the data array.
    count: i32,
    /// A set of additional fields returned for each result in the data array.
    /// The metadata fields depend on the field input parameter
    meta: HashMap<String, String>,
}
