use peopledatalabs::{BaseParams, IPBaseParams, IPParams, PDL};

fn main() {
    let client = PDL::new();
    let base_params = BaseParams::default();
    let mut ip_base_params = IPBaseParams::default();
    ip_base_params.ip = Some("72.212.42.169".to_string());
    let params = IPParams {
        base_params,
        ip_base_params,
    };

    let results = client.ip.get(params);

    println!("{:#?}", results);
}
