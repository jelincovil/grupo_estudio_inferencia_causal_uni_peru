# Evaluación de calidad del material — Grupo de Estudio de Inferencia Causal UNI Perú

> **Documento temporal de trabajo.** No forma parte del material del repositorio.
> Generado el 2026-11-07. Criterios fundamentados en Parallel/OpenAlex, Jina,
> Firecrawl y las memorias mem0 (`memo_enrique` / `atlas_movilidad_cl`).

---

## 1. Método y fuentes

| Fuente | Uso | Resultado clave |
|---|---|---|
| **OpenAlex (Parallel)** | Verificar la referencia canónica del método DML de la demo | La demo implementa *double/debiased machine learning* (Chernozhukov et al. 2018) pero **no lo cita**. |
| **OpenAlex** | Marcos de rigor causal | *Target trial* (Hernán & Robins 2016), TARGET Statement (BMJ 2025). |
| **Jina / web** | Normas de reporte y reproducibilidad | "Claim causality with clarity", *Reproducible research practices* (Hochheimer 2024), checklists SAMBR. |
| **Firecrawl** | Normativa institucional UNI | Reglamento vigente R.R. Nº 3145-2023-UNI-DN (referencia a citar y verificar). |
| **mem0 (memorias)** | Estándares metodológicos propios del usuario | Criterios: *estado de verificación*, *nivel de evidencia*, *amenazas*, *criterio de falsación*, *procedencia*, *doble cifra*, *reproducibilidad de microdatos*. |
| **Context7** | Verificar sintaxis y buenas prácticas oficiales | Typst (`style: "apa"` correcto, APA 7.ª ed.); LaTeX (`babel`: última opción = idioma principal); *LaTeX Advice* recomienda `latexmk` y excluir artefactos del repo. |

---

## 2. Inventario del material

```
grupo_estudio_inferencia_causal_uni_peru/
├── README.md                         ← raíz (incompleto: "Objetivos" vacío, links sin llenar)
├── plantilla_tesis_uni_latex/        ← plantilla LaTeX (tesis maestría/doctorado)
│   ├── tesis_template_uni.tex
│   ├── GUIA_COMPLETA.md / README.md / referencias.bib
│   ├── compilar_v2.sh
│   └── (artefactos de build: .aux .log .bcf .synctex.gz .pdf .zip …)
├── plantilla_tesis_uni_typst/        ← plantilla Typst (nueva, no versionada aún)
│   ├── tesis.typ (demo DML) / plantilla.typ / referencias.bib
│   ├── generar_recursos.py / results.json / compilar.sh
│   └── figuras/
└── ejemplos_tesis/README.md          ← tesis de referencia (maestría + doctorado)
```

---

## 3. Evaluación por componente

### 3.1 Plantilla Typst (`plantilla_tesis_uni_typst/`) — **muy buena**

**Verificado en esta sesión:** `typst compile tesis.typ` → exit 0, PDF de ~1 MB, sin errores.

Fortalezas:
- **Demo coherente y real**, no *lorem ipsum*: una mini-tesis sobre estimación causal
  con DML, con texto, tablas y figuras que provienen de un Monte Carlo reproducible.
- **Correcta implementación del estimador**: *partialling-out* con *cross-fitting*
  (`KFold(5)`), dos aprendices de gradiente por histogramas, residuales cruzados y
  error estándar por función de influencia (`psi = Ar*(Yr − τ·Ar)/den`, `se = std(psi)/√n`).
  Esto reproduce fielmente el DML2 parcialmente lineal de Chernozhukov et al. (2018).
- **Comparadores bien elegidos**: diferencia de medias, IPW de Hájek con recorte
  `[0.02, 0.98]`, y OLS **deliberadamente mal especificado** (omite `X₃²`) para
  mostrar que el sesgo no decrece con `n`.
- **Semillas fijas** (`2026 + 17·i`) → réplicas reproducibles.
- **Pedagogía de alto valor**: el bootstrap del ajuste lineal ("preciso pero sesgado")
  enseña que *precisión no implica validez causal*. Es exactamente la distinción
  asociación↔causalidad que exigen las memorias mem0.

Debilidades:
1. **Omite la referencia fundacional.** `referencias.bib` no contiene a
   Chernozhukov et al. (2018); la demo implementa DML citando a Hastie/Friedman/
   LeCun/Gelman pero no al artículo que define el método.
