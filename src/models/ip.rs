use serde::{Deserialize, Serialize};

use crate::{models::common::BaseParams, PDLError};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ConfidenceLevel {
    VeryHigh,
    High,
    Moderate,
    Low,
    VeryLow,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPBaseParams {
    /// IP that is used as the seed for enrichment
    #[serde(rename = "ip", default)]
    pub ip: Option<String>,
    /// If true, the response will include the location of the IP
    #[serde(rename = "return_ip_location", default)]
    pub return_ip_location: Option<bool>,
    /// If true, the response will include the metadata of the IP
    #[serde(rename = "return_ip_metadata", default)]
    pub return_ip_metadata: Option<bool>,
    /// If true, the response will include the person fields
    #[serde(rename = "return_person", default)]
    pub return_person: Option<bool>,
    /// If true, the response will return metadata/location even if no company is found
    #[serde(rename = "return_if_unmatched", default)]
    pub return_if_unmatched: Option<bool>,
    /// If true, the response will return updated title tags
    #[serde(rename = "updated_title_roles", default)]
    pub updated_title_roles: Option<bool>,
    /// Minimum confidence level required for returning data
    #[serde(rename = "min_confidence", default)]
    pub min_confidence: Option<ConfidenceLevel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IPParams {
    #[serde(flatten)]
    pub base_params: Option<BaseParams>,

    #[serde(flatten)]
    pub ip_base_params: IPBaseParams,
}

impl IPParams {
    pub fn validate(&self) -> Result<(), PDLError> {
        if self.ip_base_params.ip.is_none() {
            return Err(PDLError::ValidationError);
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IPResponse {
    pub status: i32,
    pub data: IPResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IPResult {
    pub ip: IPInfo,
    pub company: Option<IPCompany>,
    pub person: Option<IPPerson>,
    pub dataset_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IPInfo {
    pub address: String,
    pub metadata: Option<IPMetadata>,
    pub location: Option<IPLocation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IPMetadata {
    pub version: i32,
    pub mobile: bool,
    pub hosting: bool,
    pub proxy: bool,
    pub tor: bool,
    pub vpn: bool,
    pub relay: bool,
    pub service: String,
    pub asn_domain: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IPLocation {
    pub name: String,
    pub locality: String,
    pub region: String,
    pub metro: String,
    pub country: String,
    pub continent: String,
    pub postal_code: String,
    pub geo: String,
    pub timezone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IPCompany {
    pub confidence: String,
    pub id: String,
    pub website: String,
    pub name: String,
    pub location: IPCompanyLocation,
    pub size: String,
    pub industry: String,
    pub inferred_revenue: String,
    pub employee_count: i32,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IPCompanyLocation {
    pub name: String,
    pub locality: String,
    pub region: String,
    pub metro: String,
    pub country: String,
    pub continent: String,
    pub street_address: String,
    pub address_line_2: String,
    pub postal_code: String,
    pub geo: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IPPerson {
    pub confidence: String,
    pub job_title_subrole: String,
    pub job_title_role: String,
    pub job_title_levels: Vec<String>,
}
