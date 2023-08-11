use crate::{
    models::{
        common::SearchParams,
        person::{
            BulkEnrichPersonParams, BulkEnrichPersonResponse, BulkRetrievePersonParams,
            BulkRetrievePersonResponse, EnrichPersonParams, EnrichPersonResponse,
            IdentifyPersonParams, IdentifyPersonResponse, RetrievePersonParams,
            RetrievePersonResponse, SearchPersonResponse,
        },
    },
    PDLClient, PDLError,
};

pub(crate) static PERSON_ENRICH_PATH: &str = "/person/enrich";
pub(crate) static PERSON_BULK_ENRICH_PATH: &str = "/person/bulk";
pub(crate) static PERSON_IDENTIFY_PATH: &str = "/person/identify";
pub(crate) static PERSON_SEARCH_PATH: &str = "/person/search";
pub(crate) static PERSON_RETRIEVE_PATH: &str = "/person/retrieve/";
pub(crate) static PERSON_BULK_RETRIEVE_PATH: &str = "/person/retrieve/bulk";

pub struct Person {
    pub client: PDLClient,
}

impl Person {
    pub fn enrich(&self, params: EnrichPersonParams) -> Result<EnrichPersonResponse, PDLError> {
        params.validate()?;
        let qs = serde_qs::to_string(&params).map_err(|_| PDLError::ValidationError)?;
        dbg!(&qs);
        let r: EnrichPersonResponse = self.client.get(PERSON_ENRICH_PATH, &qs)?;

        Ok(r)
    }

    pub fn bulk_enrich(
        &self,
        params: BulkEnrichPersonParams,
    ) -> Result<BulkEnrichPersonResponse, PDLError> {
        params.validate()?;
        let qs = serde_qs::to_string(&params).map_err(|_| PDLError::ValidationError)?;
        let r: BulkEnrichPersonResponse = self.client.get(PERSON_BULK_ENRICH_PATH, &qs)?;

        Ok(r)
    }

    pub fn identify(
        &self,
        params: IdentifyPersonParams,
    ) -> Result<IdentifyPersonResponse, PDLError> {
        params.validate()?;
        let qs = serde_qs::to_string(&params).map_err(|_| PDLError::ValidationError)?;
        let r: IdentifyPersonResponse = self.client.get(PERSON_IDENTIFY_PATH, &qs)?;

        Ok(r)
    }

    pub fn search(&self, params: SearchParams) -> Result<SearchPersonResponse, PDLError> {
        params.validate()?;
        let qs = serde_qs::to_string(&params).map_err(|_| PDLError::ValidationError)?;
        let r: SearchPersonResponse = self.client.get(PERSON_SEARCH_PATH, &qs)?;

        Ok(r)
    }

    pub fn retrieve(
        &self,
        params: RetrievePersonParams,
    ) -> Result<RetrievePersonResponse, PDLError> {
        params.validate()?;
        let qs = serde_qs::to_string(&params).map_err(|_| PDLError::ValidationError)?;
        let url = PERSON_RETRIEVE_PATH.to_string() + &params.person_id;
        let r: RetrievePersonResponse = self.client.get(&url, &qs)?;

        Ok(r)
    }

    pub fn bulk_retrieve(
        &self,
        params: BulkRetrievePersonParams,
    ) -> Result<BulkRetrievePersonResponse, PDLError> {
        params.validate()?;
        let qs = serde_qs::to_string(&params).map_err(|_| PDLError::ValidationError)?;
        let r: BulkRetrievePersonResponse = self.client.get(PERSON_BULK_RETRIEVE_PATH, &qs)?;

        Ok(r)
    }
}
