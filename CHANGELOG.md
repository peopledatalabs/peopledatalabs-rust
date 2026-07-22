# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [5.0.0] - 2026-07-22

- Added Job Posting API support: new `jobposting` endpoint, `JobPostingApi`, and request/response models
- Added PDL API v35.0 fields: `technologies_used` on Company, additional IP and Job Posting fields
- **Breaking:** many `IPInfo`/`IPMetadata`/`IPLocation`/`IPCompany`/`IPCompanyLocation`/`IPPerson` fields changed from required to `Option<T>` to reflect fields the API may omit
- **Breaking:** `IPPerson.job_title_subrole` now deserializes from `job_title_sub_role`; added `IPPerson.job_title_class` and `IPCompany.display_name`
- **Breaking:** `JobPosting.location: Option<String>` replaced with `locations: Option<Vec<JobPostingLocation>>`
- **Breaking:** `JobPosting.salary_range_min`/`salary_range_max` changed from `Option<i32>` to `Option<f64>` and now deserialize from `salary_min`/`salary_max`
- **Breaking:** `JobPosting.inferred_skills` changed from `Option<Vec<String>>` to `Option<Vec<JobPostingSkill>>`
- `error.type` fields on Company/Person/Changelog errors now accept either a single string or an array of strings from the API

## [4.2.0] - 2026-03-18

- Added employee_growth_rate_12_month_by_country to Company model (PDL API v33.2)
- Field is a HashMap of country code to EmployeeGrowthRate12MonthByCountry struct with current_headcount, 12_month_headcount, and 12_month_growth_rate
