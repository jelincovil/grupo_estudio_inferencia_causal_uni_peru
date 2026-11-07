# Equivalencias LaTeX ↔ Typst

El repositorio mantiene la misma plantilla en dos sistemas. Esta tabla permite
pasar de uno a otro sin reaprender, y decidir con qué empezar.

---

## 1 · Cuál elegir

| Elija **Typst** si… | Elija **LaTeX** si… |
|---|---|
| Empieza de cero | Ya tiene capítulos escritos en LaTeX |
| Quiere ver el PDF al guardar (compila en segundos) | Su asesor o coautores solo trabajan en LaTeX |
| Los mensajes de error de LaTeX le han costado horas | Necesita un paquete sin equivalente: `pgfplots`, `tikz-cd`, `algorithm2e`, `glossaries` |
| Va a programar la plantilla (lógica, bucles, condicionales) | La unidad de posgrado exige entrega de fuentes `.tex` |
| Necesita instalar nada: un binario único | Debe usar un estilo bibliográfico exótico de `biblatex` |

Ninguna de las dos es «más seria». Typst produce el mismo PDF y ambas plantillas
del repositorio siguen la misma estructura institucional. **Riesgo real a
considerar:** LaTeX tiene 40 años de respuestas en línea y cualquier asesor lo
conoce; Typst no. Si va a necesitar ayuda de terceros, eso pesa.

> **Lo que no cambia según el sistema:** el estimando, los supuestos, la
> sensibilidad y la verificación de cifras. Elegir plantilla es una decisión de
> 20 minutos; la cadena causal es la tesis.

---

## 2 · Estructura del documento

| | LaTeX | Typst |
|---|---|---|
| Clase / activación | `\documentclass[12pt,a4paper]{book}` | `#show: tesis.with(meta)` |
| Preámbulo / estilo | paquetes en el preámbulo | módulo `plantilla.typ` importado |
| Metadatos | `\newcommand{\doctitle}{…}` + `\hypersetup` | diccionario `meta` + `set document(...)` |
| Preliminares | `\frontmatter` | `#show: frontmatter` |
| Cuerpo | `\mainmatter` | `#show: mainmatter` |
| Apéndices | `\begin{appendices}` | `#show: apendices` |
| Capítulo | `\chapter{Título}` | `= Título` |
| Sección | `\section{…}` | `== …` |
| Subsección | `\subsection{…}` | `=== …` |
| Sin numerar | `\chapter*{…}` + `\addcontentsline` | `#prefacio([Título])[…]` |
| Índice | `\tableofcontents` | `#outline(title: [Índice general], depth: 2)` |
| Índice de figuras | `\listoffigures` | `#outline(target: figure.where(kind: image), …)` |
| Salto de página | `\newpage` / `\clearpage` | `#pagebreak()` |

---

## 3 · Texto

| | LaTeX | Typst |
|---|---|---|
| Negrita | `\textbf{x}` | `*x*` o `#strong[x]` |
| Cursiva | `\textit{x}` / `\emph{x}` | `_x_` o `#emph[x]` |
| Versalitas | `\textsc{x}` | `#smallcaps[x]` |
| Monoespaciado | `\texttt{x}` | `` `x` `` o `#raw("x")` |
| Lista con viñetas | `itemize` + `\item` | `- elemento` |
| Lista numerada | `enumerate` + `\item` | `+ elemento` |
| Lista de descripción | `description` + `\item[Término]` | `#terms(terms.item([Término], [def]))` |
| Nota al pie | `\footnote{…}` | `#footnote[…]` |
| Enlace | `\href{url}{texto}` | `#link("url")[texto]` |
| Espacio vertical | `\vspace{1cm}` | `#v(1cm)` |
| Centrado | `\begin{center}…\end{center}` | `#align(center)[…]` |
| Cita en bloque | `quote` | `#quote[…]` |

---

## 4 · Figuras, tablas y código

```latex
% LaTeX
\begin{figure}[H]
  \centering
  \includegraphics[width=0.85\textwidth]{figuras/mi.pdf}
  \caption{Leyenda descriptiva.}
  \label{fig:mi}
\end{figure}

\begin{table}[H]
  \centering
  \caption{Leyenda de la tabla.}
  \label{tab:mi}
  \begin{tabular}{lcc}
    \toprule
    Variable & Media & EE \\
    \midrule
    Ingreso  & 1250.4 & 12.1 \\
    \bottomrule
  \end{tabular}
\end{table}
```

