# Empiece aquí (tesista UNI)

Este repositorio le da **formato y criterios de redacción**, no el contenido de su investigación. Siga este orden.

## 1 · Normativa institucional

Antes de elegir Typst o LaTeX, revise lo que la UNI y la FIEECS exigen (márgenes, carátula, entrega al repositorio institucional):

- [Reglamentos UNI · DIRCE](https://dirce.uni.edu.pe/reglamentos-uni) (*Formato de Presentación de Tesis*, entre otros)
- Enlaces curados: [`documentacion/06-biblioteca-enlaces.md`](documentacion/06-biblioteca-enlaces.md)

## 2 · Elija plantilla

| Situación | Carpeta |
| :--- | :--- |
| Quiere ver un PDF completo de referencia y trabajar en Typst | [`plantilla_tesis_uni_typst/`](plantilla_tesis_uni_typst/) |
| Prefiere LaTeX / biblatex-APA | [`plantilla_tesis_uni_latex/`](plantilla_tesis_uni_latex/) |
| No está seguro | [`documentacion/05-equivalencias-latex-typst.md`](documentacion/05-equivalencias-latex-typst.md) |

En Typst encontrará un **documento de ejemplo** (`tesis.pdf`). Cópielo como punto de partida y sustituya título, capítulos y bibliografía por los suyos.

En LaTeX el cuerpo trae texto de relleno (`\lipsum`); la utilidad es la **estructura** (preliminares, capítulos, apéndices, APA).

## 3 · Compile una vez en su máquina

```bash
# Typst
cd plantilla_tesis_uni_typst && ./compilar.sh

# o LaTeX
cd plantilla_tesis_uni_latex && ./compilar.sh
```

Si no compila, resuelva dependencias **antes** de escribir cientos de páginas.

## 4 · Lea las guías mientras escribe

| Archivo | Uso |
| :--- | :--- |
| [`00-modelo-tesis.md`](documentacion/00-modelo-tesis.md) | Orden de capítulos (maestría / doctorado) |
| [`01-guia-editorial.md`](documentacion/01-guia-editorial.md) | Tiempos verbales, resumen, discusión, números en español |
| [`02-guia-visual.md`](documentacion/02-guia-visual.md) | Tablas, figuras, tipografía |
| [`03-guia-formulas.md`](documentacion/03-guia-formulas.md) | Notación matemática |
| [`04-guia-referencias.md`](documentacion/04-guia-referencias.md) | APA, higiene del `.bib`, verificar enlaces |

Las guías priorizan normativa UNI/FIEECS sobre recomendaciones internas del grupo (está indicado en cada archivo).

## 5 · Metadatos y portada

Edite **un solo bloque** de metadatos en la plantilla elegida (autor, título, grado oficial de su programa, asesor). El grado debe coincidir con el nombre que publica la Unidad de Posgrado FIEECS.

## 6 · Bibliografía y tesis de referencia

- Use `referencias.bib` de la plantilla como modelo de campos APA.
- Para ver trabajos ajenos bien citados: [`ejemplos_tesis/README.md`](ejemplos_tesis/README.md) (solo entradas verificadas).

## 7 · Antes de entregar al asesor

Lista de comprobación: [`01-guia-editorial.md`](documentacion/01-guia-editorial.md) §7.

---

*Mantenido por el Grupo de Estudio de Inferencia Causal · UNI · Perú*
