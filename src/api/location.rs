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
        let r = self.client.get::<CleanLocationResponse>(PATH, &qs)?;

        Ok(r)
    }
}

#[cfg(test)]
mod tests {
    use crate::{client::PDLClient, BaseParams, CleanLocationParams, LocationParams};

    use super::Location;

    #[test]
    fn test_location_clean() {
        let api_key = std::env::var("PDL_API_KEY").unwrap();
        let client = PDLClient::new(&api_key).build();

        let location = Location { client };

        let mut base_params = BaseParams::default();
        base_params.pretty = Some(true);

        let location_params = LocationParams {
            location: Some("portland".to_string()),
        };

        let clean_location_params = CleanLocationParams {
            base_params: Some(base_params),
            location_params,
            additional_params: None,
        };

        let resp = location.clean(clean_location_params).expect("ERROR");

        assert_eq!(resp.status, 200);
        assert_eq!(
            resp.name,
            Some("portland, oregon, united states".to_string())
        );
    }
}
