# autoseeker

Small Rust CLI to query Bright Data's LinkedIn Jobs dataset and output classified job postings as JSON.

## Prerequisites
- Rust toolchain (stable)
- A Bright Data API token

## Configure
Export your token (or pass `--token`). `.env` is supported:

```bash
echo 'BRIGHTDATA_TOKEN=<YOUR_TOKEN>' > .env
# or
export BRIGHTDATA_TOKEN="<YOUR_TOKEN>"
```

## Run
Fetch jobs for Montpellier with keyword Rust and classify:

```bash
cargo run -- --location Montpellier --keyword Rust --limit-per-input 10
```

Outputs a JSON array where each job object includes a `classification` field with:
- `seniority`: inferred from `job_seniority_level` or title/description keywords
- `contains_rust`: true if "Rust" appears in title/summary/description
- `remote_like`: heuristic based on "remote"/"télétravail" in location/description
- `city`: parsed from `job_location` or discovery input
- `company`: company name

## Notes
- Response parsing supports both JSON arrays and NDJSON lines.
- For other cities/keywords, adjust `--location` and `--keyword`.
