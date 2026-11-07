#!/usr/bin/env python3
"""Comprueba que las cifras de la Tabla 4.2 (tab:simulacion) en tesis.typ coinciden con results.json."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

BASE = Path(__file__).resolve().parent
TESIS = BASE / "tesis.typ"
JSON = BASE / "results.json"

# Orden de filas en la tabla Typst (dentro de cada panel n)
METHODS = [
    "Diferencia de medias",
    "IPW (logístico)",
    "OLS ajustado",
    "DML (gradiente)",
]
NS = [250, 1000, 5000]

# Patrones en tesis.typ: [Método], [Sesgo], [RMSE], [EE], [Cobertura %]
ROW_RE = re.compile(
    r"\[([^,\]]+)\],\s*\[\$?([\+\-]?\d+\.\d+)\$\],\s*\[(\d+\.\d+)\],\s*\[(\d+\.\d+)\],\s*\[(\d+(?:\.\d+)?)\s*%\]"
)


def load_json():
    with JSON.open(encoding="utf-8") as f:
        return json.load(f)["resultados"]


def parse_table(text: str) -> dict[tuple[str, int], dict[str, float]]:
    rows = ROW_RE.findall(text)
    out: dict[tuple[str, int], dict[str, float]] = {}
    idx = 0
    for n in NS:
        for method in METHODS:
            if idx >= len(rows):
                raise RuntimeError("La tabla en tesis.typ tiene menos filas de las esperadas")
            m_label, sesgo, rmse, se, cov = rows[idx]
            idx += 1
            if m_label.strip() != method:
                raise RuntimeError(
                    f"Fila inesperada en panel n={n}: «{m_label}» (esperaba «{method}»)"
                )
            out[(method, n)] = {
                "sesgo": float(sesgo),
                "rmse": float(rmse),
                "se": float(se),
                "cobertura": float(cov),
            }
    return out


def json_metrics(data, method: str, n: int) -> dict[str, float]:
    block = data[method][str(n)]
    return {
        "sesgo": round(block["sesgo"], 3),
        "rmse": round(block["rmse"], 3),
        "se": round(block["se"], 3),
        "cobertura": round(block["cobertura"] * 100, 1),
    }


def main() -> int:
    if not TESIS.is_file() or not JSON.is_file():
        print("Faltan tesis.typ o results.json", file=sys.stderr)
        return 2

    text = TESIS.read_text(encoding="utf-8")
    end = text.find("<tab:simulacion>")
    begin = text.find("<tab:descriptivos>")
    if end == -1:
        print("No se encontró <tab:simulacion> en tesis.typ", file=sys.stderr)
        return 2
    chunk = text[begin:end] if begin != -1 else text[:end]

    table = parse_table(chunk)
    data = load_json()
    errors = []

    for n in NS:
        for method in METHODS:
            got = table[(method, n)]
            exp = json_metrics(data, method, n)
            for key in ("sesgo", "rmse", "se", "cobertura"):
                if got[key] != exp[key]:
                    errors.append(
                        f"n={n} {method} {key}: tesis.typ={got[key]} results.json={exp[key]}"
                    )

    if errors:
        print("DRIFT detectado entre tesis.typ y results.json:")
        for e in errors:
            print(" ", e)
        return 1

    print("OK: Tabla 4.2 (tab:simulacion) coherente con results.json")
    return 0


if __name__ == "__main__":
    sys.exit(main())
