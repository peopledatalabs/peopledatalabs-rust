use peopledatalabs::{JobTitleBaseParams, JobTitleParams, PDL};

fn main() {
    let client = PDL::new();
    let job_title_base_params = JobTitleBaseParams {
        job_title: Some("software engineer".to_string()),
    };
    let params = JobTitleParams {
        base_params: None,
        job_title_base_params,
    };

    let results = client.job_title.get(params);

    println!("{:#?}", results);
}
