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
        self.client
            .get::<CleanSchoolResponse, CleanSchoolParams>(PATH, params)
    }
}

#[cfg(test)]
mod tests {
    use crate::{client::PDLClient, BaseParams, CleanSchoolParams, SchoolParams};

    use super::School;

    #[test]
    fn test_school_clean() {
        let api_key = std::env::var("PDL_API_KEY").unwrap();
        let client = PDLClient::new(&api_key).build();

        let school = School { client };

        let mut base_params = BaseParams::default();
        base_params.pretty = Some(true);

        let mut school_params = SchoolParams::default();
        school_params.profile = Some("linkedin.com/school/ucla".to_string());

        let clean_school_params = CleanSchoolParams {
            base_params: Some(base_params),
            school_params,
            additional_params: None,
        };

        let resp = school.clean(clean_school_params).expect("ERROR");

        assert_eq!(resp.status, 200);
        assert_eq!(
            resp.name,
            Some("university of california, los angeles".to_string())
        );
    }
}
