#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
generar_recursos.py — Regenera las figuras y las cifras del experimento
Monte Carlo que alimenta el capítulo de Resultados de la demo.

Requisitos:  pip install numpy scipy matplotlib scikit-learn segno
Ejecución:   OMP_NUM_THREADS=1 python3 generar_recursos.py
             (o por escenarios: 250 / 1000 / 5000 / final)

Salidas: figuras/*.png, figuras/qr_repositorio.svg, results.json
         (las cifras del texto provienen de results.json)
"""

import json
import os
import sys
import numpy as np
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import FancyArrowPatch, Circle
import segno

import os
BASE = os.path.dirname(os.path.abspath(__file__))
FIGS = os.path.join(BASE, "figuras")
RUTA_JSON = os.path.join(BASE, "results.json")

# ---------- Paleta editorial ----------
INK      = "#1c1c1c"
NAVY     = "#1f3a5f"
NAVY_L   = "#5a7a9a"
GOLD     = "#8c6d3f"
SAGE     = "#5f7161"
REGLE    = "#c9c9c9"
PAPER    = "#f7f6f2"

plt.rcParams.update({
    "font.family": "sans-serif",
    "font.sans-serif": ["DejaVu Sans"],
    "text.color": INK, "axes.edgecolor": "#8a8a8a",
    "axes.labelcolor": INK,
    "axes.spines.top": False, "axes.spines.right": False,
    "axes.linewidth": 0.8, "xtick.color": "#555555", "ytick.color": "#555555",
    "font.size": 10.5, "axes.titlesize": 11.5,
})

# =====================================================================
# Funciones (importables sin efectos secundarios)
# =====================================================================
def expit(x):
    return 1.0 / (1.0 + np.exp(-x))

def m_verdadera(X):
    """Resultado esperado sin tratamiento (no lineal)."""
    return (1.6 * X[:, 0] - 1.1 * X[:, 1] + 0.85 * (X[:, 2] ** 2 - 1)
            + 0.6 * X[:, 0] * X[:, 1])

def generar_datos(n, rng):
    X = rng.standard_normal((n, 5))
    # X2 con colas pesadas (t_3 estandarizado) para realismo
    X[:, 1] = rng.standard_t(3, n) / np.sqrt(3.0)
    # La propensidad depende de X3 de forma NO lineal (X3² centrado):
    # quien ajuste solo efectos lineales quedará confundido.
    e = expit(1.1 * X[:, 0] - 0.7 * X[:, 1] + 0.9 * (X[:, 2] ** 2 - 1))
    A = (rng.random(n) < e).astype(float)
    tau = 2.0
    Y = tau * A + m_verdadera(X) + rng.normal(0, 1.5, n)
    return X, A, Y, tau

def est_dm(X, A, Y):
    """Diferencia de medias (ingenua, confundida)."""
    y1, y0 = Y[A == 1], Y[A == 0]
    tau_hat = y1.mean() - y0.mean()
    se = np.sqrt(y1.var(ddof=1) / y1.size + y0.var(ddof=1) / y0.size)
    return tau_hat, se

def logit_mle(Xd, y, iters=30, tol=1e-9):
    """Regresión logística por Newton-Raphson (IRLS)."""
    beta = np.zeros(Xd.shape[1])
    for _ in range(iters):
        p = expit(Xd @ beta)
        W = np.clip(p * (1 - p), 1e-6, None)
        grad = Xd.T @ (y - p)
        H = (Xd * W[:, None]).T @ Xd + 1e-8 * np.eye(Xd.shape[1])
        step = np.linalg.solve(H, grad)
        beta += step
        if np.max(np.abs(step)) < tol:
            break
    return beta

def est_ipw(X, A, Y):
    """IPW (Hájek) con propensidad logística bien especificada
    (base: 1, X1, X2, X3, X3²)."""
    Xd = np.column_stack([np.ones_like(A), X[:, 0], X[:, 1], X[:, 2],
                          X[:, 2] ** 2])
    beta = logit_mle(Xd, A)
    e = np.clip(expit(Xd @ beta), 0.02, 0.98)
    w1 = A / e
    w0 = (1 - A) / (1 - e)
    mu1 = (w1 * Y).sum() / w1.sum()
    mu0 = (w0 * Y).sum() / w0.sum()
    tau_hat = mu1 - mu0
    i1 = w1 * (Y - mu1) / w1.mean()
    i0 = w0 * (Y - mu0) / w0.mean()
    se = np.std(i1 - i0, ddof=1) / np.sqrt(len(A))
    return tau_hat, se

def est_ols(X, A, Y):
    """OLS con ajuste lineal de las cinco covariables observadas
    (mal especificado: omite el término X3² del confusor)."""
    Z = np.column_stack([np.ones_like(A), A, X])
    XtXi = np.linalg.inv(Z.T @ Z)
    beta = XtXi @ (Z.T @ Y)
    resid = Y - Z @ beta
    s2 = (resid ** 2).sum() / (len(Y) - Z.shape[1])
    se = np.sqrt(s2 * XtXi[1, 1])
    return beta[1], se

def est_dml(X, A, Y, rng=None, B_pred=5):
    """DML (partialling-out) con potenciación de gradiente por histogramas
    y validación cruzada de 5 particiones (cross-fitting)."""
    from sklearn.ensemble import HistGradientBoostingRegressor
    from sklearn.model_selection import KFold
    n = len(A)
    Yr = np.empty(n); Ar = np.empty(n)
    for tr, te in KFold(B_pred, shuffle=True, random_state=7).split(X):
        fy = HistGradientBoostingRegressor(max_iter=120, learning_rate=0.08,
                                           max_leaf_nodes=11, random_state=11)
        fa = HistGradientBoostingRegressor(max_iter=120, learning_rate=0.08,
                                           max_leaf_nodes=11, random_state=13)
        fy.fit(X[tr], Y[tr]); fa.fit(X[tr], A[tr])
        Yr[te] = Y[te] - fy.predict(X[te])
        Ar[te] = A[te] - fa.predict(X[te])
    denom = (Ar ** 2).mean()
    tau_hat = (Ar * Yr).mean() / denom
    psi = Ar * (Yr - tau_hat * Ar) / denom
    se = np.std(psi, ddof=1) / np.sqrt(n)
    return tau_hat, se

METODOS = {
    "Diferencia de medias": est_dm,
    "IPW (logístico)": est_ipw,
    "OLS ajustado": est_ols,
    "DML (gradiente)": est_dml,
}

ESCENARIOS = [(250, 300), (1000, 150), (5000, 60)]  # (n, réplicas)
Z = 1.959963984540054

def replica(i, n):
    """Una réplica: genera datos y corre los cuatro estimadores."""
    rng = np.random.default_rng(2026 + 17 * i)
    X, A, Y, tau = generar_datos(n, rng)
    out = {}
    for m, fn in METODOS.items():
        th, se = fn(X, A, Y)
        out[m] = (float(th), float(se))
    out["tau"] = tau
    return out

# =====================================================================
# Piezas de generación
# =====================================================================
def gen_qr():
    qr = segno.make("https://github.com/jelincovil/grupo_estudio_inferencia_causal_uni_peru",
                    error="m")
    qr.save(f"{FIGS}/qr_repositorio.svg", scale=10, border=2, dark=NAVY,
            light="#ffffff")
    print("QR generado", flush=True)

def gen_dag():
    fig, ax = plt.subplots(figsize=(6.4, 3.4), constrained_layout=True)
    ax.set_xlim(0, 10); ax.set_ylim(0, 5.2); ax.axis("off")

    def nodo(x, y, etiqueta, sub, color):
        ax.add_patch(Circle((x, y), 0.78, facecolor=PAPER, edgecolor=color,
                            linewidth=1.4, zorder=3))
        ax.text(x, y + 0.10, etiqueta, ha="center", va="center",
                fontsize=15, math_fontfamily="dejavuserif", color=color, zorder=4)
        ax.text(x, y - 0.38, sub, ha="center", va="center", fontsize=7.6,
                color="#666666", zorder=4)

    def flecha(x0, y0, x1, y1, color, curva=0.0, lw=1.4):
        ax.add_patch(FancyArrowPatch((x0, y0), (x1, y1),
                     connectionstyle=f"arc3,rad={curva}", arrowstyle="-|>",
                     mutation_scale=16, linewidth=lw, color=color,
                     shrinkA=14, shrinkB=14, zorder=2))

    nodo(2.0, 3.9, "C", "confusor", GOLD)
    nodo(2.0, 1.3, "A", "tratamiento", NAVY)
    nodo(8.0, 2.6, "Y", "resultado", NAVY)
    flecha(2.75, 1.55, 7.25, 2.45, NAVY)                 # A -> Y
    flecha(2.55, 3.55, 2.55, 1.75, GOLD)                 # C -> A
    flecha(2.75, 4.05, 7.25, 2.90, GOLD, curva=0.12)     # C -> Y
    ax.text(5.35, 1.72, r"$\tau$ (efecto causal)", fontsize=9.5, color=NAVY,
            ha="center", style="italic")
    ax.text(1.05, 2.62, "asociación\nsesgada", fontsize=8.2, color="#777777",
            ha="left", va="center", style="italic")
    fig.savefig(f"{FIGS}/cap2_dag_confusion.png", dpi=300, facecolor="white")
    plt.close(fig)
    print("DAG generado", flush=True)

def cargar_json():
    if os.path.exists(RUTA_JSON):
        with open(RUTA_JSON, encoding="utf-8") as f:
            return json.load(f)
    return {"resultados": {m: {} for m in METODOS}, "descriptivos": None,
            "nombres": None, "ns": [n for n, _ in ESCENARIOS],
            "bootstrap": None}

def correr_escenario(n, B):
    import multiprocessing as mp
    print(f"\n== escenario n={n} (B={B}) ==", flush=True)
    t0 = __import__("time").time()
    with mp.Pool(2) as pool:
        reps = pool.starmap(replica, [(i, n) for i in range(B)], chunksize=8)
    tau = reps[0]["tau"]
    res_m = {}
    for m in METODOS:
        taus = np.array([r[m][0] for r in reps])
        ses = np.array([r[m][1] for r in reps])
        cubre = np.abs(taus - tau) <= Z * ses
        res_m[m] = {
            "sesgo": float(taus.mean() - tau),
            "rmse": float(np.sqrt(((taus - tau) ** 2).mean())),
            "se": float(ses.mean()),
            "cobertura": float(cubre.mean()),
        }
        print(f"n={n:5d} | {m:22s} | sesgo={res_m[m]['sesgo']:+.3f} "
              f"RMSE={res_m[m]['rmse']:.3f} SE={res_m[m]['se']:.3f} "
              f"cob={res_m[m]['cobertura']*100:.1f}%", flush=True)
    print(f"[{__import__('time').time()-t0:.0f}s]", flush=True)
    return res_m

def gen_descriptivos():
    rng_d = np.random.default_rng(42)
    X, A, Y, _ = generar_datos(1000, rng_d)
    nombres = ["$X_1$ (confusor 1)", "$X_2$ (confusor 2)", "$X_3$ (confusor 3)",
               "$X_4$ (ruido)", "$X_5$ (ruido)", "$A$ (tratamiento)", "$Y$ (respuesta)"]
    cols = [X[:, 0], X[:, 1], X[:, 2], X[:, 3], X[:, 4], A, Y]
    desc = []
    for c in cols:
        q25, q50, q75 = np.percentile(c, [25, 50, 75])
        desc.append({"media": float(c.mean()), "sd": float(c.std(ddof=1)),
                     "min": float(c.min()), "q25": float(q25), "med": float(q50),
                     "q75": float(q75), "max": float(c.max())})
    print("\n--- Descriptivos (n=1000) ---", flush=True)
    for nom, d in zip(nombres, desc):
        print(f"{nom:22s} media={d['media']:+.2f} sd={d['sd']:.2f} "
              f"min={d['min']:+.2f} med={d['med']:+.2f} max={d['max']:+.2f}",
              flush=True)
    return nombres, desc

def gen_fig_cobertura(res):
    fig, ax = plt.subplots(figsize=(6.6, 3.9), constrained_layout=True)
    colores = {"Diferencia de medias": "#9a9a9a", "IPW (logístico)": GOLD,
               "OLS ajustado": SAGE, "DML (gradiente)": NAVY}
    marcas = {"Diferencia de medias": "s", "IPW (logístico)": "^",
              "OLS ajustado": "D", "DML (gradiente)": "o"}
    ns = [n for n, _ in ESCENARIOS]
    for m in METODOS:
        ys = [res[m][str(n)]["cobertura"] * 100 for n in ns]
        ax.plot(ns, ys, marker=marcas[m], color=colores[m], linewidth=1.6,
                markersize=6, label=m)
    ax.axhline(95, color="#8a8a8a", linestyle="--", linewidth=1.1)
    ax.text(253, 95.6, "nivel nominal (95 %)", fontsize=8.6, color="#777777")
    ax.set_xscale("log")
    ax.set_xticks(ns)
    ax.set_xticklabels([f"{n:,}" for n in ns])
    ax.set_xlabel("tamaño de muestra $n$ (escala logarítmica)")
    ax.set_ylabel("cobertura empírica (%)")
    ax.set_ylim(35, 100)
    ax.grid(axis="y", color=REGLE, linewidth=0.6, alpha=0.8)
    ax.legend(frameon=False, fontsize=9.2, loc="lower right")
    fig.savefig(f"{FIGS}/cap4_cobertura.png", dpi=300, facecolor="white")
    plt.close(fig)
    print("\nFigura cobertura generada", flush=True)

def gen_fig_bootstrap():
    B_boot = 2000
    rng_b = np.random.default_rng(99)
    Xb, Ab, Yb, tau = generar_datos(1000, rng_b)
    idx = np.arange(1000)
    taus_b = np.empty(B_boot)
    for b in range(B_boot):
        s = rng_b.choice(idx, size=1000, replace=True)
        Xs, As, Ys = Xb[s], Ab[s], Yb[s]
        Z = np.column_stack([np.ones(1000), Xs])
        lam = 1.0
        P = np.eye(6) * lam; P[0, 0] = 0
        betaY = np.linalg.solve(Z.T @ Z + P, Z.T @ Ys)
        betaA = np.linalg.solve(Z.T @ Z + P, Z.T @ As)
        Yr = Ys - Z @ betaY
        Ar = As - Z @ betaA
        taus_b[b] = (Ar * Yr).mean() / (Ar ** 2).mean()
    q_lo, q_hi = np.percentile(taus_b, [2.5, 97.5])
    fig, ax = plt.subplots(figsize=(6.6, 3.6), constrained_layout=True)
    ax.hist(taus_b, bins=42, color=NAVY_L, edgecolor="white", linewidth=0.4,
            density=True)
    ax.axvline(tau, color=INK, linewidth=1.4)
    ax.axvline(q_lo, color=GOLD, linewidth=1.3, linestyle="--")
    ax.axvline(q_hi, color=GOLD, linewidth=1.3, linestyle="--")
    ax.text(tau + 0.03, ax.get_ylim()[1] * 0.92, r"$\tau$ verdadero $=2$",
            fontsize=9.3, color=INK)
    ax.text(q_lo - 0.03, ax.get_ylim()[1] * 0.80, "IC 95 %",
            fontsize=9.3, color=GOLD, ha="right")
    ax.set_xlabel(r"estimador bootstrap de $\hat{\tau}$ (ajuste lineal parcial)")
    ax.set_ylabel("densidad")
    ax.grid(axis="y", color=REGLE, linewidth=0.6, alpha=0.7)
    fig.savefig(f"{FIGS}/cap4_bootstrap.png", dpi=300, facecolor="white")
    plt.close(fig)
    print(f"Figura bootstrap generada | media boot={taus_b.mean():.3f} "
          f"IC=[{q_lo:.3f}, {q_hi:.3f}]", flush=True)
    return {"media": float(taus_b.mean()), "ic": [float(q_lo), float(q_hi)],
            "sd": float(taus_b.std(ddof=1))}

# =====================================================================
# Orquestación
# =====================================================================
if __name__ == "__main__":
    arg = sys.argv[1] if len(sys.argv) > 1 else None

    if arg is None or arg == "final":
        gen_qr()
        gen_dag()

    datos = cargar_json()

    if arg in (None, "250", "1000", "5000"):
        n_obj = int(arg) if arg else None
        for n, B in ESCENARIOS:
            if n_obj is None or n == n_obj:
                res_m = correr_escenario(n, B)
                for m in METODOS:
                    datos["resultados"][m][str(n)] = res_m[m]
                with open(RUTA_JSON, "w", encoding="utf-8") as f:
                    json.dump(datos, f, ensure_ascii=False, indent=1)
                print("results.json actualizado", flush=True)
            elif str(n) not in datos["resultados"]["DML (gradiente)"]:
                print(f"(pendiente: escenario n={n})", flush=True)

    if arg is None or arg == "final":
        completo = all(
            str(n) in datos["resultados"][m]
            for n, _ in ESCENARIOS for m in METODOS
        )
        if not completo:
            print("AVISO: faltan escenarios; ejecute 250, 1000, 5000 y luego "
                  "'final'. Solo se generan QR y DAG.", flush=True)
        else:
            nombres, desc = gen_descriptivos()
            datos["nombres"] = nombres
            datos["descriptivos"] = desc
            gen_fig_cobertura(datos["resultados"])
            datos["bootstrap"] = gen_fig_bootstrap()
            with open(RUTA_JSON, "w", encoding="utf-8") as f:
                json.dump(datos, f, ensure_ascii=False, indent=1)
            print("\nresults.json final guardado", flush=True)
