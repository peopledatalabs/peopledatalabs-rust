use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    models::common::{AdditionalParams, BaseParams, param_serialize},
    PDLError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PersonParams {
    #[serde(rename = "pdl_id", serialize_with="param_serialize", default)]
    pub pdl_id: Option<Vec<String>>,

    #[serde(rename = "name", serialize_with="param_serialize", default)]
    pub name: Option<Vec<String>>,

    #[serde(rename = "first_name", serialize_with="param_serialize", default)]
    pub first_name: Option<Vec<String>>,

    #[serde(rename = "last_name", serialize_with="param_serialize", default)]
    pub last_name: Option<Vec<String>>,

    #[serde(rename = "middle_name", serialize_with="param_serialize", default)]
    pub middle_name: Option<Vec<String>>,

    #[serde(rename = "location", serialize_with="param_serialize", default)]
    pub location: Option<Vec<String>>,

    #[serde(rename = "street_address", default)]
    pub street_address: Option<String>,

    #[serde(rename = "locality", default)]
    pub locality: Option<String>,

    #[serde(rename = "region", default)]
    pub region: Option<String>,

    #[serde(rename = "country", default)]
    pub country: Option<String>,

    #[serde(rename = "postal_code", serialize_with="param_serialize", default)]
    pub postal_code: Option<Vec<String>>,

    #[serde(rename = "company", serialize_with="param_serialize", default)]
    pub company: Option<Vec<String>>,

    #[serde(rename = "school", serialize_with="param_serialize", default)]
    pub school: Option<Vec<String>>,

    #[serde(rename = "phone", serialize_with="param_serialize", default)]
    pub phone: Option<Vec<String>>,

    #[serde(rename = "email", serialize_with="param_serialize", default)]
    pub email: Option<Vec<String>>,

    #[serde(rename = "email_hash", serialize_with="param_serialize", default)]
    pub email_hash: Option<Vec<String>>,

    #[serde(rename = "profile", serialize_with="param_serialize", default)]
    pub profile: Option<Vec<String>>,

    #[serde(rename = "lid", serialize_with="param_serialize", default)]
    pub lid: Option<Vec<String>>,

    #[serde(rename = "birth_date", serialize_with="param_serialize", default)]
    pub birth_date: Option<Vec<String>>,
}

impl Default for PersonParams {
    fn default() -> Self {
        Self {
            pdl_id: None,
            name: None,
            first_name: None,
            last_name: None,
            middle_name: None,
            location: None,
            street_address: None,
            locality: None,
            region: None,
            country: None,
            postal_code: None,
            company: None,
            school: None,
            phone: None,
            email: None,
            email_hash: None,
            profile: None,
            lid: None,
            birth_date: None,
        }
    }
}

