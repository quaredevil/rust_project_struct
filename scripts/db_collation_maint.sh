#!/usr/bin/env bash
set -euo pipefail

# Script: scripts/db_collation_maint.sh
# Objetivo: inspeção não destrutiva / manutenção (dump -> REINDEX -> VACUUM -> recheck)
# Uso: execute na raiz do repositório que contém docker-compose.dev.yml

DC_FILE="../docker-compose.dev.yml"
SERVICE=postgres
DB_NAME=listener
DB_USER=postgres
DB_PASS=postgres

if [ ! -f "$DC_FILE" ]; then
  echo "[ERROR] Arquivo $DC_FILE não encontrado no diretório atual. Rode este script na raiz do projeto." >&2
  exit 2
fi

echo "[INFO] Usando docker compose file: $DC_FILE"

# verifica container do postgres
CID=$(docker compose -f "$DC_FILE" ps -q $SERVICE || true)
if [ -z "$CID" ]; then
  echo "[ERROR] Serviço '$SERVICE' não está em execução pelo docker compose. Suba o serviço e reexecute: docker compose -f $DC_FILE up -d $SERVICE" >&2
  exit 3
fi

TS=$(date -u +"%Y%m%dT%H%M%SZ")
DUMP_CONTAINER_PATH="/tmp/${DB_NAME}_backup_${TS}.dump"
DUMP_HOST_PATH="./${DB_NAME}_backup_${TS}.dump"

echo "[STEP] 1) Criando dump compacto (pg_dump -Fc) dentro do container -> $DUMP_CONTAINER_PATH"
docker compose -f "$DC_FILE" exec -T -e PGPASSWORD="$DB_PASS" $SERVICE \
  pg_dump -U "$DB_USER" -Fc -f "$DUMP_CONTAINER_PATH" "$DB_NAME"

echo "[STEP] 2) Copiando dump do container($CID) para host -> $DUMP_HOST_PATH"
docker cp "${CID}:${DUMP_CONTAINER_PATH}" "$DUMP_HOST_PATH"

echo "[STEP] 3) Reindexando database '$DB_NAME' (REINDEX DATABASE)"
docker compose -f "$DC_FILE" exec -T -e PGPASSWORD="$DB_PASS" $SERVICE \
  psql -U "$DB_USER" -d "$DB_NAME" -c "REINDEX DATABASE \"$DB_NAME\";"

echo "[STEP] 4) Executando VACUUM (VERBOSE, ANALYZE)"
docker compose -f "$DC_FILE" exec -T -e PGPASSWORD="$DB_PASS" $SERVICE \
  psql -U "$DB_USER" -d "$DB_NAME" -c "VACUUM (VERBOSE, ANALYZE);"

echo "[STEP] 5) Rechecagem: listando datcollversion para todos os DBs"
docker compose -f "$DC_FILE" exec -T -e PGPASSWORD="$DB_PASS" $SERVICE \
  psql -U "$DB_USER" -c "SELECT datname, datcollversion FROM pg_database ORDER BY datname;"

echo "[STEP] 6) Mostrando últimos logs do container postgres (tail 200 linhas)"
docker compose -f "$DC_FILE" logs --no-color $SERVICE | tail -n 200

echo "[DONE] Dump salvo em: $DUMP_HOST_PATH"

exit 0
