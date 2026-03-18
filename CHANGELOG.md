# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [2.2.0] - 2026-03-18

- Added employee_growth_rate_12_month_by_country to Company model (PDL API v33.2)
- Field is a HashMap of country code to EmployeeGrowthRate12MonthByCountry struct with current_headcount, 12_month_headcount, and 12_month_growth_rate
