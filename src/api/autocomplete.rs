use crate::{
    client::{PDLClient, PDLError},
    models::autocomplete::AutocompleteResponse,
    AutocompleteParams,
};

pub struct Autocomplete {
    pub client: PDLClient,
}

pub(crate) static AUTOCOMPLETE_PATH: &str = "/autocomplete";

impl Autocomplete {
    /// Autocomplete allows your users to get suggestions for Search API query values
    /// along with the number of available records for each suggestion.
    /// For example, schools starting with "stanf".
    pub fn autocomplete(
        &self,
        params: AutocompleteParams,
    ) -> Result<AutocompleteResponse, PDLError> {
        params.validate()?;
        self.client
            .get::<AutocompleteResponse, AutocompleteParams>(AUTOCOMPLETE_PATH, params)
    }
}

#[cfg(test)]
mod tests {
    use crate::{client::PDLClient, AutocompleteBaseParams, AutocompleteParams, BaseParams};

    use super::Autocomplete;

    #[test]
    fn test_autocomplete() {
        let api_key = std::env::var("PDL_API_KEY").unwrap();
        let client = PDLClient::new(&api_key).build();

        let autocomplete = Autocomplete { client };

        let mut base_params = BaseParams::default();
        base_params.pretty = Some(true);
        base_params.size = Some(10);

        let autocomplete_base_params = AutocompleteBaseParams {
            field: "school".to_string(),
            text: Some("stanf".to_string()),
            titlecase: Some(false),
        };

        let autocomplete_params = AutocompleteParams {
            base_params: Some(base_params),
            autocomplete_base_params,
        };

        let resp = autocomplete
            .autocomplete(autocomplete_params)
            .expect("ERROR");
        assert_eq!(resp.status, 200);
    }
}
