use peopledatalabs::{CleanSchoolParams, SchoolParams, PDL};

fn main() {
    let client = PDL::new();
    let mut school_params = SchoolParams::default();
    school_params.name = Some("UConn".to_string());
    let params = CleanSchoolParams {
        base_params: None,
        school_params,
        additional_params: None,
    };

    let results = client.school.clean(params);

    println!("{:#?}", results);
}
