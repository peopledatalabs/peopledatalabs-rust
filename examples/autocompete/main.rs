use peopledatalabs::{PDL, AutocompleteBaseParams, AutocompleteParams};

fn main() {
    let client = PDL::new();
    let autocomplete_base_params = AutocompleteBaseParams{
        field: "text".to_string(),
        text: Some("full".to_string())
    };
    let autocomplete_params = AutocompleteParams {
        base_params: None,
        autocomplete_base_params,
    };

    let request = client.autocomplete.autocomplete(autocomplete_params);

    println!("{:#?}", request);
}
