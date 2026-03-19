use crate::{
    models::changelog::{ChangelogPersonParams, ChangelogPersonResponse},
    PDLClient, PDLError,
};

pub(crate) static PERSON_CHANGELOG_PATH: &str = "/person/changelog";

pub struct Changelog {
    pub client: PDLClient,
}

impl Changelog {
    pub fn get_person(
        &self,
        params: ChangelogPersonParams,
    ) -> Result<ChangelogPersonResponse, PDLError> {
        params.validate()?;
        self.client
            .post::<ChangelogPersonResponse, ChangelogPersonParams>(
                PERSON_CHANGELOG_PATH,
                params,
            )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        client::{PDLCLientOptions, PDLClient, PDLError},
        models::changelog::ChangelogPersonParams,
    };

    use super::Changelog;

    #[test]
    fn test_person_changelog() {
        let api_key = std::env::var("PDL_API_KEY").unwrap();
        let client = PDLClient::new(&api_key).build();

        let changelog = Changelog { client };

        let params = ChangelogPersonParams {
            origin_version: Some("33.1".to_string()),
            current_version: Some("33.2".to_string()),
            type_: Some("updated".to_string()),
            ..Default::default()
        };

        let resp = changelog.get_person(params).expect("ERROR");

        assert!(resp.error.is_none());
        assert!(resp.data.is_some());
    }

    #[test]
    fn test_person_changelog_sandbox() {
        let api_key = std::env::var("PDL_API_KEY").unwrap();
        let mut client_options = PDLCLientOptions::default();
        client_options.sandbox = true;
        let client = PDLClient::new(&api_key).options(client_options).build();

        let changelog = Changelog { client };

        let params = ChangelogPersonParams {
            origin_version: Some("33.1".to_string()),
            current_version: Some("33.2".to_string()),
            type_: Some("updated".to_string()),
            ..Default::default()
        };

        match changelog.get_person(params) {
            Ok(resp) => {
                assert!(resp.error.is_none());
                assert!(resp.data.is_some());
            }
            Err(PDLError::HTTPError(status)) if status.as_u16() == 404 => {
                // Sandbox does not support the changelog endpoint
            }
            Err(e) => panic!("ERROR: {:?}", e),
        }
    }
}
