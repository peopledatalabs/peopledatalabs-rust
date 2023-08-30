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
        let r = self
            .client
            .get::<EnrichPersonResponse>(PERSON_ENRICH_PATH, &qs)?;

        Ok(r)
    }

    pub fn bulk_enrich(
        &self,
        params: BulkEnrichPersonParams,
    ) -> Result<BulkEnrichPersonResponse, PDLError> {
        params.validate()?;
        let qs = serde_qs::to_string(&params).map_err(|_| PDLError::ValidationError)?;
        let r = self
            .client
            .get::<BulkEnrichPersonResponse>(PERSON_BULK_ENRICH_PATH, &qs)?;

        Ok(r)
    }

    pub fn identify(
        &self,
        params: IdentifyPersonParams,
    ) -> Result<IdentifyPersonResponse, PDLError> {
        params.validate()?;
        let qs = serde_qs::to_string(&params).map_err(|_| PDLError::ValidationError)?;
        let r = self
            .client
            .get::<IdentifyPersonResponse>(PERSON_IDENTIFY_PATH, &qs)?;

        Ok(r)
    }

    pub fn search(&self, params: SearchParams) -> Result<SearchPersonResponse, PDLError> {
        params.validate()?;
        let qs = serde_qs::to_string(&params).map_err(|_| PDLError::ValidationError)?;
        let r = self
            .client
            .get::<SearchPersonResponse>(PERSON_SEARCH_PATH, &qs)?;

        Ok(r)
    }

    pub fn retrieve(
        &self,
        params: RetrievePersonParams,
    ) -> Result<RetrievePersonResponse, PDLError> {
        params.validate()?;
        let qs = serde_qs::to_string(&params).map_err(|_| PDLError::ValidationError)?;
        let url = PERSON_RETRIEVE_PATH.to_string() + &params.person_id;
        let r = self.client.get::<RetrievePersonResponse>(&url, &qs)?;

        Ok(r)
    }

    pub fn bulk_retrieve(
        &self,
        params: BulkRetrievePersonParams,
    ) -> Result<BulkRetrievePersonResponse, PDLError> {
        params.validate()?;
        let qs = serde_qs::to_string(&params).map_err(|_| PDLError::ValidationError)?;
        let r = self
            .client
            .get::<BulkRetrievePersonResponse>(PERSON_BULK_RETRIEVE_PATH, &qs)?;

        Ok(r)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        client::PDLClient, BaseParams, BulkRetrievePersonParams, BulkRetrieveSinglePersonParams,
        EnrichPersonParams, IdentifyPersonParams, PersonParams, RetrievePersonParams,
        SearchBaseParams, SearchParams,
    };

    use super::Person;

    #[test]
    fn test_person_enrich() {
        let api_key = std::env::var("PDL_API_KEY").unwrap();
        let client = PDLClient::new(&api_key, "v5");

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
    #[ignore = "bulk? 🤷"]
    fn test_person_bulk_enrich() {
        todo!()
    }

    #[test]
    fn test_person_identify() {
        let api_key = std::env::var("PDL_API_KEY").unwrap();
        let client = PDLClient::new(&api_key, "v5");

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
    fn test_person_search() {
        let api_key = std::env::var("PDL_API_KEY").unwrap();
        let client = PDLClient::new(&api_key, "v5");

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
    #[ignore]
    fn test_person_retrieve() {
        let api_key = std::env::var("PDL_API_KEY").unwrap();
        let client = PDLClient::new(&api_key, "v5");

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
    #[ignore = "Needs to POST"]
    fn test_person_bulk_retriev() {
        let api_key = std::env::var("PDL_API_KEY").unwrap();
        let client = PDLClient::new(&api_key, "v5");

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
        assert_eq!(resp.status, 200);
    }
}
