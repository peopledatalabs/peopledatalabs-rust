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
        let r: JobTitleResponse = self.client.get(PATH, &qs)?;

        Ok(r)
    }
}
