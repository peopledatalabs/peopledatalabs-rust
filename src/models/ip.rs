use serde::{Deserialize, Serialize};

use crate::{models::common::BaseParams, PDLError};

#[derive(Debug, Serialize, Deserialize)]
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
}

impl Default for IPBaseParams {
    fn default() -> Self {
        Self {
            ip: None,
            return_ip_location: None,
            return_ip_metadata: None,
            return_person: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IPParams {
    #[serde(flatten)]
    pub base_params: BaseParams,

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
    status: i32,
    data: IPResult,
}

#[derive(Debug, Serialize, Deserialize)]
struct IPResult {
    ip: IPInfo,
    company: Option<IPCompany>,
    person: Option<IPPerson>,
}

#[derive(Debug, Serialize, Deserialize)]
struct IPInfo {
    address: String,
    metadata: Option<IPMetadata>,
    location: Option<IPLocation>,
}

#[derive(Debug, Serialize, Deserialize)]
struct IPMetadata {
    version: i32,
    mobile: bool,
    hosting: bool,
    proxy: bool,
    tor: bool,
    vpn: bool,
    relay: bool,
    service: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct IPLocation {
    name: String,
    locality: String,
    region: String,
    metro: String,
    country: String,
    continent: String,
    postal_code: String,
    geo: String,
    timezone: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct IPCompany {
    confidence: String,
    id: String,
    domain: String,
    name: String,
    location: IPCompanyLocation,
    size: String,
    industry: String,
    inferred_revenue: String,
    employee_count: i32,
    tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct IPCompanyLocation {
    name: String,
    locality: String,
    region: String,
    metro: String,
    country: String,
    continent: String,
    street_address: String,
    address_line_2: String,
    postal_code: String,
    geo: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct IPPerson {
    confidence: String,
    job_title_subrole: String,
    job_title_role: String,
    job_title_levels: Vec<String>,
}
