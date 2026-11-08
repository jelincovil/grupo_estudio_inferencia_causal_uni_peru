#!/usr/bin/env bash
# Verificación rápida del repositorio (sin regenerar Monte Carlo completo).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")" && pwd)"

if [[ "${1:-}" == "--leccion" ]]; then
  exec python3 "$ROOT/leccion_60s.py"
fi

cd "$ROOT/plantilla_tesis_uni_typst"

echo "== verificar_cifras.py =="
python3 verificar_cifras.py

echo "== typst compile =="
if command -v typst >/dev/null 2>&1; then
  typst compile tesis.typ
else
  echo "AVISO: typst no instalado; se omite compilación."
fi

if command -v latexmk >/dev/null 2>&1; then
  echo "== latexmk (plantilla LaTeX) =="
  (cd "$ROOT/plantilla_tesis_uni_latex" && latexmk -pdf -interaction=nonstopmode tesis.tex >/dev/null)
else
  echo "AVISO: latexmk no instalado; se omite LaTeX."
fi

echo "OK: reproduce.sh"
