#!/usr/bin/env bash
set -euo pipefail

echo "[entrypoint] starting listener service"
# Resolve preferred Kafka env vars (new KAFKA_* overrides legacy KAFKA__* / APP__KAFKA__*)
KAFKA_BROKERS_RESOLVED="${KAFKA_BROKERS:-${KAFKA__BROKERS:-${APP__KAFKA__BROKERS:-}}}"
KAFKA_SCHEMA_REGISTRY_URL_RESOLVED="${KAFKA_SCHEMA_REGISTRY_URL:-${KAFKA__SCHEMA_REGISTRY_URL:-${APP__KAFKA__SCHEMA_REGISTRY_URL:-}}}"

echo "[entrypoint] ENV CHECK: KAFKA_BROKERS=${KAFKA_BROKERS_RESOLVED:-<unset>}"
if [ -z "${KAFKA_BROKERS_RESOLVED:-}" ]; then
  echo "[entrypoint] ERROR: required environment variable KAFKA_BROKERS (or legacy KAFKA__BROKERS / APP__KAFKA__BROKERS) is not set."
  echo "[entrypoint] Please set KAFKA_BROKERS=broker:9092 and restart the container."
  exit 1
fi

echo "[entrypoint] ENV CHECK: KAFKA_SCHEMA_REGISTRY_URL=${KAFKA_SCHEMA_REGISTRY_URL_RESOLVED:-<unset>}"
echo "[entrypoint] ENV CHECK: DATABASE_URL=${DATABASE_URL:-<unset>}"
echo "[entrypoint] ENV CHECK: REDIS_URL=${REDIS_URL:-<unset>}"

if [ -n "${DATABASE_URL:-}" ]; then
  echo "[entrypoint] (placeholder) migrations step - integrate sqlx migrate run here"
  # sqlx migrate run || echo "[entrypoint] migrations skipped (sqlx not installed in runtime image)"
fi
exec "$@"
