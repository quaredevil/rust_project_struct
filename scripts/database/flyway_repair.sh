#!/usr/bin/env bash
set -euo pipefail

# Utilitário para executar `flyway repair` no container Flyway definido no docker-compose de desenvolvimento.
# Uso: ./scripts/flyway_repair.sh [-f docker-compose-file]

COMPOSE_FILE="docker-compose.dev.yml"
if [[ ${1:-} == "-f" && -n ${2:-} ]]; then
  COMPOSE_FILE="$2"
fi

echo "Running Flyway repair using compose file: $COMPOSE_FILE"

docker compose -f "$COMPOSE_FILE" run --rm flyway repair

echo "Done. Verifique os logs e re-execute o docker compose up para aplicar novas migrations."
