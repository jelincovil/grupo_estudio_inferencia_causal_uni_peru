#!/usr/bin/env bash
# Verificación rápida del repositorio (sin regenerar Monte Carlo completo).
# Compila a un directorio temporal para no ensuciar el árbol de trabajo.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")" && pwd)"
TMP="$(mktemp -d -t reproduce.XXXXXX)"
trap 'rm -rf "$TMP"' EXIT

echo "== verificar_cifras.py =="
(cd "$ROOT/plantilla_tesis_uni_typst" && python3 verificar_cifras.py)

if command -v typst >/dev/null 2>&1; then
  echo "== typst compile (tesis.typ y tesis_doctorado.typ -> directorio temporal) =="
  typst compile "$ROOT/plantilla_tesis_uni_typst/tesis.typ" "$TMP/tesis.pdf"
  typst compile "$ROOT/plantilla_tesis_uni_typst/tesis_doctorado.typ" "$TMP/tesis_doctorado.pdf"
else
  echo "AVISO: typst no instalado; se omite compilación."
fi

if command -v latexmk >/dev/null 2>&1; then
  echo "== latexmk (plantilla LaTeX -> directorio temporal) =="
  mkdir -p "$TMP/latex"
  (cd "$ROOT/plantilla_tesis_uni_latex" && latexmk -pdf -interaction=nonstopmode -outdir="$TMP/latex" tesis.tex >/dev/null)
else
  echo "AVISO: latexmk no instalado; se omite LaTeX."
fi

echo "OK: reproduce.sh"
