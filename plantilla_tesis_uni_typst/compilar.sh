#!/usr/bin/env bash
# =====================================================================
# compilar.sh — Compilación de la tesis (plantilla UNI · Typst)
# Uso:
#   ./compilar.sh            compila tesis.typ → tesis.pdf
#   ./compilar.sh vigilante  recompila al guardar cambios
# =====================================================================
set -e
cd "$(dirname "$0")"

if ! command -v typst >/dev/null 2>&1; then
  echo "✗ Typst no está instalado."
  echo "  Instálelo con:  cargo install typst-cli"
  echo "  o descargue el binario de https://github.com/typst/typst/releases"
  exit 1
fi

if [ "$1" = "vigilante" ]; then
  echo "▸ Compilando en modo vigilante (Ctrl+C para salir)…"
  typst watch tesis.typ
else
  echo "▸ Compilando tesis.typ…"
  typst compile tesis.typ
  echo "✓ Generado: tesis.pdf"
fi
