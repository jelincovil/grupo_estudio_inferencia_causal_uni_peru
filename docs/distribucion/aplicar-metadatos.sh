#!/usr/bin/env bash
# Ejecutar con gh autenticado (scopes repo). No contiene secretos.
set -euo pipefail
REPO="${1:-jelincovil/grupo_estudio_inferencia_causal_uni_peru}"
DESC='Plantillas de tesis en Typst y LaTeX con estructura UNI/APA, guías de redacción y ejemplos compilables. Para tesistas de ingeniería y posgrado en español.'
HOME='https://github.com/jelincovil/grupo_estudio_inferencia_causal_uni_peru/releases/tag/v0.1.0-demo'

gh api -X PATCH "repos/${REPO}" -f description="$DESC" -f homepage="$HOME"
gh api -X PUT "repos/${REPO}/topics" --input - <<'JSON'
{"names":["typst","latex","thesis-template","plantilla-tesis","tesis","apa","ingenieria","causal-inference","causalinf","education","spanish","uni-peru","posgrado","redaccion-cientifica","latex-template"]}
JSON
echo "Topics actuales:"
gh api "repos/${REPO}/topics" --jq .names
