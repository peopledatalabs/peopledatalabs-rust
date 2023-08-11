use peopledatalabs::{
    BulkEnrichPersonParams, BulkEnrichSinglePersonParams, BulkRetrievePersonParams,
    BulkRetrieveSinglePersonParams, EnrichPersonParams, IdentifyPersonParams, PersonParams,
    RetrievePersonParams, SearchBaseParams, SearchParams, PDL,
};

fn main() {
    let client = PDL::new();
    let mut person_params = PersonParams::default();
    person_params.name = Some(vec!["josh finnie".to_string()]);
    person_params.location = Some(vec!["washington, dc".to_string()]);

    // Enrich
    let mut enrich_params = EnrichPersonParams::default();
    enrich_params.person_params = person_params.clone();

    let enrich_results = client.person.enrich(enrich_params);

    println!("{:#?}", enrich_results);

    // Bulk Enrich
    let request = BulkEnrichSinglePersonParams {
        params: person_params.clone(),
        metadata: None,
    };
    let bulk_enrich_params = BulkEnrichPersonParams {
        requests: vec![request],
    };

    let bulk_enrich_results = client.person.bulk_enrich(bulk_enrich_params);

    println!("{:#?}", bulk_enrich_results);

    // Identify
    let mut identify_params = IdentifyPersonParams::default();
    identify_params.person_params = person_params.clone();

    let identify_results = client.person.identify(identify_params);

    println!("{:#?}", identify_results);

    // Search
    let mut search_base_params = SearchBaseParams::default();
    search_base_params.query = Some(serde_json::value::Value::String(
        "{'bool':{'must': [{'term': {'job_title_role': 'health'}},]}}".to_string(),
    ));

    let mut search_params = SearchParams::default();
    search_params.search_base_params = search_base_params;

    let search_results = client.person.search(search_params);

    println!("{:#?}", search_results);

    // Retrieve
    let retrieve_person_params = RetrievePersonParams {
        base_params: None,
        person_id: "82MYIGZzMttzdyKiQBv4ZQ_0000".to_string(),
    };

    let retrieve_results = client.person.retrieve(retrieve_person_params);

    println!("{:#?}", retrieve_results);

    // Bulk Retrieve
    let retrieve_request = BulkRetrieveSinglePersonParams {
        id: "82MYIGZzMttzdyKiQBv4ZQ_0000".to_string(),
        metadata: None,
    };

    let mut bulk_retrieve_params = BulkRetrievePersonParams::default();
    bulk_retrieve_params.requests = vec![retrieve_request];

    let bulk_retrieve_results = client.person.bulk_retrieve(bulk_retrieve_params);

    println!("{:#?}", bulk_retrieve_results);
}
