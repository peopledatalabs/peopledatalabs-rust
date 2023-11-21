use crate::{
    models::{
        common::SearchParams,
        company::{
            CleanCompanyParams, CleanCompanyResponse, CompanyResponse, EnrichCompanyParams,
            SearchCompanyResponse,
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
    pub fn enrich(&self, params: EnrichCompanyParams) -> Result<CompanyResponse, PDLError> {
        params.validate()?;
        self.client
            .get::<CompanyResponse, EnrichCompanyParams>(ENRICH_PATH, params)
    }

    /// Search gives you access to every record in our full Company dataset,
    /// which you can filter and segment using a search query.
    /// docs: https://docs.peopledatalabs.com/docs/company-search-api
    pub fn search(&self, params: SearchParams) -> Result<SearchCompanyResponse, PDLError> {
        params.validate()?;
        self.client
            .get::<SearchCompanyResponse, SearchParams>(SEARCH_PATH, params)
    }

    /// Clean your company data, so you can better query our person data
    /// docs: https://docs.peopledatalabs.com/docs/cleaner-apis-reference
    pub fn clean(&self, params: CleanCompanyParams) -> Result<CleanCompanyResponse, PDLError> {
        params.validate()?;
        self.client
            .get::<CleanCompanyResponse, CleanCompanyParams>(CLEAN_PATH, params)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        client::PDLClient, BaseParams, CleanCompanyParams, CompanyParams, EnrichCompanyParams,
        SearchBaseParams, SearchParams,
    };

    use super::Company;

    #[test]
    fn test_company_enrich() {
        let api_key = std::env::var("PDL_API_KEY").unwrap();
        let client = PDLClient::new(&api_key).build();

        let company = Company { client };

        let mut base_params = BaseParams::default();
        base_params.pretty = Some(true);

        let mut company_params = CompanyParams::default();
        company_params.name = Some("google".to_string());

        let enrich_params = EnrichCompanyParams {
            base_params: Some(base_params),
            company_params,
            additional_params: None,
        };

        let resp = company.enrich(enrich_params).expect("ERROR");

        assert_eq!(resp.status, Some(200));
        assert_eq!(resp.name, Some("google".to_string()));
    }

    #[test]
    fn test_company_clean() {
        let api_key = std::env::var("PDL_API_KEY").unwrap();
        let client = PDLClient::new(&api_key).build();

        let company = Company { client };

        let mut clean_params = CleanCompanyParams::default();
        clean_params.name = Some("google".to_string());

        let resp = company.clean(clean_params).expect("ERROR");

        assert_eq!(resp.status, 200);
        assert_eq!(resp.name, Some("google".to_string()));
    }

    #[test]
    fn test_company_search() {
        let api_key = std::env::var("PDL_API_KEY").unwrap();
        let client = PDLClient::new(&api_key).build();

        let company = Company { client };

        let mut base_params = BaseParams::default();
        base_params.pretty = Some(true);

        let mut search_base_params = SearchBaseParams::default();
        search_base_params.sql =
            Some("SELECT * FROM company WHERE website='google.com';".to_string());

        let search_params = SearchParams {
            base_params: Some(base_params),
            search_base_params,
            additional_params: None,
        };

        let resp = company.search(search_params).expect("ERROR");

        assert_eq!(resp.status, 200);
        assert_eq!(resp.total, Some(1));
    }
}