2. **Supuestos de identificación no sistematizados.** El texto menciona "no confusión
   y positividad" de pasada, pero no hay una lista explícita supuesto↔estimador↔amenaza.
3. **Sin análisis de sensibilidad formal** (violación de positividad, confusión no
   observada): solo hay bootstrap diagnóstico.
4. **Dependencias sin fijar versiones** (`pip install numpy scipy matplotlib …` sin
   `requirements.txt`/lock). No se reporta la versión de software usada.

### 3.2 Plantilla LaTeX (`plantilla_tesis_uni_latex/`) — **buena estructura, detalles a corregir**

Fortalezas: arquitectura modular, entornos `amsthm`, `biblatex`-APA, `booktabs`,
`cleveref`, metadatos PDF, checklist pre-entrega.

Debilidades:
1. **Idioma principal mal configurado**: `\usepackage[spanish,english]{babel}` deja el
   **inglés** como idioma principal de una tesis en español (el propio comentario del
   archivo lo reconoce).
2. **Inconsistencia de nombres de archivo**: el `.tex` real es `tesis_template_uni.tex`,
   pero `compilar_v2.sh` y la guía apuntan a `tesis_doctoral_v2_final.tex` (inexistente);
   el `README.md` alterna entre `tesis_doctoral_internacional.tex` y
   `tesis_maestria-doctoral_internacional.tex`. El script, tal cual, **fallaría**.
3. **Artefactos de build versionados**: `.aux`, `.log`, `.bcf`, `.run.xml`,
   `.synctex.gz`, `.toc`, `.lof`, `.lot`, `.pdf` y `templatetesis.zip` no deberían
   estar en el repositorio (falta `.gitignore`).

### 3.3 Ejemplos de tesis (`ejemplos_tesis/`) — **reescrito (curado)**

- Contenido casi duplicado entre ambos `README.md`.
- Afirmaciones sin trazabilidad verificable ("información basada en búsquedas recientes
  (enero 2026)") y enlaces de descarga frágiles (uno a un PDF de conferencia EDM2021,
  otro con doble extensión `.pdf.pdf`). No hay DOI ni metadatos de procedencia — viola
  directamente el criterio mem0 de *procedencia verificable* y *estado de verificación*.

### 3.4 Raíz y gobernanza — **incompleta**

- `README.md` raíz: sección **"Objetivos" vacía** y links de LinkedIn de 4/5 integrantes
  sin URL.
- Colisión de mayúsculas `README.md`/`Readme.md` (ya resuelta en commit local).
- `.DS_Store` presente (sin `.gitignore`).

---

## 4. Criterios de mejora (priorizados)

Cada criterio enlaza su **procedencia** (mem0, OpenAlex, Jina/web, Firecrawl) y una
**acción concreta**.

### A. Rigor causal y técnico

| # | Criterio | Procedencia | Acción concreta |
|---|---|---|---|
| A1 | Citar la referencia canónica del método implementado | OpenAlex | Añadir a `referencias.bib` **Chernozhukov et al. (2018), *Econometrics Journal* 21(1):C1–C68, doi:10.1111/ectj.12097** y citarla en el Marco Teórico/Estado del arte. |
| A2 | Declarar supuestos de identificación por estimador | "Claim causality with clarity" (PMC10917363); NRC | Tabla supuesto↔estimador: **no confusión**, **positividad/overlap**, **SUTVA/consistencia**; indicar cuál falla en cada método (DM/OLS: confusión; IPW: near-violación de positividad). |
| A3 | Etiquetar nivel de evidencia de cada conclusión | mem0 (`nivel de evidencia`) | Añadir rótulo a cada resultado: `asociación` / `identificación` / `simulación` / `conjetura`. |
| A4 | Análisis de sensibilidad formal | TARGET Statement (ítem 7h-ii) | Añadir 1–2 sensibilidades: (a) variar el recorte de propensidad; (b) confusión no observada vía sesgo de Rosenbaum/E-value o variable omitida simulada. |
| A5 | Reportar estimando y software/versión | TARGET (ítems 7g–7h) | Declarar explícitamente el **ATE** como estimando y fijar/versionar `numpy`, `scikit-learn`, `matplotlib`. |

