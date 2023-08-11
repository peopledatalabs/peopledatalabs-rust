use peopledatalabs::{BaseParams, CleanLocationParams, LocationParams, PDL};

fn main() {
    let client = PDL::new();
    let base_params = BaseParams::default();
    let location_params = LocationParams {
        location: "New York, NY",
    };
    let params = CleanLocationParams {
        base_params,
        location_params,
        additional_params: None,
    };

    let results = client.location.clean(params);

    println!("{:#?}", results);
}
