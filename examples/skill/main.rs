use peopledatalabs::{SkillBaseParams, SkillParams, PDL};

fn main() {
    let client = PDL::new();
    let skill_base_params = SkillBaseParams {
        skill: Some("python".to_string()),
    };
    let params = SkillParams {
        base_params: None,
        skill_base_params,
    };

    let results = client.skill.get(params);

    println!("{:#?}", results);
}
