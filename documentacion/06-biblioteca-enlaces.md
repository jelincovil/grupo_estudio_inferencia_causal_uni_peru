# Biblioteca de enlaces

Fuentes de referencia del grupo. Organizadas por para qué se necesitan, no por
tipo de recurso.

**Leyenda de verificación**

| | Significado |
|---|---|
| ✅ | Resuelto y confirmado el **2026-11-07** |
| ◻ | Dominio estable de uso corriente, **no** reconfirmado en esa fecha: ábralo antes de citarlo |
| 📖 | Obra impresa: se cita por autoría, título, editorial y año, sin URL |

Regla de la carpeta: **si va a citar algo de aquí en una tesis, ábralo primero y
registre la fecha de consulta.** Un enlace heredado de una lista no cuenta como
verificado. Véase [`04-guia-referencias.md`](04-guia-referencias.md) §5.

---

## 1 · Normativa institucional UNI

La autoridad sobre formato, carátula, márgenes y entrega. **Mande siempre esto
por encima de las plantillas y de esta documentación.**

| Recurso | Enlace | |
|---|---|---|
| Reglamentos UNI — incluye el *Formato de Presentación de Tesis de la UNI* | https://dirce.uni.edu.pe/reglamentos-uni | ✅ |
| Secretaría FIEECS — plantilla de tesis, formato de presentación, reglamento de grados y títulos, y la guía de entrega al Repositorio UNI | https://fieecs.uni.edu.pe/secretaria/ | ✅ |
| Unidad de Posgrado FIEECS — programas de maestría y doctorado vigentes | https://fieecs.uni.edu.pe/presentacion-posgrado/ | ✅ |
| Reglamentos, directivas y documentos normativos institucionales (incl. Reglamento de Estudios de Posgrado) | https://portal.uni.edu.pe/index.php/acerca/225-reglamentos-directivas-manuales-y-otros-documentos-institucionales | ✅ |

> **Advertencia.** La denominación exacta del grado debe copiarse del programa en
> que está matriculado, no redactarse de memoria. Los programas de doctorado de
> la FIEECS se denominan, por ejemplo, *Doctorado en Ciencias e Ingeniería
> Estadística* ✅, que no es lo mismo que «Doctor en Ciencias con Mención en
> Estadística». Una carátula con el grado mal nombrado se observa en trámite.
>
> La sigla de la facultad es **FIEECS** (Facultad de Ingeniería Económica,
> Estadística y Ciencias Sociales) ✅, no «FIECS».

---

## 2 · Typst

| Recurso | Enlace | |
|---|---|---|
| Documentación oficial (referencia del lenguaje y de la librería) | https://typst.app/docs/ | ◻ |
| Typst Universe — paquetes y plantillas de la comunidad | https://typst.app/universe/ | ◻ |
| Foro oficial | https://forum.typst.app/ | ◻ |
| Repositorio y binarios (*releases*) | https://github.com/typst/typst | ◻ |

Para dudas de sintaxis, la documentación oficial es más rápida y más fiable que
buscar en foros: Typst es joven y mucho código que circula usa versiones viejas
del lenguaje. **Verifique siempre contra qué versión está escrito un ejemplo.**

---

## 3 · LaTeX

| Recurso | Enlace | |
|---|---|---|
| CTAN — archivo canónico de paquetes y su documentación | https://ctan.org/ | ✅ |
| TeX StackExchange — resolución de problemas concretos | https://tex.stackexchange.com/ | ✅ |
| Overleaf Learn — tutoriales por tema | https://www.overleaf.com/learn | ✅ |

Documentación de los paquetes que usan las plantillas (`texdoc <paquete>` en una
terminal abre la misma documentación sin conexión):

| Paquete | Para qué | Enlace | |
|---|---|---|---|
| `biblatex` | Bibliografía y citación | https://ctan.org/pkg/biblatex | ◻ |
| `biblatex-apa` | Estilo APA para `biblatex` | https://ctan.org/pkg/biblatex-apa | ◻ |
| `babel-spanish` | Español: partición, nombres, coma decimal, opciones `es-*` | https://ctan.org/pkg/babel-spanish | ◻ |
| `booktabs` | Tablas sin trazos verticales | https://ctan.org/pkg/booktabs | ◻ |
| `microtype` | Microtipografía | https://ctan.org/pkg/microtype | ◻ |
| `cleveref` | Referencias cruzadas con rótulo automático | https://ctan.org/pkg/cleveref | ◻ |
| `siunitx` | Unidades y alineación decimal en tablas | https://ctan.org/pkg/siunitx | ◻ |
| `latexmk` | Compilación automática (pasadas + `biber`) | https://ctan.org/pkg/latexmk | ◻ |
| `amsthm` | Teoremas y demostraciones | https://ctan.org/pkg/amsthm | ◻ |
| `xurl` | Partición de URLs largas | https://ctan.org/pkg/xurl | ◻ |

---

## 4 · Canon de inferencia causal

| Obra | Dónde | |
|---|---|---|
| Hernán, M. A. y Robins, J. M. (2020). *Causal Inference: What If*. Chapman & Hall/CRC. **Descarga gratuita** desde la página de los autores, con código y datos | https://miguelhernan.org/whatifbook · https://www.hsph.harvard.edu/miguel-hernan/causal-inference-book/ | ✅ |
| Chernozhukov, V., Chetverikov, D., Demirer, M., Duflo, E., Hansen, C., Newey, W. y Robins, J. (2018). Double/debiased machine learning for treatment and structural parameters. *The Econometrics Journal, 21*(1), C1–C68 | `doi:10.1111/ectj.12097` | ◻ |
| Pearl, J. (2009). *Causality: Models, Reasoning, and Inference* (2.ª ed.). Cambridge University Press | | 📖 |
| Imbens, G. W. y Rubin, D. B. (2015). *Causal Inference for Statistics, Social, and Biomedical Sciences*. Cambridge University Press | | 📖 |
| Angrist, J. D. y Pischke, J.-S. (2009). *Mostly Harmless Econometrics*. Princeton University Press | | 📖 |
| Efron, B. (1979). Bootstrap methods: another look at the jackknife. *The Annals of Statistics, 7*(1), 1–26 | `doi:10.1214/aos/1176344552` | ◻ |

