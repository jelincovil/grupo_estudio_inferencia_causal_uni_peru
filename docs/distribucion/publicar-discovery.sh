#!/usr/bin/env bash
# Ejecutar en su Mac (requiere git push + gh autenticado).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"

if ! command -v gh >/dev/null || ! gh auth status >/dev/null 2>&1; then
  echo "Primero: gh auth login"
  exit 1
fi

echo "== git push =="
git push origin chore/discovery-improvements

if ! gh pr list --head chore/discovery-improvements --json number --jq length | grep -q '^[1-9]'; then
  echo "== crear PR =="
  gh pr create --base main --head chore/discovery-improvements \
    --title "chore: topics, README y release de demostración" \
  --body-file docs/distribucion/PR_BODY.md
else
  echo "== PR ya existe =="
  gh pr list --head chore/discovery-improvements
fi

echo "== metadatos GitHub =="
bash docs/distribucion/aplicar-metadatos.sh

echo "Listo. Revise el PR y merge cuando el grupo apruebe."
