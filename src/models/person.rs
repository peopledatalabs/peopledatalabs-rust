use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    models::common::{param_serialize, AdditionalParams, BaseParams},
    PDLError,
};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PersonParams {
    #[serde(
        rename = "pdl_id",
        serialize_with = "param_serialize",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub pdl_id: Option<Vec<String>>,

    #[serde(
        rename = "name",
        serialize_with = "param_serialize",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub name: Option<Vec<String>>,

    #[serde(
        rename = "first_name",
        serialize_with = "param_serialize",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub first_name: Option<Vec<String>>,

    #[serde(
        rename = "last_name",
        serialize_with = "param_serialize",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub last_name: Option<Vec<String>>,

    #[serde(
        rename = "middle_name",
        serialize_with = "param_serialize",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub middle_name: Option<Vec<String>>,

    #[serde(
        rename = "location",
        serialize_with = "param_serialize",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub location: Option<Vec<String>>,

    #[serde(
        rename = "street_address",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub street_address: Option<String>,

    #[serde(rename = "locality", skip_serializing_if = "Option::is_none", default)]
    pub locality: Option<String>,

    #[serde(rename = "region", skip_serializing_if = "Option::is_none", default)]
    pub region: Option<String>,

    #[serde(rename = "country", skip_serializing_if = "Option::is_none", default)]
    pub country: Option<String>,

    #[serde(
        rename = "postal_code",
        serialize_with = "param_serialize",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub postal_code: Option<Vec<String>>,

    #[serde(
        rename = "company",
        serialize_with = "param_serialize",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub company: Option<Vec<String>>,

    #[serde(
        rename = "school",
        serialize_with = "param_serialize",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub school: Option<Vec<String>>,

    #[serde(
        rename = "phone",
        serialize_with = "param_serialize",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub phone: Option<Vec<String>>,

    #[serde(
        rename = "email",
        serialize_with = "param_serialize",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub email: Option<Vec<String>>,

    #[serde(
        rename = "email_hash",
        serialize_with = "param_serialize",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub email_hash: Option<Vec<String>>,

    #[serde(
        rename = "profile",
        serialize_with = "param_serialize",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub profile: Option<Vec<String>>,

    #[serde(
        rename = "lid",
        serialize_with = "param_serialize",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub lid: Option<Vec<String>>,

    #[serde(
        rename = "birth_date",
        serialize_with = "param_serialize",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub birth_date: Option<Vec<String>>,
}

