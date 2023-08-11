use crate::{
    models::ip::{IPParams, IPResponse},
    PDLClient, PDLError,
};

pub(crate) static PATH: &str = "/ip/enrich";

pub struct IP {
    pub client: PDLClient,
}

impl IP {
    pub fn get(&self, params: IPParams) -> Result<IPResponse, PDLError> {
        params.validate()?;
        let qs = serde_qs::to_string(&params).map_err(|_| PDLError::ValidationError)?;
        let r: IPResponse = self.client.get(PATH, &qs)?;

        Ok(r)
    }
}
