# Guía de referencias y citación

Estilo de citación, higiene del archivo `.bib` y —sobre todo— **verificación de
fuentes**, que es donde más fácilmente se pierde la credibilidad de un trabajo
por completo.

---

## 1 · Estilo

**APA 7.ª edición, autor–año**, localizado al español. Es lo que usan las dos
plantillas del repositorio.

| | LaTeX | Typst |
|---|---|---|
| Motor | `biblatex` + `biber` | nativo |
| Configuración | `\usepackage[backend=biber,style=apa]{biblatex}` | `bibliography("referencias.bib", style: "apa")` |
| Localización | `\DeclareLanguageMapping{spanish}{spanish-apa}` | automática vía `text(lang: "es")` |
| Cita textual | `\textcite{clave}` → Efron (1979) | `@clave` |
| Cita parentética | `\parencite{clave}` → (Efron, 1979) | `#cite(<clave>, form: "normal")` |
| Comillas | `csquotes` + `\enquote{}` | `"..."` según `lang` |

No use `bibtex` con APA: produce formato autor–número y rompe la norma. No mezcle
`\cite` de LaTeX con `biblatex` en el mismo documento.

---

## 2 · Reglas de citación

1. **Cite la fuente del método que usa, no un sustituto.** Si implementa
   *double/debiased machine learning*, la cita obligatoria es Chernozhukov et al.
   (2018); citar a Hastie o a Breiman en su lugar es citar a los vecinos del
   método. Este fue un defecto real de la demo del repositorio, ya corregido.
2. **Cite la edición que consultó**, con año y edición. «Casella & Berger (2002),
   2.ª ed.» no es lo mismo que la primera.
3. **Cite lo primario.** Un artículo de divulgación sobre un paper no es el
   paper. Una entrada de blog sobre un teorema no es el teorema.
4. **No cite lo que no leyó.** Si solo vio el resumen, o lo conoce por una cita
   de otro, use `citado en` y diga por quién.
5. **Página o sección para toda cita específica**: cifras, definiciones,
   enunciados. `(Wasserman, 2006, p. 142)`.
6. **Autocitas declaradas.** Si reutiliza texto propio ya publicado, dígalo. El
   autoplagio es plagio ante una comisión de grados.
7. **Asistencia de IA declarada** si se usó, con herramienta, alcance y versión,
   en una nota de métodos o en los agradecimientos según exija la normativa. Una
   herramienta no es coautora y no puede figurar como tal.

---

## 3 · Higiene del `.bib`

### Campos obligatorios

| Tipo | Obligatorio | Muy recomendado |
|---|---|---|
| `@article` | author, title, journal, year, volume, pages | number, **doi** |
| `@book` | author/editor, title, publisher, year | edition, address, **isbn**, doi |
| `@incollection` | author, title, booktitle, editor, publisher, year, pages | **doi** |
| `@inproceedings` | author, title, booktitle, year, pages | address, **doi** |
| `@phdthesis` | author, title, school, year | type, **doi/handle**, url + urldate |
| `@online` | author/organization, title, year, **url**, **urldate** | version |

**El DOI no es opcional.** Si la obra tiene DOI y la entrada no lo trae, la
entrada está incompleta. Es lo único que sobrevive al cambio de URLs.

### El tipo de entrada importa

El tipo determina cómo APA formatea la referencia. Dos errores reales del
`referencias.bib` del repositorio:

```bibtex
% ✗ Es un libro (CRC Press, serie «Monographs on Statistics and
%   Applied Probability»), no un artículo
@article{hastie2015statistical,
  title = {Statistical Learning with Sparsity: The Lasso and Generalizations},
  journal = {Monographs on Statistics and Applied Probability}, ...
}

% ✓
@book{hastie2015statistical,
  author    = {Hastie, Trevor and Tibshirani, Robert and Wainwright, Martin},
  title     = {Statistical Learning with Sparsity: The Lasso and Generalizations},
  series    = {Monographs on Statistics and Applied Probability},
  publisher = {CRC Press},
  address   = {Boca Raton, FL},
  year      = {2015},
  doi       = {10.1201/b18401}
}
```

```bibtex
% ✗ «Proceedings of the IEEE» es una revista, no unas actas de congreso
@inproceedings{lecun1998gradient, booktitle = {Proceedings of the IEEE}, ... }

% ✓
@article{lecun1998gradient,
  author  = {LeCun, Yann and Bottou, L{\'e}on and Bengio, Yoshua and Haffner, Patrick},
  title   = {Gradient-Based Learning Applied to Document Recognition},
  journal = {Proceedings of the IEEE},
  year    = {1998}, volume = {86}, number = {11}, pages = {2278--2324},
  doi     = {10.1109/5.726791}
}
```

### Detalles que rompen la salida

- **Mayúsculas protegidas** en títulos: `{DML}`, `{B}ayes`, `{M}onte {C}arlo`.
  Sin llaves, APA pasa el título a minúsculas y deja «dml» y «bayes».
