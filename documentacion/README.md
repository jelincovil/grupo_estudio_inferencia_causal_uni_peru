# Documentación · Grupo de Estudio de Inferencia Causal UNI

Carpeta de referencia para quienes escriben una tesis de maestría o doctorado
con las plantillas de este repositorio (`plantilla_tesis_uni_latex/`,
`plantilla_tesis_uni_typst/`). No contiene plantillas: contiene los
**criterios** con que se juzga un documento académico de alto nivel —
estructurales, editoriales, visuales, matemáticos y bibliográficos— y la
biblioteca de enlaces verificados del grupo.

La regla que organiza toda la carpeta: **una tesis se evalúa por la cadena
estimando → identificación → estimación → sensibilidad, y se lee por su
tipografía.** Lo primero decide si el trabajo es válido; lo segundo, si alguien
lo leerá hasta el final.

---

## Orden de lectura

| # | Archivo | Para qué sirve | Cuándo leerlo |
|---|---|---|---|
| 00 | [`00-modelo-tesis.md`](00-modelo-tesis.md) | Estructura capítulo por capítulo, diferenciada maestría / doctorado | Antes de escribir la primera línea |
| 01 | [`01-guia-editorial.md`](01-guia-editorial.md) | Voz, tiempos verbales, arquitectura del párrafo, cómo se redacta cada sección | Al redactar |
| 02 | [`02-guia-visual.md`](02-guia-visual.md) | Tipografía, retícula, figuras, tablas, gráficos, color | Al maquetar y al hacer figuras |
| 03 | [`03-guia-formulas.md`](03-guia-formulas.md) | Notación matemática normalizada (ISO 80000-2), ecuaciones, estimandos | Al escribir el marco teórico y la metodología |
| 04 | [`04-guia-referencias.md`](04-guia-referencias.md) | Citación APA 7, higiene de `.bib`, verificación de fuentes | Al construir la bibliografía |
| 05 | [`05-equivalencias-latex-typst.md`](05-equivalencias-latex-typst.md) | Tabla de traducción entre las dos plantillas | Al migrar o al elegir plantilla |
| 06 | [`06-biblioteca-enlaces.md`](06-biblioteca-enlaces.md) | Enlaces verificados: normativa UNI, documentación técnica, canon causal | Siempre que necesite una fuente |

---

## Principio de trazabilidad

Toda afirmación en un documento del grupo —tesis, informe o README— cae en una
de cuatro categorías, y se rotula como tal cuando no es obvia por el contexto:

| Rótulo | Significado | Qué la respalda |
|---|---|---|
| `asociación` | Patrón observado en los datos | El dato mismo |
| `identificación` | Efecto causal, bajo supuestos declarados | Los supuestos + el diseño |
| `simulación` | Propiedad de un generador conocido | El código y la semilla |
| `conjetura` | Hipótesis no contrastada en este trabajo | Nada; dígalo |

Un resultado de simulación **no** es evidencia sobre el mundo: es evidencia
sobre el estimador. Confundir las dos categorías es el error más común —y más
costoso— en las tesis que este grupo revisa.

---

## Regla de verificación de fuentes

Ninguna referencia entra a un documento del grupo sin **DOI, handle de
repositorio o URL institucional comprobada en el momento de escribir**, con la
fecha de consulta registrada. Un enlace que no se abrió nunca no es una
referencia: es una conjetura con formato de referencia.

Esta regla existe porque es exactamente donde falla el material heredado del
repositorio (véase [`04-guia-referencias.md`](04-guia-referencias.md), §5).

---

*Grupo de Estudio de Inferencia Causal · UNI · FIEECS*
