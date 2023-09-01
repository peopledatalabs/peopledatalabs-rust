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
        let r = self.client.get::<IPResponse>(PATH, &qs)?;

        Ok(r)
    }
}

#[cfg(test)]
mod tests {
    use crate::{client::PDLClient, BaseParams, IPBaseParams, IPParams};

    use super::IP;

    #[test]
    fn test_ip_get() {
        let api_key = std::env::var("PDL_API_KEY").unwrap();
        let client = PDLClient::new(&api_key);

        let ip = IP { client };

        let mut base_params = BaseParams::default();
        base_params.pretty = Some(true);

        let mut ip_base_params = IPBaseParams::default();
        ip_base_params.ip = Some("72.212.42.169".to_string());

        let ip_params = IPParams {
            base_params: Some(base_params),
            ip_base_params,
        };

        let resp = ip.get(ip_params).expect("ERROR");

        assert_eq!(resp.status, 200);
        assert_eq!(resp.data.ip.address, "72.212.42.169");
    }
}
