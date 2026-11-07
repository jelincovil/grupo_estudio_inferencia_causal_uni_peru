#!/usr/bin/env bash
# =====================================================================
# compilar.sh — Compilación de la tesis (plantilla UNI · Typst)
# Uso:
#   ./compilar.sh              compila tesis.typ → tesis.pdf
#   ./compilar.sh doctorado    compila tesis_doctorado.typ
#   ./compilar.sh vigilante    recompila al guardar tesis.typ
# =====================================================================
set -e
cd "$(dirname "$0")"

if ! command -v typst >/dev/null 2>&1; then
  echo "✗ Typst no está instalado."
  echo "  Instálelo con:  cargo install typst-cli"
  echo "  o descargue el binario de https://github.com/typst/typst/releases"
  exit 1
fi

ARCHIVO="tesis.typ"
SALIDA="tesis.pdf"
if [ "${1:-}" = "doctorado" ]; then
  ARCHIVO="tesis_doctorado.typ"
  SALIDA="tesis_doctorado.pdf"
elif [ "${1:-}" = "vigilante" ]; then
  echo "▸ Compilando en modo vigilante (Ctrl+C para salir)…"
  typst watch tesis.typ
  exit 0
fi

echo "▸ Compilando ${ARCHIVO}…"
typst compile "$ARCHIVO" "$SALIDA"
echo "✓ Generado: ${SALIDA}"
