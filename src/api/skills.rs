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
        let qs = serde_qs::to_string(&params).map_err(|_| PDLError::ValidationError)?;
        let r: SkillResponse = self.client.get(PATH, &qs)?;

        Ok(r)
    }
}
