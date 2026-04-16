use peopledatalabs::{
    BaseParams, JobPostingSearchBaseParams, JobPostingSearchParams, RemoteWorkPolicy, PDL,
};

fn main() {
    let client = PDL::new();

    let mut base_params = BaseParams::default();
    base_params.size = Some(10);

    // By Search (Field Parameters)
    let search_base_params = JobPostingSearchBaseParams {
        title_role: Some("engineering".to_string()),
        remote_work_policy: Some(RemoteWorkPolicy::Remote),
        is_active: Some(true),
        ..JobPostingSearchBaseParams::default()
    };

    let params = JobPostingSearchParams {
        base_params: Some(base_params),
        search_base_params,
    };

    let results = client.job_posting.search(params);

    println!("{:#?}", results);
}
