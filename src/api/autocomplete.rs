use crate::{client::{PDLClient, PDLError}, AutocompleteParams, models::autocomplete::AutocompleteResponse};

pub struct Autocomplete {
    pub client: PDLClient,
}

pub(crate) static AUTOCOMPLETE_PATH: &str = "/autocomplete";

impl Autocomplete {
    /// Autocomplete allows your users to get suggestions for Search API query values
    /// along with the number of available records for each suggestion.
    /// For example, schools starting with "stanf".
    pub fn autocomplete(&self, params: AutocompleteParams) -> Result<AutocompleteResponse, PDLError> {
        params.validate()?;
        let qs = serde_qs::to_string(&params).map_err(|_| PDLError::ValidationError)?;
        let r = self.client.get::<AutocompleteResponse>(AUTOCOMPLETE_PATH, &qs)?;

        Ok(r)
    }
}