impl PersonParams {
    fn validate(&self) -> Result<(), PDLError> {
        if self.pdl_id.is_some() {
            return Ok(());
        }
        if self.profile.is_some() {
            return Ok(());
        }
        if self.email.is_some() {
            return Ok(());
        }
        if self.phone.is_some() {
            return Ok(());
        }
        if self.email_hash.is_some() {
            return Ok(());
        }
        if self.lid.is_none() {
            return Ok(());
        }

        if ((self.first_name.is_some() && self.last_name.is_some()) || self.name.is_some())
            && (self.locality.is_some()
                || self.region.is_some()
                || self.company.is_some()
                || self.school.is_none()
                || self.location.is_none()
                || self.postal_code.is_none())
        {
            return Ok(());
        }

        Err(PDLError::ValidationError)
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EnrichPersonParams {
    #[serde(flatten)]
    pub base_params: Option<BaseParams>,

    #[serde(flatten)]
    pub person_params: PersonParams,

    #[serde(flatten)]
    pub additional_params: Option<AdditionalParams>,
}

impl EnrichPersonParams {
    pub fn validate(&self) -> Result<(), PDLError> {
        self.person_params.validate()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnrichPersonResponse {
    pub status: i32,
    pub likelihood: i32,
    pub data: Person,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkEnrichPersonParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub requires: Option<String>,
    pub requests: Vec<BulkEnrichSinglePersonParams>,
}

impl BulkEnrichPersonParams {
    pub fn validate(&self) -> Result<(), PDLError> {
        for request in &self.requests {
            request.validate()?
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkEnrichSinglePersonParams {
    pub params: PersonParams,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub metadata: Option<PersonMetadata>,
}

impl BulkEnrichSinglePersonParams {
    pub fn validate(&self) -> Result<(), PDLError> {
        self.params.validate()?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkEnrichPersonResponse {
    #[serde(flatten)]
    pub data: Option<Vec<EnrichPersonResponse>>,
    pub status: i32,
    pub likelihood: Option<i32>,
    pub metadata: Option<PersonMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IdentifyPersonParams {
    #[serde(flatten)]
    pub base_params: Option<BaseParams>,

    #[serde(flatten)]
    pub person_params: PersonParams,

    #[serde(flatten)]
    pub additional_params: Option<AdditionalParams>,
}

impl IdentifyPersonParams {
    pub fn validate(&self) -> Result<(), PDLError> {
        self.person_params.validate()?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifyPersonResponse {
    pub status: i32,
    pub matches: Vec<PersonMatch>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonMatch {
    pub data: Person,
    pub match_score: i32,
    pub matched_on: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RetrievePersonParams {
    #[serde(flatten)]
    pub base_params: Option<BaseParams>,

    #[serde(rename = "-")]
    pub person_id: String, // The ID of a person
}

impl RetrievePersonParams {
    pub fn validate(&self) -> Result<(), PDLError> {
        if self.person_id.is_empty() {
            return Err(PDLError::ValidationError);
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RetrievePersonResponse {
    pub status: i32,
    pub data: Person,
    pub billed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkRetrievePersonParams {
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub base_params: Option<BaseParams>,

    pub requests: Vec<BulkRetrieveSinglePersonParams>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub additional_params: Option<AdditionalParams>,
}

impl BulkRetrievePersonParams {
    pub fn validate(&self) -> Result<(), PDLError> {
        if self.requests.is_empty() {
            return Err(PDLError::ValidationError);
        }
        if self.requests.len() > 100 {
            return Err(PDLError::ValidationError);
        }
        for request in &self.requests {
            request.validate()?
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkRetrieveSinglePersonParams {
    pub id: String, // The ID of a person
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PersonMetadata>,
}

impl BulkRetrieveSinglePersonParams {
    pub fn validate(&self) -> Result<(), PDLError> {
        if self.id.is_empty() {
            return Err(PDLError::ValidationError);
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkRetrievePersonResponse {
    pub status: i32,
    pub data: Person,
    pub billed: bool,
    pub metadata: Option<PersonMetadata>,
}

pub type PersonMetadata = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchPersonResponse {
    pub status: i32,

    #[serde(rename = "error")]
    pub error_info: Option<PersonErrorInfo>,

    pub data: Option<Vec<Person>>,
    pub total: i32,
    pub scroll_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonErrorInfo {
    #[serde(rename = "type")]
    pub error_type: Vec<String>,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub id: Option<String>,
    pub full_name: Option<String>,
    pub first_name: Option<String>,
    pub middle_initial: Option<String>,
    pub middle_name: Option<String>,
    pub last_initial: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<String>,
    pub birth_year: Option<i32>,
    pub birth_date: Option<String>,
    pub linkedin_url: Option<String>,
    pub linkedin_username: Option<String>,
    pub linkedin_id: Option<String>,
    pub facebook_url: Option<String>,
    pub facebook_username: Option<String>,
    pub facebook_id: Option<String>,
    pub twitter_url: Option<String>,
    pub twitter_username: Option<String>,
    pub github_url: Option<String>,
    pub github_username: Option<String>,
    pub work_email: Option<String>,
    pub personal_emails: Option<Vec<String>>,
    pub recommended_personal_email: Option<String>,
    pub mobile_phone: Option<String>,
    pub industry: Option<String>,
    pub job_title: Option<String>,
    pub job_title_role: Option<String>,
    pub job_title_sub_role: Option<String>,
    pub job_title_levels: Option<Vec<String>>,
    pub job_company_id: Option<String>,
    pub job_company_name: Option<String>,
    pub job_company_website: Option<String>,
    pub job_company_size: Option<String>,
    pub job_company_founded: Option<i32>,
    pub job_company_industry: Option<String>,
    pub job_company_linkedin_url: Option<String>,
    pub job_company_linkedin_id: Option<String>,
    pub job_company_facebook_url: Option<String>,
    pub job_company_twitter_url: Option<String>,
    pub job_company_location_name: Option<String>,
    pub job_company_location_locality: Option<String>,
    pub job_company_location_metro: Option<String>,
    pub job_company_location_region: Option<String>,
    pub job_company_location_geo: Option<String>,
    pub job_company_location_street_address: Option<String>,
    pub job_company_location_address_line_2: Option<String>,
    pub job_company_location_postal_code: Option<String>,
    pub job_company_location_country: Option<String>,
    pub job_company_location_continent: Option<String>,
    pub job_last_updated: Option<String>,
    pub job_start_date: Option<String>,
    pub location_name: Option<String>,
    pub location_locality: Option<String>,
    pub location_metro: Option<String>,
    pub location_region: Option<String>,
    pub location_country: Option<String>,
    pub location_continent: Option<String>,
    pub location_street_address: Option<String>,
    pub location_address_line_2: Option<String>,
    pub location_postal_code: Option<String>,
    pub location_geo: Option<String>,
    pub location_last_updated: Option<String>,
    pub phone_numbers: Option<Vec<String>>,
    pub emails: Option<Vec<Email>>,
    pub interests: Option<Vec<String>>,
    pub skills: Option<Vec<String>>,
    pub location_names: Option<Vec<String>>,
    pub regions: Option<Vec<String>>,
    pub countries: Option<Vec<String>>,
    pub street_addresses: Option<Vec<StreetAddress>>,
    pub experience: Option<Vec<Experience>>,
    pub education: Option<Vec<Education>>,
    pub profiles: Option<Vec<Profile>>,
    pub version_status: Option<VersionStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Email {
    pub address: Option<String>,
    pub type_: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreetAddress {
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
pub struct Experience {
    pub company: Option<Company>,
    pub location_names: Option<Vec<String>>,
    pub end_date: Option<String>,
    pub start_date: Option<String>,
    pub title: Option<Title>,
    pub is_primary: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Company {
    pub name: Option<String>,
    pub size: Option<String>,
    pub id: Option<String>,
    pub founded: Option<i32>,
    pub industry: Option<String>,
    pub location: Option<Location>,
    pub linkedin_url: Option<String>,
    pub linkedin_id: Option<String>,
    pub facebook_url: Option<String>,
    pub twitter_url: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
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
pub struct Title {
    pub name: Option<String>,
    pub role: Option<String>,
    pub sub_role: Option<String>,
    pub levels: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Education {
    pub school: Option<School>,
    pub end_date: Option<String>,
    pub start_date: Option<String>,
    pub gpa: Option<f32>,
    pub degrees: Option<Vec<String>>,
    pub majors: Option<Vec<String>>,
    pub minors: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct School {
    pub name: Option<String>,
    pub type_: Option<String>,
    pub id: Option<String>,
    pub location: Option<Location>,
    pub linkedin_url: Option<String>,
    pub facebook_url: Option<String>,
    pub twitter_url: Option<String>,
    pub website: Option<String>,
    pub domain: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub network: Option<String>,
    pub id: Option<String>,
    pub url: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionStatus {
    pub status: Option<String>,
    pub contains: Option<Vec<String>>,
    pub previous_version: Option<String>,
    pub current_version: Option<String>,
}