**Ruta de lectura sugerida para un integrante nuevo:** Hernán & Robins partes I
y II (identificación antes que estimación) → Chernozhukov et al. 2018 §1–2 →
la demo de `plantilla_tesis_uni_typst/` ejecutada en su propia máquina. Leer DML
antes de entender identificación es el camino largo.

---

## 5 · Normas de reporte y reproducibilidad

| Recurso | Enlace | |
|---|---|---|
| **TARGET Statement** — reporte transparente de estudios observacionales que emulan un ensayo objetivo. Lista de 21 ítems en 6 secciones. Cashin et al., *BMJ* 2025;390:e087179 y *JAMA* 334(12), 1084–1093 (`doi:10.1001/jama.2025.13350`) | https://target-guideline.org/ | ✅ |
| Ficha de TARGET en la red EQUATOR, con la lista de verificación descargable | https://www.equator-network.org/reporting-guidelines/22462/ | ✅ |
| EQUATOR Network — catálogo general de guías de reporte (STROBE, CONSORT, PRISMA, RECORD…) | https://www.equator-network.org/ | ✅ |
| Hansford et al. (2023). Development of the TARGET guideline. *BMJ Open, 13*(9), e074626 | `doi:10.1136/bmjopen-2023-074626` | ✅ |

**Para una tesis del grupo**, TARGET es la referencia más directa: cubre
exactamente el caso de estimar efectos causales con datos observacionales y
exige lo que esta documentación exige —estimando explícito, supuestos de
identificación nombrados y análisis de sensibilidad—. Use su lista de
verificación como auditoría del capítulo de metodología, aunque su tesis no sea
de salud.

---

## 6 · Software para estimación causal

| Paquete | Para qué | Enlace | |
|---|---|---|---|
| DoubleML | DML en Python y R, con *cross-fitting* e inferencia | https://docs.doubleml.org/ | ◻ |
| DoWhy | Identificación con grafos, refutación y sensibilidad | https://github.com/py-why/dowhy | ◻ |
| EconML | Efectos heterogéneos (CATE), bosques causales | https://github.com/py-why/EconML | ◻ |
| scikit-learn | Aprendices para las funciones *nuisance* | https://scikit-learn.org/ | ◻ |

La demo del repositorio implementa DML **a mano** a propósito: así se ve la
mecánica del parcialado y del *cross-fitting* en veinte líneas. Para un trabajo
aplicado, use una librería mantenida y cítela con su versión.

---

## 7 · Tipografía y edición académica

| Obra / recurso | Dónde | |
|---|---|---|
| Bringhurst, R. *The Elements of Typographic Style*. Hartley & Marks | | 📖 |
| Tufte, E. R. *The Visual Display of Quantitative Information* (2.ª ed.). Graphics Press | | 📖 |
| Butterick, M. *Practical Typography* — libre en línea, referencia rápida de medida, interlineado y jerarquía | https://practicaltypography.com/ | ◻ |
| APA Style — estilo de citación y presentación de tablas y figuras | https://apastyle.apa.org/ | ◻ |
| **ISO 80000-2**, *Quantities and units — Part 2: Mathematics*. Norma de notación matemática (redonda/cursiva, operadores, constantes) | catálogo ISO | 📖 |
| Knuth, D. E., Larrabee, T. y Roberts, P. M. *Mathematical Writing*. Mathematical Association of America | | 📖 |

---

## 8 · Repositorios de tesis para estudiar modelos

No para copiar: para ver cómo se estructura un documento aprobado y premiado en
un programa exigente.

| Repositorio | Enlace | |
|---|---|---|
| Biblioteca Digital de Tesis y Disertaciones de la USP | https://www.teses.usp.br/ | ✅ |
| Repositorio Institucional de la UNI (acceso desde el portal) | véase §1 | ✅ |
| Premios INFORMS a disertaciones — útil para localizar tesis distinguidas en estadística y optimización, con la denominación exacta del premio | https://www.informs.org/Recognizing-Excellence/Award-Recipients | ◻ |

> Cuando incorpore una tesis a un listado del grupo, siga el formato de ficha de
> [`04-guia-referencias.md`](04-guia-referencias.md) §5, con DOI o handle
> resuelto y la línea de «por qué está aquí». Las listas heredadas
> El listado curado actual está en `ejemplos_tesis/README.md`. Los listados
> heredados (dos carpetas duplicadas) **no** cumplían ese formato y
> están pendientes de depuración: no las use como fuente hasta entonces.

---

## 9 · Datos peruanos para aplicaciones

| Fuente | Enlace | |
|---|---|---|
| INEI — microdatos de ENAHO, ENDES, censos | https://www.inei.gob.pe/ | ◻ |
| Plataforma Nacional de Datos Abiertos | https://www.datosabiertos.gob.pe/ | ◻ |

Para un capítulo de aplicación: declare en la tesis el año y la versión del
archivo descargado, el diccionario de variables y la fecha de descarga. Las bases
de encuestas se reponderan y se reeditan; una aplicación sin versión declarada
no es reproducible aunque el código sea perfecto.

---

*Última revisión: 2026-11-07. Al añadir una entrada, marque su estado de
verificación y la fecha.*
