# autoseeker

Small Rust CLI to interact with Bright Data's LinkedIn Jobs dataset.

Command groups:
- `jobs`: fetch and save parsed jobs
- `snapshot`: trigger/list/download raw snapshots

## Prerequisites
- Rust toolchain (stable)
- A Bright Data API token

## Configure
`BRIGHTDATA_TOKEN` is required. `.env` is supported:

```bash
echo 'BRIGHTDATA_TOKEN=<YOUR_TOKEN>' > .env
# or
export BRIGHTDATA_TOKEN="<YOUR_TOKEN>"
```

## Run
Fetch jobs for Montpellier with keyword Rust (waits for snapshot readiness and saves `jobs.json`):

```bash
cargo run -- jobs get --location Montpellier --keyword Rust
```

Tune the discovery limit (per-input) if needed:

```bash
cargo run -- jobs get --location Montpellier --keyword Rust --limit 50
```

Trigger a new snapshot run without downloading/parsing jobs:

```bash
cargo run -- snapshot trigger --location Montpellier --keyword Rust --limit 50
```

Add optional Bright Data discovery criteria (single input):

```bash
cargo run -- snapshot trigger \
	--location paris \
	--keyword "product manager" \
	--country FR \
	--time-range "Past month" \
	--job-type "Full-time" \
	--experience-level Internship \
	--remote "On-site" \
	--company "" \
	--location-radius "" \
	--selective-search "" \
	--limit 50
```

Multi-criteria search (multiple inputs in one trigger request) via JSON file:

```bash
cargo run -- snapshot trigger --inputs-file inputs.json --limit 50
```

Example `inputs.json`:

```json
{
	"input": [
		{
			"location": "paris",
			"keyword": "product manager",
			"country": "FR",
			"time_range": "Past month",
			"job_type": "Full-time",
			"experience_level": "Internship",
			"remote": "On-site",
			"selective_search": "",
			"company": "",
			"location_radius": ""
		},
		{
			"location": "New York",
			"keyword": "\"python developer\"",
			"experience_level": "Executive"
		}
	]
}
```

Complete `inputs.json` template (all supported keys):

```json
{
	"input": [
		{
			"location": "",
			"keyword": "",
			"country": "",
			"time_range": "",
			"job_type": "",
			"experience_level": "",
			"remote": "",
			"selective_search": "",
			"company": "",
			"location_radius": ""
		}
	]
}
```

Notes:
- Any field can be omitted; missing string fields default to `""`.
- Enum fields (`time_range`, `job_type`, `experience_level`, `remote`) can be omitted, or set to `""` or `null` to mean “unset”.

Allowed enum values in `inputs.json` (exact strings):
- `time_range`: `Past 24 hours`, `Past week`, `Past month`, `Any time`
- `job_type`: `Full-time`, `Part-time`, `Contract`, `Temporary`, `Volunteer`
- `experience_level`: `Internship`, `Entry level`, `Associate`, `Mid-Senior level`, `Director`, `Executive`
- `remote`: `On-site`, `Remote`, `Hybrid`

Worked multi-input example (mix of strict enums + omitted fields):

```json
{
	"input": [
		{
			"location": "Montpellier",
			"keyword": "Rust",
			"country": "FR",
			"time_range": "Past week",
			"job_type": "Full-time",
			"remote": "Hybrid"
		},
		{
			"location": "Paris",
			"keyword": "product manager",
			"experience_level": "Executive",
			"selective_search": "startup"
		}
	]
}
```

List available snapshots (printed as a table):

```bash
cargo run -- snapshot list
```

Download a snapshot by id (writes to `snapshot.json` by default):

```bash
cargo run -- snapshot download <SNAPSHOT_ID>
# or choose an output file
cargo run -- snapshot download <SNAPSHOT_ID> --output my_snapshot.json
```

For options, run:

```bash
cargo run -- --help
cargo run -- jobs --help
cargo run -- jobs get --help
cargo run -- snapshot --help
cargo run -- snapshot trigger --help
cargo run -- snapshot list --help
cargo run -- snapshot download --help
```

Notes on output:
- `jobs get` writes parsed jobs to `jobs.json`.
- `snapshot download` writes the raw response body to disk (JSON array or NDJSON depending on dataset settings).

## Notes
- Response parsing supports both JSON arrays and NDJSON lines.
- For other cities/keywords, adjust `--location` and `--keyword`.

Enums:
- `--time-range`: `Past 24 hours`, `Past week`, `Past month`, `Any time`
- `--job-type`: `Full-time`, `Part-time`, `Contract`, `Temporary`, `Volunteer`
- `--experience-level`: `Internship`, `Entry level`, `Associate`, `Mid-Senior level`, `Director`, `Executive`
- `--remote`: `On-site`, `Remote`, `Hybrid`