### B. Reproducibilidad

| # | Criterio | Procedencia | Acción concreta |
|---|---|---|---|
| B1 | Documentar supuestos y datos en README | Hochheimer et al. (2024) | `README` de datos: definición de cada variable simulada, supuestos, semilla. |
| B2 | Fijar entorno reproducible | Hochheimer 2024; mem0 (*sin microdatos no hay reproducibilidad*) | Añadir `requirements.txt`/`environment.yml` con versiones; `OMP_NUM_THREADS=1` ya documentado. |
| B3 | Verificar que texto y `results.json` coinciden | mem0 (*doble cifra*) | Test/check que compare las cifras de la Tabla 4.2 con `results.json` (detectar *drift* si se edita el experimento). |
| B4 | Control de integridad de salidas | mem0 | *Checksum* de `results.json`/figuras o un target que regenere y compare. |

### C. Estructura y normativa institucional

| # | Criterio | Procedencia | Acción concreta |
|---|---|---|---|
| C1 | Citar y verificar la norma UNI vigente | Firecrawl | Referenciar **R.R. Nº 3145-2023-UNI-DN** y la guía de la Unidad de Posgrado (con fecha y enlace) en lugar de afirmarlo sin cita. |
| C2 | Homologar estructura LaTeX ↔ Typst | consistencia interna | Unificar portada, "Copia de documentos", matriz de consistencia y de operacionalización en ambas plantillas. |
| C3 | Plantilla alineada a líneas de investigación | Firecrawl (reglamento) | Enlazar la demo a una línea de investigación declarada de la FIEECS/Unidad de Posgrado. |

### D. Calidad editorial y gobernanza del repo

| # | Criterio | Procedencia | Acción concreta |
|---|---|---|---|
| D1 | Corregir idioma principal de `babel` | Context7 (LaTeX) | `babel` usa la **última opción** como idioma principal: cambiar a `\usepackage[english,spanish]{babel}` para tesis en español. |
| D2 | Unificar nombre de archivo LaTeX y scripts | Context7 (*LaTeX Advice*) | Renombrar a un único nombre (p. ej. `tesis.tex`) y reemplazar `compilar_v2.sh` por **`latexmk`** (gestiona `pdflatex`+`biber`+pasadas múltiples automáticamente). |
| D3 | `.gitignore` y limpieza | Context7 (*LaTeX Advice*) | Ignorar `*.aux *.log *.bcf *.run.xml *.synctex.gz *.toc *.lof *.lot *.pdf *.zip .DS_Store`; retirar artefactos. *LaTeX Advice*: no versionar PDF compilado, bibliografías compiladas ni figuras generadas. |
| D4 | Completar README raíz | revisión editorial | Llenar "Objetivos" y los LinkedIn pendientes. |
| D5 | Consolidar o depurar los dos `README` de ejemplos | revisión editorial | Un único listado con DOI/procedencia verificable, o eliminarlos si no aportan. |

### E. Verificación continua

| # | Criterio | Procedencia | Acción concreta |
|---|---|---|---|
| E1 | Compilación automatizada | Context7 (*LaTeX Advice*); verificación de esta sesión (Typst OK) | *Check* de CI que compile `tesis.typ` y, si se arregla D2, `tesis.tex` vía **`latexmk`** en cada PR. |
| E2 | Prueba de regresión numérica | mem0 (*doble cifra*) | *Golden test*: ejecutar `generar_recursos.py 250` y comparar contra `results.json` comprometido. |

---

## 5. Veredicto resumido

- **La plantilla Typst es el activo de mayor calidad**: contenido real, coherente y
  reproducible, con un estimador DML implementado correctamente y una pedagogía causal
  acertada. Las mejoras de mayor impacto son **A1 (citar DML)**, **A2 (supuestos)** y
  **B3 (verificar cifras)**.
- **La plantilla LaTeX** tiene buena arquitectura pero hoy **no compilaría por el script
  desalineado** (D2) y tiene el idioma principal invertido (D1).
- **Los ejemplos de tesis y el README raíz** son la parte más débil por falta de
  trazabilidad (criterio mem0 de *procedencia*).

Prioridad sugerida de ejecución: **A1 → D2 → D1 → A2 → B3 → D3**, y luego C1–C3.
