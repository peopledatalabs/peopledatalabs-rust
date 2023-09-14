<p align="center">
<img src="https://i.imgur.com/S7DkZtr.png" width="250" alt="People Data Labs Logo">
</p>
<h1 align="center">People Data Labs Rust Client</h1>
<p align="center">Official Rust client for the People Data Labs API.</p>

<p align="center">
  <a href="https://github.com/peopledatalabs/peopledatalabs">
    <img src="https://img.shields.io/badge/repo%20status-Active-limegreen" alt="Repo Status">
  </a>&nbsp;
  <a href="https://github.com/peopledatalabs/peopledatalabs-rust/actions/workflows/test.yml">
    <img src="https://github.com/peopledatalabs/peopledatalabs-rust/actions/workflows/test.yml/badge.svg" alt="Tests Status" />
  </a>
</p>

## Table of Contents

- [🔧 Installation](#installation)
- [🚀 Usage](#usage)
- [🏝 Sandbox Usage](#sandbox)
- [🌐 Endpoints](#endpoints)
- [📘 Documentation](#documentation)

## 🔧 Installation <a name="installation"></a>

1. To use peopledatalabs SDK in your project initialize your Rust crate then run:

```bash
cargo add peopledatalabs
```

2. Sign up for a [free PDL API key](https://www.peopledatalabs.com/signup).
3. Set your API key as a environment variable.

## 🚀 Usage <a name="usage"></a>

First, create the PeopleDataLabs client:

```rust
let api_key = std::env::var("PDL_API_KEY").unwrap();
let client = PDL::new(&api_key);
```

Then, send requests to any PDL API Endpoint.

### Person Data

#### Enrichment

```rust
let mut person_params = PersonParams::default();
person_params.name = Some(vec!["josh finnie".to_string()]);
person_params.location = Some(vec!["washington, dc".to_string()]);

let mut enrich_params = EnrichPersonParams::default();
enrich_params.person_params = person_params.clone();

let results = client.person.enrich(enrich_params);
```

#### Bulk Enrichment

```rust
let mut person_params = PersonParams::default();
person_params.name = Some(vec!["josh finnie".to_string()]);
person_params.location = Some(vec!["washington, dc".to_string()]);

let request = BulkEnrichSinglePersonParams {
    params: person_params.clone(),
    metadata: None,
};
let bulk_enrich_params = BulkEnrichPersonParams {
    requests: vec![request],
};

let results = client.person.bulk_enrich(bulk_enrich_params);
```

#### Search (Elasticsearch)

```rust
let mut search_base_params = SearchBaseParams::default();
search_base_params.query = Some(serde_json::value::Value::String(
    "{'bool':{'must': [{'term': {'job_title_role': 'health'}},]}}".to_string(),
));

let mut search_params = SearchParams::default();
search_params.search_base_params = search_base_params;

let search_results = client.person.search(search_params);
```

#### Search (SQL)

```rust
let query = r#"
    SELECT * FROM person
        WHERE location_country='mexico'
        AND job_title_role='health'
        AND phone_numbers IS NOT NULL;
"#.to_string();
let mut search_base_params = SeachBaseParams::default();
search_base_params.sql = query;

let mut seach_params = SearchParams::default();
search_params.search_base_params = search_base_params;

let search_results = client.person.searach(seach_params);
```

#### `PDL_ID` (Retrieve API)

```rust
let retrieve_person_params = RetrievePersonParams {
    base_params: None,
    person_id: "82MYIGZzMttzdyKiQBv4ZQ_0000".to_string(),
};

let retrieve_results = client.person.retrieve(retrieve_person_params);
```

#### Bulk Retrieve API

```rust
let retrieve_request = BulkRetrieveSinglePersonParams {
    id: "82MYIGZzMttzdyKiQBv4ZQ_0000".to_string(),
    metadata: None,
};

let mut bulk_retrieve_params = BulkRetrievePersonParams::default();
bulk_retrieve_params.requests = vec![retrieve_request];

let bulk_retrieve_results = client.person.bulk_retrieve(bulk_retrieve_params);
```

#### Fuzzy Enrichment (Identify API)

```rust
let mut person_params = PersonParams::default();
person_params.name = Some(vec!["josh finnie".to_string()]);
person_params.location = Some(vec!["washington, dc".to_string()]);

let mut identify_params = IdentifyPersonParams::default();
identify_params.person_params = person_params.clone();

let identify_results = client.person.identify(identify_params);
```

### Company Data

#### Enrichment

```rust
let mut company_params = CompanyParams::default();
company_params.name = Some("google".to_string());

let enrich_params = EnrichCompanyParams {
    base_params: None,
    company_params,
    additional_params: None,
};

let enrich_request = client.company.enrich(enrich_params);
```

#### Search (Elasticsearch)

```rust
let mut search_base_params = SearchBaseParams::default();
search_base_params.query = Some(serde_json::value::Value::String(
    r#"{'query': {'bool': {'must': {
            {"term": {"tags": "bigdata"}},
            {"term": {"industry": "financial services"}},
            {"term": {"location.country": "united states"}},
        },},},
    }"#.to_string(),
));

let mut search_params = SearchParams::default();
search_params.search_base_params = search_base_params;

let search_results = client.company.search(search_params);
```

#### Search (SQL)

```rust
let mut search_base_params = SearchBaseParams::default();
search_base_params.sql = Some("SELECT * FROM company WHERE website='google.com';".to_string());

let search_params = SearchParams {
    base_params: None,
    search_base_params,
    additional_params: None,
};

let search_results = client.company.search(search_params);
```

### Supporting APIs

#### Get Autocomplete Suggestions

```rust
let autocomplete_base_params = AutocompleteBaseParams{
    field: "text".to_string(),
    text: Some("full".to_string())
};
let autocomplete_params = AutocompleteParams {
    base_params: None,
    autocomplete_base_params,
};

let request = client.autocomplete.autocomplete(autocomplete_params);
```

#### Clean Raw Company Strings

```rust
let mut clean_params = CleanCompanyParams::default();
clean_params.name = Some("google".to_string());

let clean_results = client.company.clean(clean_params);
```

#### Clean Raw Location Strings

```rust
let base_params = BaseParams::default();
let location_params = LocationParams {
    location: Some("New York, NY".to_string()),
};
let params = CleanLocationParams {
    base_params: None,
    location_params,
    additional_params: None,
};

let results = client.location.clean(params);
```

#### Clean Raw School Strings

```rust
let mut school_params = SchoolParams::default();
school_params.name = Some("UConn".to_string());
let params = CleanSchoolParams {
    base_params: None,
    school_params,
    additional_params: None,
};

let results = client.school.clean(params);
```

#### Enrich Job Title

```rust
let job_title_base_params = JobTitleBaseParams {
    job_title: Some("software engineer".to_string()),
};
let params = JobTitleParams {
    base_params: None,
    job_title_base_params,
};

let results = client.job_title.get(params);
```

#### Enrich Skill

```rust
let skill_base_params = SkillBaseParams {
    skill: Some("python".to_string()),
};
let params = SkillParams {
    base_params: None,
    skill_base_params,
};

let results = client.skill.get(params);
```

#### Enrich IP

```rust
let mut ip_base_params = IPBaseParams::default();
ip_base_params.ip = Some("72.212.42.169".to_string());
let params = IPParams {
    base_params: None,
    ip_base_params,
};

let results = client.ip.get(params);
```

## 🏝 Sandbox Usage <a name="sandbox"></a>
```rust
// To enable sandbox usage, pass in the following options to the PDLClient before building
let api_key = std::env::var("PDL_API_KEY").unwrap();
let mut client_options = PDLCLientOptions::default();
client_options.sandbox = true;
let client = PDLClient::new(&api_key).options(client_options).build();
```

## 🌐 Endpoints <a name="endpoints"></a>

**Person Endpoints**

| API Endpoint                                                                           | SDK Function                         |
|----------------------------------------------------------------------------------------|--------------------------------------|
| [Person Enrichment API](https://docs.peopledatalabs.com/docs/enrichment-api)           | `client.person.enrich(params)`       |
| [Person Bulk Enrichment API](https://docs.peopledatalabs.com/docs/bulk-enrichment-api) | `client.person.bulk_enrich(params)`   |
| [Person Search API](https://docs.peopledatalabs.com/docs/search-api)                   | `client.person.search(params)`       |
| [Person Retrieve API](https://docs.peopledatalabs.com/docs/person-retrieve-api)        | `client.person.retrieve(params)`     |
| [Person Bulk Retrieve API](https://docs.peopledatalabs.com/docs/bulk-person-retrieve)  | `client.person.bulk_retrieve(params)` |
| [Person Identify API](https://docs.peopledatalabs.com/docs/identify-api)               | `client.person.identify(params)`     |

**Company Endpoints**

| API Endpoint                                                                          | SDK Function                    |
| ------------------------------------------------------------------------------------- |---------------------------------|
| [Company Enrichment API](https://docs.peopledatalabs.com/docs/company-enrichment-api) | `client.company.enrich(params)` |
| [Company Search API](https://docs.peopledatalabs.com/docs/company-search-api)         | `client.company.search(params)` |

**Supporting Endpoints**

| API Endpoint                                                                            | SDK Function                    |
| --------------------------------------------------------------------------------------- |---------------------------------|
| [Autocomplete API](https://docs.peopledatalabs.com/docs/autocomplete-api)               | `client.autocomplete.autocomplete(params)`   |
| [Company Cleaner API](https://docs.peopledatalabs.com/docs/cleaner-apis#companyclean)   | `client.company.clean(params)`  |
| [Location Cleaner API](https://docs.peopledatalabs.com/docs/cleaner-apis#locationclean) | `client.location.clean(params)` |
| [School Cleaner API](https://docs.peopledatalabs.com/docs/cleaner-apis#schoolclean)     | `client.school.clean(params)`   |
| [Job Title Enrichment API](https://docs.peopledatalabs.com/docs/job-title-enrichment-api) | `client.job_title.enrich(params)` |
| [Skill Enrichment API](https://docs.peopledatalabs.com/docs/skill-enrichment-api) | `client.skill.enrich(params)` |
| [IP Enrichment API](https://docs.peopledatalabs.com/docs/ip-enrichment-api) | `client.ip.enrich(params)` |

## 📘 Documentation <a name="documentation"></a>

All of our API endpoints are documented at: https://docs.peopledatalabs.com/

These docs describe the supported input parameters, output responses and also provide additional technical context.

As illustrated in the [Endpoints](#endpoints) section above, each of our API endpoints is mapped to a specific method in the API Client. For each of these class methods, **all function inputs are mapped as input parameters to the respective API endpoint**, meaning that you can use the API documentation linked above to determine the input parameters for each endpoint.
