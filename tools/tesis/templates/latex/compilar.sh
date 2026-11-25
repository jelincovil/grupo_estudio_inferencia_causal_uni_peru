#!/usr/bin/env bash
# Compilación con latexmk (pdflatex + biber + pasadas cruzadas).
set -euo pipefail
cd "$(dirname "$0")"
ARCHIVO=tesis.tex
if ! command -v latexmk >/dev/null 2>&1; then
  echo "latexmk no está instalado. Use TeX Live completo o: tlmgr install latexmk"
  exit 1
fi
latexmk -pdf -interaction=nonstopmode -file-line-error "$ARCHIVO"
echo "Listo: ${ARCHIVO%.tex}.pdf"
