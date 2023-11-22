use crate::{
    models::skills::{SkillParams, SkillResponse},
    PDLClient, PDLError,
};

pub(crate) static PATH: &str = "/skill/enrich";

pub struct Skill {
    pub client: PDLClient,
}

impl Skill {
    pub fn get(&self, params: SkillParams) -> Result<SkillResponse, PDLError> {
        params.validate()?;
        self.client.get::<SkillResponse, SkillParams>(PATH, params)
    }
}

#[cfg(test)]
mod tests {
    use crate::{client::PDLClient, BaseParams, SkillBaseParams, SkillParams};

    use super::Skill;

    #[test]
    fn test_skill_get() {
        let api_key = std::env::var("PDL_API_KEY").unwrap();
        let client = PDLClient::new(&api_key).build();

        let skill = Skill { client };

        let mut base_params = BaseParams::default();
        base_params.pretty = Some(true);

        let skill_base_params = SkillBaseParams {
            skill: Some("rust".to_string()),
            titlecase: Some(false),
        };

        let skill_params = SkillParams {
            base_params: Some(base_params),
            skill_base_params,
        };

        let resp = skill.get(skill_params).expect("ERROR");

        assert_eq!(resp.status, 200);
        assert_eq!(resp.data.cleaned_skill, "rust".to_string());
    }
}
