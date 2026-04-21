use crate::{
    models::jobposting::{JobPostingSearchParams, SearchJobPostingResponse},
    PDLClient, PDLError,
};

pub(crate) static JOB_POSTING_SEARCH_PATH: &str = "/job_posting/search";

pub struct JobPosting {
    pub client: PDLClient,
}

impl JobPosting {
    /// Searches PDL's job_posting dataset.
    /// docs: https://docs.peopledatalabs.com/docs/job-posting-search-api
    pub fn search(
        &self,
        params: JobPostingSearchParams,
    ) -> Result<SearchJobPostingResponse, PDLError> {
        params.validate()?;
        self.client
            .post::<SearchJobPostingResponse, JobPostingSearchParams>(
                JOB_POSTING_SEARCH_PATH,
                params,
            )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        client::PDLClient,
        models::jobposting::{
            JobPostingSearchBaseParams, JobPostingSearchParams, RemoteWorkPolicy,
        },
        BaseParams,
    };

    use super::JobPosting;

    #[test]
    fn test_job_posting_search_query() {
        let api_key = std::env::var("PDL_API_KEY").unwrap();
        let client = PDLClient::new(&api_key).build();

        let job_posting = JobPosting { client };

        let mut base_params = BaseParams::default();
        base_params.size = Some(3);

        let query = serde_json::json!({
            "query": {
                "bool": {
                    "must": [
                        { "term": { "title_role": "engineering" } },
                    ]
                }
            }
        });

        let search_base_params = JobPostingSearchBaseParams {
            query: Some(query),
            ..JobPostingSearchBaseParams::default()
        };

        let params = JobPostingSearchParams {
            base_params: Some(base_params),
            search_base_params,
        };

        let resp = job_posting.search(params).expect("ERROR");

        assert_eq!(resp.status, 200);
        assert!(resp.data.is_some());
    }

    #[test]
    fn test_job_posting_search_params() {
        let api_key = std::env::var("PDL_API_KEY").unwrap();
        let client = PDLClient::new(&api_key).build();

        let job_posting = JobPosting { client };

        let mut base_params = BaseParams::default();
        base_params.size = Some(3);

        let search_base_params = JobPostingSearchBaseParams {
            title_role: Some("engineering".to_string()),
            remote_work_policy: Some(RemoteWorkPolicy::Remote),
            ..JobPostingSearchBaseParams::default()
        };

        let params = JobPostingSearchParams {
            base_params: Some(base_params),
            search_base_params,
        };

        let resp = job_posting.search(params).expect("ERROR");

        assert_eq!(resp.status, 200);
        assert!(resp.data.is_some());
    }

    #[test]
    fn test_job_posting_search_size_out_of_range() {
        let api_key = std::env::var("PDL_API_KEY").unwrap();
        let client = PDLClient::new(&api_key).build();

        let job_posting = JobPosting { client };

        let mut base_params = BaseParams::default();
        base_params.size = Some(101);

        let search_base_params = JobPostingSearchBaseParams {
            title: Some("engineer".to_string()),
            ..JobPostingSearchBaseParams::default()
        };

        let params = JobPostingSearchParams {
            base_params: Some(base_params),
            search_base_params,
        };

        let resp = job_posting.search(params);
        assert!(resp.is_err());
    }

    #[test]
    fn test_job_posting_is_active_serialization_opt_in() {
        // None must be omitted entirely; Some(false) must round-trip.
        let mut params = JobPostingSearchBaseParams::default();
        params.title = Some("engineer".to_string());
        let body = serde_json::to_string(&params).unwrap();
        assert!(!body.contains("is_active"));

        params.is_active = Some(false);
        let body = serde_json::to_string(&params).unwrap();
        assert!(body.contains("\"is_active\":false"));
    }

    #[test]
    fn test_job_posting_scroll_token_round_trips_verbatim() {
        let token = "eyJhIjogMX0=".to_string();
        let mut params = JobPostingSearchBaseParams::default();
        params.scroll_token = Some(token.clone());
        let body = serde_json::to_string(&params).unwrap();
        assert!(body.contains(&format!("\"scroll_token\":\"{}\"", token)));
    }
}
