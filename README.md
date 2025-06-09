# GITHUB ISSUE ROULETTE

This little program fetches all issues of a github project, filters them by issues that you can define and prints randomly one of that list.
I saw myself skipping issues, because there where easyer ones to do, so difficult ones where untouched for eternity.
This tool solves the problem for me, because it shows select an issue for me, that i HAVE to do.

## Usage

The username or organizationname and the repository are required:
`./github_issue_roulette embassy-rs embassy`

To include label 'kind-feature' and exclude label 'e-stm32':
`./github_issue_roulette embassy-rs embassy --incl-labels kind-feature,e-cyw43 --excl-labels e-stm32`

## Installation

Pick a release, download the binary and GOGOGO

### Building from source

Clone the repo and build with cargo: `cargo build -r`.
The compiled binary will live in `/target/release/`.
