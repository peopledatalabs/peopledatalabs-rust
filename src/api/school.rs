use crate::{
    models::school::{CleanSchoolParams, CleanSchoolResponse},
    PDLClient, PDLError,
};

pub(crate) static PATH: &str = "/school/clean";

pub struct School {
    pub client: PDLClient,
}

impl School {
    pub fn clean(&self, params: CleanSchoolParams) -> Result<CleanSchoolResponse, PDLError> {
        params.validate()?;
        let qs = serde_qs::to_string(&params).map_err(|_| PDLError::ValidationError)?;
        let r: CleanSchoolResponse = self.client.get(PATH, &qs)?;

        Ok(r)
    }
}
