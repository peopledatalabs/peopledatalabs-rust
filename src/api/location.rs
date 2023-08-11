use crate::{
    models::location::{CleanLocationParams, CleanLocationResponse},
    PDLClient, PDLError,
};

pub(crate) static PATH: &str = "/location/clean";

pub struct Location {
    pub client: PDLClient,
}

impl Location {
    pub fn clean(&self, params: CleanLocationParams) -> Result<CleanLocationResponse, PDLError> {
        params.validate()?;
        let qs = serde_qs::to_string(&params).map_err(|_| PDLError::ValidationError)?;
        let r: CleanLocationResponse = self.client.get(PATH, &qs)?;

        Ok(r)
    }
}