```typst
// Typst
#figure(
  image("figuras/mi.pdf", width: 85%),
  caption: [Leyenda descriptiva.],
) <fig:mi>

#figure(
  table(
    columns: (2.2fr, 1fr, 1fr),
    table.hline(stroke: 1pt),
    table.header([*Variable*], [*Media*], [*EE*]),
    table.hline(stroke: 0.5pt),
    [Ingreso], [1250.4], [12.1],
    table.hline(stroke: 1pt),
  ),
  caption: [Leyenda de la tabla.],
) <tab:mi>
```

| | LaTeX | Typst |
|---|---|---|
| Referencia cruzada | `\cref{fig:mi}` → «Figura 2.1» | `@fig:mi` |
| Etiqueta | `\label{fig:mi}` dentro del entorno | `<fig:mi>` tras el `#figure` |
| Código con resaltado | `lstlisting` o `minted` | `` ```python … ``` `` dentro de `#figure` |
| Subfiguras | `subcaption` + `subfigure` | `grid` de `#figure`s |
| Tabla multipágina | `longtable` | `table` (se parte solo) |

En Typst la etiqueta va **después** del elemento, no dentro. Es el error de
migración más común.

---

## 5 · Entornos matemáticos

```latex
% LaTeX (amsthm)
\newtheorem{theorem}{Teorema}[chapter]
\newtheorem{definition}[theorem]{Definición}  % contador compartido

\begin{definition}[Espacio de probabilidad]\label{def:prob}
  Una terna $(\Omega, \mathcal{F}, P)$ donde…
\end{definition}

\begin{proof} … \end{proof}
```

```typst
// Typst (plantilla.typ del repositorio)
#definicion(nombre: [Espacio de probabilidad], id: "def:prob")[
  Una terna $(Omega, cal(F), P)$ donde…
]

#demostracion[ … ]
```

Disponibles en la plantilla Typst: `teorema`, `lema`, `corolario`,
`proposicion`, `definicion`, `axioma`, `observacion`, `nota`, `ejemplo`,
`demostracion`, `algoritmo`, `aviso`, `TODO`.

Para la sintaxis de fórmulas, véase [`03-guia-formulas.md`](03-guia-formulas.md) §5.

---

## 6 · Bibliografía

| | LaTeX | Typst |
|---|---|---|
| Declarar fuente | `\addbibresource{referencias.bib}` | — |
| Imprimir | `\printbibliography` | `#bibliography("referencias.bib", style: "apa")` |
| Cita textual | `\textcite{clave}` | `@clave` |
| Motor externo | `biber` (pasada aparte) | ninguno |

El mismo `referencias.bib` sirve para las dos plantillas sin cambios. Es la
razón para mantenerlo idéntico en ambas carpetas.

---

## 7 · Compilación

```bash
# LaTeX — latexmk resuelve biber y las pasadas múltiples por sí solo
latexmk -pdf tesis.tex
latexmk -pvc tesis.tex      # recompila al guardar
latexmk -C                  # limpia artefactos

# Typst
typst compile tesis.typ
typst watch tesis.typ       # recompila al guardar
```

No invoque `pdflatex` + `biber` + `pdflatex` + `pdflatex` a mano: `latexmk`
decide cuántas pasadas hacen falta y no se equivoca. Un guion que fija la
secuencia a mano falla en silencio cuando las referencias cruzadas necesitan una
pasada extra.

---

## 8 · Trampas de migración

| Síntoma | Causa |
|---|---|
| La referencia cruzada da «Figura 0.1» | La etiqueta Typst va antes del `#figure`, o falta el `<…>` |
| El índice de figuras muestra números mal | La numeración se evalúa en la posición del índice, no en el destino; la plantilla ya lo corrige con `counter(...).at(loc)` |
| Las tablas aparecen como «Cuadro N» (LaTeX) | `babel`+`spanish` usa «Cuadro» por defecto: pase la opción `es-tabla` |
| El idioma sale en inglés (LaTeX) | `babel` toma la **última** opción como idioma principal: `[english,spanish]` |
| Decimales con punto en tablas y coma en texto | Dos convenciones conviviendo; unifique en el generador de las tablas |
| Una ecuación numerada sin referenciar | Numere solo lo que cita |
| Una llamada a función se imprime como texto en el PDF (Typst) | Se usó una función que no existe o con argumentos de otro tipo; Typst no siempre falla, a veces compone la llamada literal. Revise el PDF, no solo el código de salida |
