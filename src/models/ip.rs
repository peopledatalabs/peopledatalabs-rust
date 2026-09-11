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
    pub ip: Option<IPInfo>,
    pub company: Option<IPCompany>,
    pub person: Option<IPPerson>,
    pub dataset_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IPInfo {
    pub address: Option<String>,
    pub metadata: Option<IPMetadata>,
    pub location: Option<IPLocation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IPMetadata {
    pub version: Option<i32>,
    pub mobile: Option<bool>,
    pub hosting: Option<bool>,
    pub proxy: Option<bool>,
    pub tor: Option<bool>,
    pub vpn: Option<bool>,
    pub relay: Option<bool>,
    pub service: Option<String>,
    pub asn_domain: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IPLocation {
    pub name: Option<String>,
    pub locality: Option<String>,
    pub region: Option<String>,
    pub metro: Option<String>,
    pub country: Option<String>,
    pub continent: Option<String>,
    pub postal_code: Option<String>,
    pub geo: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IPCompany {
    pub confidence: Option<String>,
    pub id: Option<String>,
    pub website: Option<String>,
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub location: Option<IPCompanyLocation>,
    pub size: Option<String>,
    pub industry: Option<String>,
    pub inferred_revenue: Option<String>,
    pub employee_count: Option<i32>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IPCompanyLocation {
    pub name: Option<String>,
    pub locality: Option<String>,
    pub region: Option<String>,
    pub metro: Option<String>,
    pub country: Option<String>,
    pub continent: Option<String>,
    pub street_address: Option<String>,
    pub address_line_2: Option<String>,
    pub postal_code: Option<String>,
    pub geo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IPPerson {
    pub confidence: Option<String>,
    pub job_title_class: Option<String>,
    #[serde(rename = "job_title_sub_role")]
    pub job_title_subrole: Option<String>,
    pub job_title_role: Option<String>,
    pub job_title_levels: Option<Vec<String>>,
}
