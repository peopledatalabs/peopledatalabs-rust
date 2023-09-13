use serde::{Deserialize, Serialize, Serializer};

use crate::PDLError;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BaseParams {
    /// Whether the output should have human-readable indentation.
    #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
    pub pretty: Option<bool>,
    /// The number of matched records to return for this query if they exist.
    /// Must be between 1 and 1000 (inclusive).
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AdditionalParams {
    /// The minimum likelihood score a response must possess in order to return a 200.
    #[serde(rename = "min_likelihood", default)]
    pub min_likelihood: Option<i32>,
    /// Parameter specifying the fields and data points a response must have to return a 200.
    #[serde(rename = "required", default)]
    pub required: Option<String>,
    /// Setting titlecase to true will titlecase the person data in 200 responses.
    #[serde(rename = "titlecase", default)]
    pub titlecase: Option<bool>,
    /// A comma-separated string of fields that you would like the response to include.
    #[serde(rename = "data_include", default)]
    pub data_include: Option<String>,
    /// If set to true, includes a top-level (alongside "data", "status", etc) field "matched" which
    /// includes a value for each queried field parameter that was "matched-on" during our internal query.
    #[serde(rename = "include_if_matched", default)]
    pub include_if_matched: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SearchBaseParams {
    /// An Elasticsearch (v7.7) query.
    #[serde(rename = "query", default)]
    pub query: Option<serde_json::Value>,
    /// A SQL query of the format: SELECT * FROM person WHERE XXX.
    #[serde(rename = "sql", default)]
    pub sql: Option<String>,
    /// An offset value for pagination.
    #[serde(rename = "from", default)]
    pub from: Option<i32>,
    /// An offset key for paginating between batches.
    #[serde(rename = "scroll_token", default)]
    pub scroll_token: Option<String>,
    /// The dataset category to return records from.
    #[serde(rename = "dataset", default)]
    pub dataset: Option<String>,
    /// Setting titlecase to true will titlecase the person data in 200 responses.
    #[serde(rename = "titlecase", default)]
    pub titlecase: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SearchParams {
    #[serde(flatten)]
    pub base_params: Option<BaseParams>,
    #[serde(flatten)]
    pub search_base_params: SearchBaseParams,
    #[serde(flatten)]
    pub additional_params: Option<AdditionalParams>,
}

impl SearchParams {
    pub fn validate(&self) -> Result<(), PDLError> {
        if (self.search_base_params.query.is_none() && self.search_base_params.sql.is_none())
            || (self.search_base_params.query.is_some() && self.search_base_params.sql.is_some())
        {
            return Err(PDLError::ValidationError);
        }
        Ok(())
    }
}

pub fn param_serialize<S>(vec: &Option<Vec<String>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match vec {
        Some(data) => serializer.serialize_str(&data.join(", ")),
        None => serializer.serialize_none(),
    }
}
