#!/usr/bin/env python3
"""
Schema Registry initialization script.
- Waits for Schema Registry to be available
- Reads all .avsc files from /schemas
- Validates JSON and registers each schema under <filename>-value
- Prints timestamped, colorized logs and a final summary
- Exits with code 0 if all registered, 1 if any failed
"""
import time
import os
import json
import sys
import urllib.request
from datetime import datetime

# ANSI color codes
GREEN = "\033[92m"
RED = "\033[91m"
YELLOW = "\033[93m"
BLUE = "\033[94m"
BOLD = "\033[1m"
RESET = "\033[0m"


def log(msg, color=""):
    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    print(f"{color}[{timestamp}] {msg}{RESET}", flush=True)


def main():
    url = os.environ.get("KAFKA_SCHEMA_REGISTRY_URL", "http://schema-registry:8081")

    log("=" * 70, BOLD)
    log("Schema Registry Initialization", BOLD + BLUE)
    log("=" * 70, BOLD)

    # Wait for Schema Registry to be ready
    log("Waiting for Schema Registry to be available...", YELLOW)
    for attempt in range(60):
        try:
            urllib.request.urlopen(url + "/subjects", timeout=5)
            log(f"\u2713 Schema Registry is ready (attempt {attempt + 1})", GREEN)
            break
        except Exception:
            if attempt >= 59:
                log("\u2717 Schema Registry not available after 60 attempts", RED)
                sys.exit(1)
            time.sleep(1)

    # Find all .avsc files
    try:
        schema_files = sorted([f for f in os.listdir('/schemas') if f.endswith('.avsc')])
    except FileNotFoundError:
        log("\u26A0 /schemas directory not found in container. Mount ./schemas to /schemas.", RED)
        sys.exit(1)

    if not schema_files:
        log("\u26A0 No .avsc schema files found in /schemas directory", YELLOW)
        sys.exit(0)

    log(f"\nFound {len(schema_files)} schema file(s) to register:", BLUE)
    for fname in schema_files:
        log(f"  \u2022 {fname}", BLUE)

    log("\n" + "-" * 70, BOLD)
    log("Registering schemas...", BOLD)
    log("-" * 70, BOLD)

    success_count = 0
    failure_count = 0
    results = []

    for fname in schema_files:
        subj = os.path.splitext(fname)[0]
        try:
            # Read and validate schema file
            with open(f'/schemas/{fname}', 'r') as f:
                schema_content = f.read()

            # Validate JSON
            try:
                json.loads(schema_content)
            except json.JSONDecodeError as je:
                raise Exception(f"Invalid JSON: {je}")

            # Register schema
            data = json.dumps({"schema": schema_content})
            req = urllib.request.Request(
                url + f"/subjects/{subj}-value/versions",
                data=data.encode('utf-8'),
                headers={'Content-Type': 'application/vnd.schemaregistry.v1+json'}
            )

            resp = urllib.request.urlopen(req, timeout=10)
            response_data = json.loads(resp.read().decode('utf-8'))
            schema_id = response_data.get('id', 'unknown')

            log(f"\u2713 {fname:<50} \u2192 ID: {schema_id}", GREEN)
            success_count += 1
            results.append({"file": fname, "subject": subj, "status": "success", "id": schema_id})

        except urllib.error.HTTPError as he:
            # Try to extract message from response body
            try:
                body = he.read().decode('utf-8') if he.fp else ''
                error_json = json.loads(body) if body else {}
                error_msg = error_json.get('message', body)
            except Exception:
                error_msg = str(he)

            log(f"\u2717 {fname:<50} \u2192 HTTP {he.code}: {error_msg}", RED)
            failure_count += 1
            results.append({"file": fname, "subject": subj, "status": "failed", "error": f"HTTP {he.code}: {error_msg}"})

        except Exception as e:
            log(f"\u2717 {fname:<50} \u2192 Error: {str(e)}", RED)
            failure_count += 1
            results.append({"file": fname, "subject": subj, "status": "failed", "error": str(e)})

    # Summary
    log("\n" + "=" * 70, BOLD)
    log("Registration Summary", BOLD + BLUE)
    log("=" * 70, BOLD)
    log(f"Total schemas: {len(schema_files)}", BLUE)
    log(f"\u2713 Successful: {success_count}", GREEN if success_count > 0 else BLUE)
    log(f"\u2717 Failed: {failure_count}", RED if failure_count > 0 else BLUE)

    if failure_count > 0:
        log("\n\u26A0 Some schemas failed to register. Check the errors above.", YELLOW)
        log("=" * 70, BOLD)
        sys.exit(1)
    else:
        log(f"\n\u2713 All schemas registered successfully!", GREEN + BOLD)
        log("=" * 70, BOLD)
        sys.exit(0)


if __name__ == '__main__':
    main()

