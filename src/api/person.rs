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
        self.client
            .get::<EnrichPersonResponse, EnrichPersonParams>(PERSON_ENRICH_PATH, params)
    }

    pub fn bulk_enrich(
        &self,
        params: BulkEnrichPersonParams,
    ) -> Result<Vec<BulkEnrichPersonResponse>, PDLError> {
        self.client
            .post::<Vec<BulkEnrichPersonResponse>, BulkEnrichPersonParams>(
                PERSON_BULK_ENRICH_PATH,
                params,
            )
    }

    pub fn identify(
        &self,
        params: IdentifyPersonParams,
    ) -> Result<IdentifyPersonResponse, PDLError> {
        params.validate()?;
        self.client
            .get::<IdentifyPersonResponse, IdentifyPersonParams>(PERSON_IDENTIFY_PATH, params)
    }

    pub fn search(&self, params: SearchParams) -> Result<SearchPersonResponse, PDLError> {
        params.validate()?;
        self.client
            .get::<SearchPersonResponse, SearchParams>(PERSON_SEARCH_PATH, params)
    }

    pub fn retrieve(
        &self,
        params: RetrievePersonParams,
    ) -> Result<RetrievePersonResponse, PDLError> {
        params.validate()?;
        let url = PERSON_RETRIEVE_PATH.to_string() + &params.person_id;
        self.client
            .get::<RetrievePersonResponse, RetrievePersonParams>(&url, params)
    }

    pub fn bulk_retrieve(
        &self,
        params: BulkRetrievePersonParams,
    ) -> Result<Vec<BulkRetrievePersonResponse>, PDLError> {
        self.client
            .post::<Vec<BulkRetrievePersonResponse>, BulkRetrievePersonParams>(
                PERSON_BULK_RETRIEVE_PATH,
                params,
            )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        client::{PDLCLientOptions, PDLClient},
        models::common::AdditionalParams,
        BaseParams, BulkEnrichPersonParams, BulkEnrichSinglePersonParams, BulkRetrievePersonParams,
        BulkRetrieveSinglePersonParams, EnrichPersonParams, IdentifyPersonParams, PersonParams,
        RetrievePersonParams, SearchBaseParams, SearchParams,
    };

    use super::Person;

    #[test]
    fn test_person_enrich() {
        let api_key = std::env::var("PDL_API_KEY").unwrap();
        let client = PDLClient::new(&api_key).build();

        let person = Person { client };

        let mut base_params = BaseParams::default();
        base_params.pretty = Some(true);

        let mut person_params = PersonParams::default();
        person_params.profile = Some(vec!["http://linkedin.com/in/seanthorne".to_string()]);

        let enrich_person_params = EnrichPersonParams {
            base_params: Some(base_params),
            person_params,
            additional_params: None,
        };

        let resp = person.enrich(enrich_person_params).expect("ERROR");

        assert_eq!(resp.status, 200);
        assert_eq!(
            resp.data.twitter_url,
            Some("twitter.com/seanthorne5".to_string())
        );
    }

    #[test]
    fn test_person_enrich_sandbox() {
        let api_key = std::env::var("PDL_API_KEY").unwrap();
        let mut client_options = PDLCLientOptions::default();
        client_options.sandbox = true;
        let client = PDLClient::new(&api_key).options(client_options).build();

        let person = Person { client };

        let mut base_params = BaseParams::default();
        base_params.pretty = Some(true);

        let mut person_params = PersonParams::default();
        person_params.email = Some(vec!["reneewillis74@aol.com".to_string()]);

        let mut additional_params = AdditionalParams::default();
        additional_params.min_likelihood = Some(6);

        let enrich_person_params = EnrichPersonParams {
            base_params: Some(base_params),
            person_params,
            additional_params: Some(additional_params),
        };

        let resp = person.enrich(enrich_person_params).expect("ERROR");

        assert_eq!(resp.status, 200);
        assert_eq!(
            resp.data.twitter_url,
            Some("twitter.com/rc1994".to_string())
        );
    }

    #[test]
    fn test_person_bulk_enrich() {
        let api_key = std::env::var("PDL_API_KEY").unwrap();
        let client = PDLClient::new(&api_key).build();

        let person = Person { client };

        let mut base_params = BaseParams::default();
        base_params.pretty = Some(true);

        let mut person_params_1 = PersonParams::default();
        person_params_1.profile = Some(vec!["linkedin.com/in/seanthorne".to_string()]);
        let mut person_params_2 = PersonParams::default();
        person_params_2.profile =
            Some(vec!["https://www.linkedin.com/in/haydenconrad/".to_string()]);

        let mut additional_params = AdditionalParams::default();
        additional_params.min_likelihood = Some(6);

        let bulk_enrich_single_person_params_1 = BulkEnrichSinglePersonParams {
            params: person_params_1,
            metadata: None,
        };

        let bulk_enrich_single_person_params_2 = BulkEnrichSinglePersonParams {
            params: person_params_2,
            metadata: None,
        };

        let bulk_enrich_params = BulkEnrichPersonParams {
            requires: None,
            requests: vec![
                bulk_enrich_single_person_params_1,
                bulk_enrich_single_person_params_2,
            ],
        };

        let resp = person.bulk_enrich(bulk_enrich_params).expect("ERROR");

        assert_eq!(resp[0].status, 200);
        assert_eq!(resp[1].status, 200);
    }

    #[test]
    fn test_person_identify() {
        let api_key = std::env::var("PDL_API_KEY").unwrap();
        let client = PDLClient::new(&api_key).build();

        let person = Person { client };

        let mut base_params = BaseParams::default();
        base_params.pretty = Some(true);

        let mut person_params = PersonParams::default();
        person_params.first_name = Some(vec!["sean".to_string()]);
        person_params.last_name = Some(vec!["thorne".to_string()]);
        person_params.company = Some(vec!["people data labs".to_string()]);

        let indentify_person_params = IdentifyPersonParams {
            base_params: Some(base_params),
            person_params,
            additional_params: None,
        };

        let resp = person.identify(indentify_person_params).expect("ERROR");

        assert_eq!(resp.status, 200);
        assert!(resp.matches.len() >= 1);
    }

    #[test]
    fn test_person_identify_sandbox() {
        let api_key = std::env::var("PDL_API_KEY").unwrap();
        let mut client_options = PDLCLientOptions::default();
        client_options.sandbox = true;
        let client = PDLClient::new(&api_key).options(client_options).build();

        let person = Person { client };

        let mut base_params = BaseParams::default();
        base_params.pretty = Some(true);

        let mut person_params = PersonParams::default();
        person_params.company = Some(vec!["adams group".to_string()]);

        let indentify_person_params = IdentifyPersonParams {
            base_params: Some(base_params),
            person_params,
            additional_params: None,
        };

        let resp = person.identify(indentify_person_params).expect("ERROR");

        assert_eq!(resp.status, 200);
        assert!(resp.matches.len() >= 1);
    }

    #[test]
    fn test_person_search() {
        let api_key = std::env::var("PDL_API_KEY").unwrap();
        let client = PDLClient::new(&api_key).build();

        let person = Person { client };

        let mut base_params = BaseParams::default();
        base_params.pretty = Some(true);

        let mut search_base_params = SearchBaseParams::default();
        search_base_params.sql = Some("SELECT * FROM person WHERE location_country='mexico' AND job_title_role='health' AND phone_numbers IS NOT NULL;".to_string());

        let search_params = SearchParams {
            base_params: Some(base_params),
            search_base_params,
            additional_params: None,
        };

        let resp = person.search(search_params).expect("ERROR");

        assert_eq!(resp.status, 200);
        assert_eq!(resp.data.unwrap().len(), 1);
    }

    #[test]
    fn test_person_search_sandbox() {
        let api_key = std::env::var("PDL_API_KEY").unwrap();
        let mut client_options = PDLCLientOptions::default();
        client_options.sandbox = true;
        let client = PDLClient::new(&api_key).options(client_options).build();

        let person = Person { client };

        let num_results: usize = 3;

        let mut base_params = BaseParams::default();
        base_params.size = Some(num_results as i32);

        let mut search_base_params = SearchBaseParams::default();
        search_base_params.sql =
            Some("SELECT * FROM person WHERE location_country='united states';".to_string());

        let search_params = SearchParams {
            base_params: Some(base_params),
            search_base_params,
            additional_params: None,
        };

        let resp = person.search(search_params).expect("ERROR");

        assert_eq!(resp.status, 200);
        assert_eq!(resp.data.unwrap().len(), num_results);
        assert_eq!(resp.scroll_token.is_empty(), false);
        assert_eq!(resp.scroll_token.is_some() && !resp.scroll_token.as_ref().unwrap().is_empty(), true);
    }

    #[test]
    fn test_person_retrieve() {
        let api_key = std::env::var("PDL_API_KEY").unwrap();
        let client = PDLClient::new(&api_key).build();

        let person = Person { client };

        let mut base_params = BaseParams::default();
        base_params.pretty = Some(true);

        let person_id = "qEnOZ5Oh0poWnQ1luFBfVw_0000".to_string();

        let retrieve_person_params = RetrievePersonParams {
            base_params: Some(base_params),
            person_id: person_id.clone(),
        };

        let resp = person.retrieve(retrieve_person_params).expect("ERROR");

        assert_eq!(resp.status, 200);
        assert_eq!(resp.data.id, Some(person_id));
        assert_eq!(resp.data.full_name, Some("sean thorne".to_string()));
    }

    #[test]
    fn test_person_bulk_retrive() {
        let api_key = std::env::var("PDL_API_KEY").unwrap();
        let client = PDLClient::new(&api_key).build();

        let person = Person { client };

        let mut base_params = BaseParams::default();
        base_params.pretty = Some(true);

        let person_id_1 = "qEnOZ5Oh0poWnQ1luFBfVw_0000".to_string();
        let person_id_2 = "9Grd31hT3RFKVzsyecBGPg_0000".to_string();

        let bulk_rerieve_singe_person_param_1 = BulkRetrieveSinglePersonParams {
            id: person_id_1,
            metadata: None,
        };

        let bulk_rerieve_singe_person_param_2 = BulkRetrieveSinglePersonParams {
            id: person_id_2,
            metadata: None,
        };

        let bulk_retrieve_person_params = BulkRetrievePersonParams {
            base_params: Some(base_params),
            requests: vec![
                bulk_rerieve_singe_person_param_1,
                bulk_rerieve_singe_person_param_2,
            ],
            additional_params: None,
        };

        let resp = person
            .bulk_retrieve(bulk_retrieve_person_params)
            .expect("ERROR");
        assert_eq!(resp[0].status, 200);
        assert_eq!(resp[1].status, 200);
    }
}