- **Acentos**: UTF-8 directo con `biber`; `{\'e}` también funciona. No mezcle.
- **Rangos de páginas con doble guion**: `267--288`, no `267-288`.
- **Nombres completos** cuando los conozca, no iniciales: APA los abrevia solo.
- **Claves estables y legibles**: `autor+año+palabra` (`efron1979bootstrap`).
  Nunca `ref1`, `paper23`.
- Una clave = una obra. Entradas duplicadas con claves distintas producen
  bibliografías con la misma obra dos veces.

---

## 4 · Citar código, datos y software

Son fuentes, y se citan:

```bibtex
@software{scikit-learn,
  author  = {Pedregosa, Fabian and others},
  title   = {scikit-learn: Machine Learning in {P}ython},
  year    = {2011},
  version = {1.6.1},
  url     = {https://scikit-learn.org},
  urldate = {2026-11-07}
}

@dataset{enaho2024,
  author    = {{Instituto Nacional de Estadística e Informática}},
  title     = {Encuesta Nacional de Hogares (ENAHO) 2024},
  year      = {2025},
  publisher = {INEI},
  urldate   = {2026-11-07}
}
```

La **versión** es parte de la cita de software: un resultado producido con
scikit-learn 1.6.1 puede no reproducirse con 1.7. Para el repositorio propio,
cite el *commit* o un DOI de versión (Zenodo).

---

## 5 · Verificación de fuentes

**Regla del grupo: ninguna referencia entra a un documento sin que alguien haya
abierto su DOI, handle o URL y confirmado que el documento es el que la cita
dice.** La fecha de esa comprobación se registra (`urldate`).

Protocolo, en orden:

1. Resolver el **DOI** (`https://doi.org/…`) y comparar título, autoría y año
   con la entrada.
2. Sin DOI, usar el **handle del repositorio institucional**.
3. Sin handle, **URL institucional** + `urldate`. Nunca el PDF alojado en un
   sitio personal o en un agregador si existe la versión institucional.
4. Para una tesis: repositorio de la universidad que la otorgó. Si no está en
   acceso abierto, cítela como tal; no sustituya el enlace por otro documento.

### Auditoría del material heredado

El listado heredado (antes duplicado en dos carpetas) era el ejemplo de lo que
esta sección previene. El reemplazo vivo es `ejemplos_tesis/README.md`. Defectos
que motivaron la auditoría (archivos antiguos):

| Defecto | Evidencia |
|---|---|
| Enlace a un documento distinto del citado | Una tesis de máster UNED de 2023 se «descarga» desde un PDF de la conferencia EDM **2021**: otro documento, otro año, otro tipo de publicación |
| *Handles* no resueltos | `spiral.imperial.ac.uk/handle/10044/1/112345` y `.../98765`: patrón de relleno. El propio texto lo delata al añadir «busca por título o autor para el archivo…», es decir, quien escribió no tenía el handle |
| Doble extensión | Un enlace termina en `.pdf.pdf` |
| Entrada sin autoría | «Autor: No especificado en detalle» |
| Procedencia global en lugar de por ítem | «La información se basa en búsquedas recientes (enero 2026)» no permite verificar ninguna entrada concreta |
| Contenido casi duplicado | Los dos archivos se superponen sin que se declare la diferencia de alcance |

No todo el contenido es incorrecto —varios de los premios citados existen y son
verificables—, y eso es precisamente el problema: **una lista que mezcla entradas
correctas con enlaces inventados no es parcialmente útil, es íntegramente no
fiable**, porque el lector no puede saber cuál es cuál sin verificarlas todas.

Formato de reemplazo propuesto, una fila por tesis:

| Campo | Contenido |
|---|---|
| Autoría | Apellido, Nombre |
| Título | exacto, en su idioma original |
| Grado e institución | p. ej. PhD, Imperial College London |
| Año de defensa | |
| Premio | nombre exacto, entidad que lo otorga, año |
| Identificador | DOI o handle, resuelto |
| Verificado | fecha + quién |
| Por qué está aquí | una línea: qué puede aprender un tesista de este documento |

El último campo es el que convierte una lista en material docente. Sin él, es
un marcador de enlaces.

---

## 6 · Lista de verificación de la bibliografía

- [ ] Toda cita del texto está en el `.bib` y toda entrada del `.bib` se cita.
- [ ] Toda entrada con DOI disponible lo trae.
- [ ] Todo enlace se abrió, con `urldate` registrada.
- [ ] Los tipos de entrada corresponden al tipo real de publicación.
- [ ] El método principal cita su fuente fundacional.
- [ ] Mayúsculas protegidas con llaves donde hace falta.
- [ ] Sin duplicados.
- [ ] El software usado está citado con su versión.
- [ ] `biber` compila sin advertencias (`biber --validate-datamodel tesis`).
- [ ] Ninguna referencia fue incorporada sin leerla, o está marcada como `citado en`.
