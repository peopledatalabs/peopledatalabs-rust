<p align="center">
<img src="https://i.imgur.com/S7DkZtr.png" width="250" alt="People Data Labs Logo">
</p>
<h1 align="center">People Data Labs Rust Client</h1>
<p align="center">Official Rust client for the People Data Labs API.</p>

<p align="center">
  <a href="https://github.com/peopledatalabs/peopledatalabs-rs">
    <img src="https://img.shields.io/badge/repo%20status-Active-limegreen" alt="Repo Status">
  </a>&nbsp;
  <a href="https://github.com/peopledatalabs/peopledatalabs-rs/actions/workflows/test.yaml">
    <img src="https://github.com/peopledatalabs/peopledatalabs-rs/actions/workflows/test.yaml/badge.svg" alt="Tests Status" />
  </a>
</p>

## Table of Contents

- [🔧 Installation](#installation)
- [🚀 Usage](#usage)
- [🏝 Sandbox Usage](#sandbox)
- [🌐 Endpoints](#endpoints)
- [📘 Documentation](#documentation)

## 🔧 Installation <a name="installation"></a>

1. To use _peopledatalabs-rs_ SDK in your project initialize your Rust crate then run:

```bash
cargo add peopledatalabs-rs
```
2. Sign up for a [free PDL API key](https://www.peopledatalabs.com/signup).
3. Set your API key as a environment variable.

## 🚀 Usage <a name="usage"></a>

First, create the PeopleDataLabs client:

```rust
```
Then, send requests to any PDL API Endpoint.

### Person Data

#### Enrichment

```rust
```

#### Bulk Enrichment

```rust
```

#### Search (Elasticsearch)

```rust
```

#### Search (SQL)

```rust
```

#### `PDL_ID` (Retrieve API)

```rust
```

#### Bulk Retrieve API

```rust
```

#### Fuzzy Enrichment (Identify API)

```rust
```

### Company Data

#### Enrichment

```rust
```

#### Search (Elasticsearch)

```rust
```

#### Search (SQL)

```rust
```

### Supporting APIs

#### Get Autocomplete Suggestions

```rust
```

#### Clean Raw Company Strings

```rust
```

#### Clean Raw Location Strings

```rust
```

#### Clean Raw School Strings

```rust
```

#### Enrich Job Title

```rust
```

#### Enrich Skill

```rust
```

#### Enrich IP

```rust
```

## 🏝 Sandbox Usage <a name="sandbox"></a>
```rust
// To enable sandbox usage, use the following
```

## 🌐 Endpoints <a name="endpoints"></a>

**Person Endpoints**

| API Endpoint                                                                           | SDK Function                         |
|----------------------------------------------------------------------------------------|--------------------------------------|
| [Person Enrichment API](https://docs.peopledatalabs.com/docs/enrichment-api)           | `client.Person.Enrich(params)`       |
| [Person Bulk Enrichment API](https://docs.peopledatalabs.com/docs/bulk-enrichment-api) | `client.Person.BulkEnrich(params)`   |
| [Person Search API](https://docs.peopledatalabs.com/docs/search-api)                   | `client.Person.Search(params)`       |
| [Person Retrieve API](https://docs.peopledatalabs.com/docs/person-retrieve-api)        | `client.Person.Retrieve(params)`     |
| [Person Bulk Retrieve API](https://docs.peopledatalabs.com/docs/bulk-person-retrieve)  | `client.Person.BulkRetrieve(params)` |
| [Person Identify API](https://docs.peopledatalabs.com/docs/identify-api)               | `client.Person.Identify(params)`     |

**Company Endpoints**

| API Endpoint                                                                          | SDK Function                    |
| ------------------------------------------------------------------------------------- |---------------------------------|
| [Company Enrichment API](https://docs.peopledatalabs.com/docs/company-enrichment-api) | `client.Company.Enrich(params)` |
| [Company Search API](https://docs.peopledatalabs.com/docs/company-search-api)         | `client.Company.Search(params)` |

**Supporting Endpoints**

| API Endpoint                                                                            | SDK Function                    |
| --------------------------------------------------------------------------------------- |---------------------------------|
| [Autocomplete API](https://docs.peopledatalabs.com/docs/autocomplete-api)               | `client.Autocomplete(params)`   |
| [Company Cleaner API](https://docs.peopledatalabs.com/docs/cleaner-apis#companyclean)   | `client.Company.Clean(params)`  |
| [Location Cleaner API](https://docs.peopledatalabs.com/docs/cleaner-apis#locationclean) | `client.Location.Clean(params)` |
| [School Cleaner API](https://docs.peopledatalabs.com/docs/cleaner-apis#schoolclean)     | `client.School.Clean(params)`   |
| [Job Title Enrichment API](https://docs.peopledatalabs.com/docs/job-title-enrichment-api) | `client.JobTitle(params)` |
| [Skill Enrichment API](https://docs.peopledatalabs.com/docs/skill-enrichment-api) | `client.Skill(params)` |
| [IP Enrichment API](https://docs.peopledatalabs.com/docs/ip-enrichment-api) | `client.IP(params)` |

## 📘 Documentation <a name="documentation"></a>

All of our API endpoints are documented at: https://docs.peopledatalabs.com/

These docs describe the supported input parameters, output responses and also provide additional technical context.

As illustrated in the [Endpoints](#endpoints) section above, each of our API endpoints is mapped to a specific method in the API Client. For each of these class methods, **all function inputs are mapped as input parameters to the respective API endpoint**, meaning that you can use the API documentation linked above to determine the input parameters for each endpoint.