impl PersonParams {
    fn validate(&self) -> Result<(), PDLError> {
        if !self.profile.is_none() {
            return Ok(());
        }
        if !self.email.is_none() {
            return Ok(());
        }
        if !self.phone.is_none() {
            return Ok(());
        }
        if !self.email_hash.is_none() {
            return Ok(());
        }
        if self.lid.is_none() {
            return Ok(());
        }

        if (!self.first_name.is_none() && !self.last_name.is_none()) || !self.name.is_none() {
            if !self.locality.is_none()
                || !self.region.is_none()
                || !self.company.is_none()
                || self.school.is_none()
                || self.location.is_none()
                || self.postal_code.is_none()
            {
                return Ok(());
            }
        }

        Err(PDLError::ValidationError)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnrichPersonParams {
    #[serde(flatten)]
    pub base_params: Option<BaseParams>,

    #[serde(flatten)]
    pub person_params: PersonParams,

    #[serde(flatten)]
    pub additional_params: Option<AdditionalParams>,
}

impl Default for EnrichPersonParams {
    fn default() -> Self {
        Self {
            base_params: None,
            person_params: PersonParams::default(),
            additional_params: None,
        }
    }
}

impl EnrichPersonParams {
    pub fn validate(&self) -> Result<(), PDLError> {
        self.person_params.validate()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnrichPersonResponse {
    status: i32,
    likelihood: i32,
    data: Person,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkEnrichPersonParams {
    pub requests: Vec<BulkEnrichSinglePersonParams>,
}

impl BulkEnrichPersonParams {
    pub fn validate(&self) -> Result<(), PDLError> {
        for request in &self.requests {
            if let Err(err) = request.validate() {
                return Err(err);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkEnrichSinglePersonParams {
    pub params: PersonParams,
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
    enrich_person_response: EnrichPersonResponse,
    metadata: Option<PersonMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifyPersonParams {
    #[serde(flatten)]
    pub base_params: Option<BaseParams>,

    #[serde(flatten)]
    pub person_params: PersonParams,

    #[serde(flatten)]
    pub additional_params: Option<AdditionalParams>,
}

impl Default for IdentifyPersonParams {
    fn default() -> Self {
        Self {
            base_params: None,
            person_params: PersonParams::default(),
            additional_params: None,
        }
    }
}

impl IdentifyPersonParams {
    pub fn validate(&self) -> Result<(), PDLError> {
        self.person_params.validate()?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifyPersonResponse {
    status: i32,
    matches: Vec<PersonMatch>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PersonMatch {
    data: Person,
    match_score: i32,
    matched_on: Vec<String>,
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
    status: i32,
    data: Person,
    billed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkRetrievePersonParams {
    #[serde(flatten)]
    pub base_params: Option<BaseParams>,

    pub requests: Vec<BulkRetrieveSinglePersonParams>,

    #[serde(flatten)]
    pub additional_params: Option<AdditionalParams>,
}

impl Default for BulkRetrievePersonParams {
    fn default() -> Self {
        Self {
            base_params: None,
            requests: vec![],
            additional_params: None,
        }
    }
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
            if let Err(err) = request.validate() {
                return Err(err);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkRetrieveSinglePersonParams {
    pub id: String, // The ID of a person
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
    status: i32,
    data: Person,
    billed: bool,
    metadata: PersonMetadata,
}

pub type PersonMetadata = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchPersonResponse {
    status: i32,

    #[serde(rename = "error")]
    error_info: PersonErrorInfo,

    data: Vec<Person>,
    total: i32,
    scroll_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PersonErrorInfo {
    #[serde(rename = "type")]
    error_type: Vec<String>,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    #[serde(rename = "pdl_id", default)]
    pdl_id: Option<String>,

    #[serde(rename = "name", default)]
    name: Option<String>,

    #[serde(rename = "first_name", default)]
    first_name: Option<String>,

    #[serde(rename = "last_name", default)]
    last_name: Option<String>,

    #[serde(rename = "middle_name", default)]
    middle_name: Option<String>,

    #[serde(rename = "location", default)]
    location: Option<String>,

    #[serde(rename = "street_address", default)]
    street_address: Option<String>,

    #[serde(rename = "locality", default)]
    locality: Option<String>,

    #[serde(rename = "region", default)]
    region: Option<String>,

    #[serde(rename = "country", default)]
    country: Option<String>,

    #[serde(rename = "postal_code", default)]
    postal_code: Option<String>,

    #[serde(rename = "company", default)]
    company: Option<String>,

    #[serde(rename = "school", default)]
    school: Option<String>,

    #[serde(rename = "phone", default)]
    phone: Option<String>,

    #[serde(rename = "email", default)]
    email: Option<String>,

    #[serde(rename = "email_hash", default)]
    email_hash: Option<String>,

    #[serde(rename = "profile", default)]
    profile: Option<String>,

    #[serde(rename = "lid", default)]
    lid: Option<String>,

    #[serde(rename = "birth_date", default)]
    birth_date: Option<String>,
}
