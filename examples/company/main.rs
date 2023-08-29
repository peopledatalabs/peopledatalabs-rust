use peopledatalabs::{
    CleanCompanyParams, CompanyParams, EnrichCompanyParams, SearchBaseParams, SearchParams, PDL,
};

fn main() {
    let client = PDL::new();
    let mut company_params = CompanyParams::default();
    company_params.name = Some("google".to_string());

    // Enrich
    let enrich_params = EnrichCompanyParams {
        base_params: None,
        company_params,
        additional_params: None,
    };

    let enrich_request = client.company.enrich(enrich_params);

    println!("{:#?}", enrich_request);

    // Clean
    let mut clean_params = CleanCompanyParams::default();
    clean_params.name = Some("google".to_string());

    let clean_results = client.company.clean(clean_params);

    println!("{:#?}", clean_results);

    // Search
    let mut search_base_params = SearchBaseParams::default();
    search_base_params.sql = Some("SELECT * FROM company WHERE website='google.com';".to_string());

    let search_params = SearchParams {
        base_params: None,
        search_base_params,
        additional_params: None,
    };

    let search_results = client.company.search(search_params);

    println!("{:#?}", search_results);
}
