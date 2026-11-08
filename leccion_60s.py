#!/usr/bin/env python3
"""
La lección en 60 segundos — lee results.json (sin NumPy).
Uso: python3 leccion_60s.py [--compartir]
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent
JSON = ROOT / "plantilla_tesis_uni_typst" / "results.json"

TAU = 2.0
N_SHOW = 5000

# ANSI
R = "\033[0m"
B = "\033[1m"
D = "\033[2m"
RED = "\033[31m"
GRN = "\033[32m"
YLW = "\033[33m"
CYN = "\033[36m"
MAG = "\033[35m"


def bar(value: float, width: int = 24, char: str = "█") -> str:
    v = min(abs(value) / 1.0, 1.0)
    n = max(1, int(v * width)) if v > 0.02 else 0
    return char * n + "░" * (width - n)


def fmt_num(x: float, places: int = 3) -> str:
    s = f"{x:+.{places}f}".replace(".", ",")
    return s


def main() -> int:
    parser = argparse.ArgumentParser(description="Demo causal en 60 s (solo stdlib).")
    parser.add_argument(
        "--compartir",
        action="store_true",
        help="Texto corto para pegar en redes (sin colores).",
    )
    args = parser.parse_args()

    if not JSON.is_file():
        print("No encuentro results.json; ejecute desde la raíz del repositorio.", file=sys.stderr)
        return 2

    data = json.loads(JSON.read_text(encoding="utf-8"))
    res = data["resultados"]
    key_n = str(N_SHOW)

    rows = [
        ("Diferencia de medias", res["Diferencia de medias"][key_n]),
        ("IPW (logístico)", res["IPW (logístico)"][key_n]),
        ("OLS ajustado", res["OLS ajustado"][key_n]),
        ("DML (gradiente)", res["DML (gradiente)"][key_n]),
    ]

    if args.compartir:
        ols = res["OLS ajustado"][key_n]
        dml = res["DML (gradiente)"][key_n]
        text = (
            f"τ verdadero = {TAU}. Con n = {N_SHOW}: OLS sesgo {ols['sesgo']:+.3f}, "
            f"cobertura {ols['cobertura']*100:.0f}%. DML sesgo {dml['sesgo']:+.3f}. "
            "Un intervalo estrecho no es una disculpa. "
            "Demo reproducible: github.com/jelincovil/grupo_estudio_inferencia_causal_uni_peru"
        )
        print(text)
        return 0

    use_color = sys.stdout.isatty()
    if not use_color:
        R = B = D = RED = GRN = YLW = CYN = MAG = ""

    ols = res["OLS ajustado"][key_n]
    dml = res["DML (gradiente)"][key_n]

    print()
    print(f"{CYN}{B}╔════════════════════════════════════════════════════════════════════╗{R}")
    print(f"{CYN}{B}║  INTERVALO MENTIROSO · Grupo Inferencia Causal UNI · τ = {TAU:.0f}           ║{R}")
    print(f"{CYN}{B}╚════════════════════════════════════════════════════════════════════╝{R}")
    print(f"{D}  Datos simulados con confusión no lineal · n = {N_SHOW:,} · cifras en results.json{R}")
    print()

    print(f"  {B}{'Método':<22} {'|sesgo|':>8}   visual          cobertura 95 %{R}")
    print(f"  {D}{'─' * 62}{R}")

    for name, block in rows:
        sesgo = block["sesgo"]
        cov = block["cobertura"] * 100
        vis = bar(sesgo)
        tag = ""
        if name == "OLS ajustado":
            vis = f"{RED}{vis}{R}"
            tag = f" {RED}← preciso y equivocado{R}"
        elif name == "DML (gradiente)":
            vis = f"{GRN}{vis}{R}"
            tag = f" {GRN}← incómodo y honesto{R}"
        print(f"  {name:<22} {fmt_num(sesgo):>8}   {vis}  {cov:5.1f} %{tag}")

    print()
    print(f"  {YLW}{B}Moraleja:{R} el error estándar del OLS {D}encoge{R} con n; el sesgo {D}no{R}.")
    print(f"  {B}Precisión ≠ causalidad.{R} Por eso esta plantilla exige sensibilidad y verificación.")
    print()
    print(f"  {D}Siguiente paso:{R}")
    print(f"    {MAG}python3 leccion_60s.py --compartir{R}     # texto para LinkedIn / X")
    print(f"    {MAG}open docs/la_leccion.html{R}           # experiencia visual")
    print(f"    {MAG}./reproduce.sh{R}                     # PDF + coherencia de cifras")
    print()
    return 0


if __name__ == "__main__":
    sys.exit(main())
