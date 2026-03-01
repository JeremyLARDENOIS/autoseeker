# autoseeker

Small Rust CLI to interact with Bright Data's Jobs datasets (LinkedIn + Indeed).

Command groups:
- `jobs`: fetch and save parsed jobs
- `snapshot`: trigger/list/download raw snapshots

## Prerequisites
- A Bright Data API token

Optional (build from source):
- Rust toolchain (stable)

## Configure
`BRIGHTDATA_TOKEN` is required. `.env` is supported:

```bash
echo 'BRIGHTDATA_TOKEN=<YOUR_TOKEN>' > .env
# or
export BRIGHTDATA_TOKEN="<YOUR_TOKEN>"
```

## Install
Download a prebuilt binary from GitHub Releases (recommended).

Assets published by the release workflow:
- Linux: `autoseeker-linux-x86_64`
- macOS: `autoseeker-macos`
- Windows: `autoseeker-windows-x86_64.exe`

Example (Linux/macOS):

```bash
# download the right asset for your platform from the Releases page
mv ./autoseeker-linux-x86_64 ./autoseeker
chmod +x ./autoseeker
./autoseeker --help
```

You can also rename it to `autoseeker` and move it into your `PATH` (e.g. `~/.local/bin`).

Windows (PowerShell) example:

```powershell
Rename-Item .\autoseeker-windows-x86_64.exe autoseeker.exe
.\autoseeker.exe --help
```

In the examples below, `autoseeker ...` assumes the binary is in your `PATH`. If it’s not, run `./autoseeker ...` (Linux/macOS) or `.\autoseeker.exe ...` (Windows).

## Run
Fetch jobs for Montpellier with keyword Rust (waits for snapshot readiness and saves `jobs.json`):

```bash
autoseeker jobs get --location Montpellier --keyword Rust
```

Tune the discovery limit (per-input) if needed:

```bash
autoseeker jobs get --location Montpellier --keyword Rust --limit 50
```

Trigger a new snapshot run without downloading/parsing jobs:

```bash
autoseeker snapshot trigger --location Montpellier --keyword Rust --limit 50
```

Add optional Bright Data discovery criteria (single input):

```bash
autoseeker snapshot trigger \
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
autoseeker snapshot trigger --inputs-file inputs.json --limit 50
```

Optional: filter providers in the file:

```bash
# run only LinkedIn entries from the file
autoseeker snapshot trigger --inputs-file inputs.json --provider linkedin

# run only Indeed entries from the file
autoseeker snapshot trigger --inputs-file inputs.json --provider indeed
```

## Build from source

```bash
cargo build --release
./target/release/autoseeker --help
```

The `--inputs-file` format supports LinkedIn and Indeed inputs.

Notes:
- Each entry can include a `provider` field (`"linkedin"` or `"indeed"`).
- Backward compatible: if `provider` is omitted, the entry is treated as **LinkedIn**.
- Any string field can be omitted; missing string fields default to `""`.

Example mixed-provider `inputs.json`:

```json
{
	"input": [
		{
			"provider": "linkedin",
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
			"provider": "indeed",
			"country": "FR",
			"domain": "fr.indeed.com",
			"keyword": "\"product manager\"",
			"location": "Paris, Île-de-France",
			"date_posted": "Last 24 hours",
			"posted_by": "",
			"location_radius": "",
			"pay": 60000
		}
	]
}
```

Complete `inputs.json` templates (all supported keys):

LinkedIn entry:

```json
{
	"input": [
		{
			"provider": "linkedin",
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

Indeed entry:

```json
{
	"input": [
		{
			"provider": "indeed",
			"country": "",
			"domain": "",
			"keyword": "",
			"location": "",
			"date_posted": "",
			"posted_by": "",
			"location_radius": "",
			"pay": null
		}
	]
}
```

LinkedIn enum notes:
- Enum fields (`time_range`, `job_type`, `experience_level`, `remote`) can be omitted, or set to `""` or `null` to mean “unset”.

Indeed enum notes:
- `date_posted` can be omitted, or set to `""` or `null` to mean “unset”.

Allowed `date_posted` values in `inputs.json` (exact strings):
- `Last 24 hours`, `Last 3 days`, `Last 7 days`, `Last 14 days`

Backward compatible values:
- `date_posted` also accepts `"1"`, `"3"`, `"7"`, `"14"` (mapped to the corresponding `Last ...` values).

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
			"provider": "linkedin",
			"location": "Montpellier",
			"keyword": "Rust",
			"country": "FR",
			"time_range": "Past week",
			"job_type": "Full-time",
			"remote": "Hybrid"
		},
		{
			"provider": "linkedin",
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
autoseeker snapshot list
```

Download a snapshot by id (writes to `snapshot.json` by default):

```bash
autoseeker snapshot download <SNAPSHOT_ID>
# or choose an output file
autoseeker snapshot download <SNAPSHOT_ID> --output my_snapshot.json
```

For options, run:

```bash
autoseeker --help
autoseeker jobs --help
autoseeker jobs get --help
autoseeker snapshot --help
autoseeker snapshot trigger --help
autoseeker snapshot list --help
autoseeker snapshot download --help
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
