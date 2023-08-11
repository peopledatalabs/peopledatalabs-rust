use crate::{
    models::{
        common::SearchParams,
        company::{
            CleanCompanyParams, CleanCompanyResponse, EnrichCompanyParams, EnrichCompanyResponse,
        },
    },
    PDLClient, PDLError,
};

pub(crate) static ENRICH_PATH: &str = "/company/enrich";
pub(crate) static SEARCH_PATH: &str = "/company/search";
pub(crate) static CLEAN_PATH: &str = "/company/clean";

pub struct Company {
    pub client: PDLClient,
}

impl Company {
    /// Enrich a company
    /// docs: https://docs.peopledatalabs.com/docs/company-enrichment-api
    pub fn enrich(&self, params: EnrichCompanyParams) -> Result<EnrichCompanyResponse, PDLError> {
        params.validate()?;
        let qs = serde_qs::to_string(&params).map_err(|_| PDLError::ValidationError)?;
        let r: EnrichCompanyResponse = self.client.get(ENRICH_PATH, &qs)?;

        Ok(r)
    }

    /// Search gives you access to every record in our full Company dataset,
    /// which you can filter and segment using a search query.
    /// docs: https://docs.peopledatalabs.com/docs/company-search-api
    pub fn search(&self, params: SearchParams) -> Result<EnrichCompanyResponse, PDLError> {
        params.validate()?;
        let qs = serde_qs::to_string(&params).map_err(|_| PDLError::ValidationError)?;
        let r: EnrichCompanyResponse = self.client.get(SEARCH_PATH, &qs)?;

        Ok(r)
    }

    /// Clean your company data, so you can better query our person data
    /// docs: https://docs.peopledatalabs.com/docs/cleaner-apis-reference
    pub fn clean(&self, params: CleanCompanyParams) -> Result<CleanCompanyResponse, PDLError> {
        params.validate()?;
        let qs = serde_qs::to_string(&params).map_err(|_| PDLError::ValidationError)?;
        let r: CleanCompanyResponse = self.client.get(CLEAN_PATH, &qs)?;

        Ok(r)
    }
}
