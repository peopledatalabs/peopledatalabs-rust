use peopledatalabs::{CleanLocationParams, LocationParams, PDL};

fn main() {
    let client = PDL::new();
    let location_params = LocationParams {
        location: Some("New York, NY".to_string()),
    };
    let params = CleanLocationParams {
        base_params: None,
        location_params,
        additional_params: None,
    };

    let results = client.location.clean(params);

    println!("{:#?}", results);
}
