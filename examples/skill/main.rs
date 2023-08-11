use peopledatalabs::{BaseParams, SkillBaseParams, SkillParams, PDL};

fn main() {
    let client = PDL::new();
    let base_params = BaseParams::default();
    let skill_base_params = SkillBaseParams {
        skill: Some("python".to_string()),
    };
    let params = SkillParams {
        base_params,
        skill_base_params,
    };

    let results = client.skill.get(params);

    println!("{:#?}", results);
}
