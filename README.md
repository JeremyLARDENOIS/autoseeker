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
- You can override the dataset id via `--dataset-id` on `trigger`.
