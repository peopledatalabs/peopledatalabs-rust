use crate::{
    models::jobtitle::{JobTitleParams, JobTitleResponse},
    PDLClient, PDLError,
};

pub(crate) static PATH: &str = "/job_title/enrich";

pub struct JobTitle {
    pub client: PDLClient,
}

impl JobTitle {
    pub fn get(&self, params: JobTitleParams) -> Result<JobTitleResponse, PDLError> {
        params.validate()?;
        let qs = serde_qs::to_string(&params).map_err(|_| PDLError::ValidationError)?;
        let r = self.client.get::<JobTitleResponse>(PATH, &qs)?;

        Ok(r)
    }
}

#[cfg(test)]
mod tests {
    use crate::{client::PDLClient, BaseParams, JobTitleBaseParams, JobTitleParams};

    use super::JobTitle;

    #[test]
    fn test_job_title_get() {
        let api_key = std::env::var("PDL_API_KEY").unwrap();
        let client = PDLClient::new(&api_key);

        let job_title = JobTitle { client };

        let mut base_params = BaseParams::default();
        base_params.pretty = Some(true);

        let job_title_base_params = JobTitleBaseParams {
            job_title: Some("data scientist".to_string()),
        };

        let job_title_params = JobTitleParams {
            base_params: Some(base_params),
            job_title_base_params,
        };

        let resp = job_title.get(job_title_params).expect("ERROR");

        assert_eq!(resp.status, 200);
        assert_eq!(resp.data.cleaned_job_title, "data scientist".to_string());
    }
}
