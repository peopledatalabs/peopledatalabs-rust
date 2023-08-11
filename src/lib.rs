mod api;
mod client;
mod models;

use api::{Company, JobTitle, Location, Person, School, Skill, IP};
use client::{PDLClient, PDLError};

pub use models::{
    AutocompleteParams, BaseParams, BulkEnrichPersonParams, BulkEnrichSinglePersonParams,
    BulkRetrievePersonParams, BulkRetrieveSinglePersonParams, CleanCompanyParams,
    CleanLocationParams, CleanSchoolParams, CompanyParams, EnrichCompanyParams, EnrichPersonParams,
    IPBaseParams, IPParams, IdentifyPersonParams, JobTitleBaseParams, JobTitleParams,
    LocationParams, PersonMetadata, PersonParams, RetrievePersonParams, SchoolParams,
    SearchBaseParams, SearchParams, SkillBaseParams, SkillParams,
};

pub struct PDL {
    pub company: Company,
    pub ip: IP,
    pub job_title: JobTitle,
    pub location: Location,
    pub person: Person,
    pub school: School,
    pub skill: Skill,
}

impl PDL {
    pub fn new() -> PDL {
        let api_key = std::env::var("PDL_API_KEY").unwrap();
        let client = PDLClient::new(&api_key, "v5");

        PDL {
            company: Company {
                client: client.clone(),
            },
            ip: IP {
                client: client.clone(),
            },
            job_title: JobTitle {
                client: client.clone(),
            },
            location: Location {
                client: client.clone(),
            },
            person: Person {
                client: client.clone(),
            },
            school: School {
                client: client.clone(),
            },
            skill: Skill {
                client: client.clone(),
            },
        }
    }
}
