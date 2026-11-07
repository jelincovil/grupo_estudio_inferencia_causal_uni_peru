# Guía tipográfica y visual

Criterios de composición para documentos impresos de alto nivel académico. Las
plantillas del repositorio ya implementan casi todo esto; esta guía explica
**por qué**, para que quien las modifique sepa qué está rompiendo.

> Los márgenes, el tamaño de papel y la carátula los fija la normativa UNI. Donde
> esta guía y la normativa discrepen, manda la normativa.

---

## 1 · Tipografía del cuerpo

| Parámetro | Valor | Razón |
|---|---|---|
| Familia | Serif con familia matemática a juego: **New Computer Modern**, Libertinus, STIX Two, Latin Modern | El texto y las fórmulas deben compartir peso, altura de x y estilo; una serif de texto con matemáticas de otra familia produce fórmulas que «flotan» |
| Cuerpo | 11–12 pt | 12 pt es el estándar institucional habitual |
| Interlineado | 1,5 si la norma lo exige; **1,15–1,25 es mejor** si hay libertad | El 1,5 fue heredado de la mecanografía para permitir correcciones a mano |
| Medida (ancho de línea) | **60–75 caracteres** | Debajo de 50 el ojo salta de línea demasiado seguido; encima de 85 pierde el retorno |
| Alineación | Justificada **con partición de palabras activada** | Justificar sin partición abre ríos de espacio blanco |
| Sangría de primera línea | 1–1,25 cm, suprimida tras un título | Sangría *o* espacio entre párrafos, nunca ambos |

**La medida es el parámetro que más se descuida.** Con A4, cuerpo 12 pt y
márgenes de 2,5 cm resultan ~95 caracteres por línea: ilegible en bloques
largos. Los márgenes de las plantillas (izq. 3,4–3,5 cm, der. 2,5–2,6 cm) no son
decorativos: corrigen la medida y dejan el canal de encuadernado.

Verifique la medida así: cuente los caracteres de tres líneas llenas y promedie.
Si pasa de 85, amplíe márgenes o suba el cuerpo.

---

## 2 · Jerarquía

Tres niveles visibles en el cuerpo, como máximo. Un cuarto nivel (`1.2.3.4`) es
señal de que la estructura del capítulo está mal planteada.

La jerarquía se construye por **contraste acumulado**, no por tamaño solo:

| Nivel | Señales |
|---|---|
| Capítulo | Página nueva · rótulo en versalitas · cuerpo grande · filete · espacio generoso arriba |
| Sección | Negrita · 14 pt · número en color de acento |
| Subsección | Negrita cursiva · 12 pt |

Un documento con cinco tamaños de letra, cuatro pesos y tres colores no tiene
jerarquía: tiene ruido. Los saltos deben ser evidentes a 2 m de distancia en una
impresión; si hay que medir para distinguir dos niveles, no se distinguen.

---

## 3 · Tablas

Reglas, en orden de importancia:

1. **Sin trazos verticales.** Jamás. El espacio en blanco separa columnas mejor
   que una línea.
2. **Tres trazos horizontales**: encima de la cabecera, bajo la cabecera, al
   final (`\toprule`, `\midrule`, `\bottomrule`). Trazos internos solo para
   separar paneles.
3. **Leyenda encima** de la tabla (y debajo de la figura). Convención editorial
   universal en ciencia; las plantillas ya lo hacen.
4. **Números alineados por el separador decimal**, con el mismo número de
   decimales en toda la columna. Una columna con `2,4`, `0,533` y `91,7` no se
   puede comparar de un vistazo.
5. **Cifras significativas honestas.** Reporte lo que el error estándar
   justifica. `2,3846974096450877` no es precisión: es falta de criterio.
6. **Unidades en la cabecera**, no repetidas en cada celda: `Sesgo (pp.)`.
7. Nota al pie de tabla para abreviaturas y para el supuesto que la condiciona.
8. Si la tabla tiene más de ~15 filas o 8 columnas, probablemente es una figura.

Una tabla debe entenderse **sin leer el texto**: alguien que la recorta y la pega
en otro documento debe saber qué muestra, de cuántas observaciones y bajo qué
condición.

---

## 4 · Figuras

| Regla | Detalle |
|---|---|
| Vectorial siempre que se pueda | PDF o SVG para gráficos generados por código; PNG a **300 dpi mínimo** solo para mapas de bits |
| Tipografía de la figura = tipografía del documento | Una figura de Matplotlib con DejaVu Sans dentro de un documento en serif se ve como lo que es: pegada |
| Cuerpo de las etiquetas ≥ 8 pt **en tamaño final** | Genere la figura al ancho real al que se insertará, no la escale después |
| Sin marco exterior, sin cuadrícula pesada, sin fondo gris | El eje izquierdo y el inferior bastan |
| Sin 3D, sin sombras, sin degradados, sin efectos | Distorsionan la lectura de magnitudes |
| Etiquetado directo antes que leyenda | Rotular la línea junto a la línea evita el viaje de ida y vuelta a la leyenda |
| El cero en el eje **y** cuando la longitud codifica magnitud (barras) | Opcional en líneas y dispersión, donde lo que importa es la variación |

### Color

- **Diseñe en gris primero.** Si la figura funciona en escala de grises,
  funcionará impresa, fotocopiada y para un lector con deficiencia de visión
  cromática. El color se añade después, como refuerzo.
- **Nunca color como único portador de información.** Doble codificación: color
  + tipo de línea, o color + marcador, o color + etiqueta.
- **Máximo 5–6 categorías** por figura. Con más, divida en paneles.
- Evite el rojo–verde como par opuesto (≈ 8 % de los varones no lo distingue).
- Paletas seguras: *viridis* y *cividis* para secuencial; esquemas divergentes
  azul–naranja o azul–marrón para diferencias con punto neutro. Verifique su
  figura con un simulador de daltonismo antes de entregar.

### Leyendas (*captions*)

Una leyenda es una **oración afirmativa que dice lo que la figura muestra**, no
un rótulo. Compare:

> ✗ «Cobertura por tamaño de muestra.»
> ✓ «Cobertura empírica de los intervalos al 95 % según el tamaño de muestra; la
> línea discontinua marca el nivel nominal.»

La segunda permite leer la figura sin volver al texto, que es exactamente lo que
hace un lector que hojea.

---

## 5 · Ecuaciones en la página

- Ecuación **desplegada** si es central, si se referencia, o si no cabe cómoda en
  línea; **en línea** en los demás casos.
- Numere **solo** las ecuaciones que referencia. Numerar todas es ruido.
- La ecuación es parte de la oración: lleva la puntuación que le corresponde
  (coma o punto al final).
- No empiece un párrafo con una ecuación desplegada, y no deje una ecuación
  desplegada huérfana entre dos párrafos sin conexión gramatical.

Detalle en [`03-guia-formulas.md`](03-guia-formulas.md).

---

## 6 · Cosas que delatan un documento descuidado

Un jurado las registra incluso sin nombrarlas:

- Viudas y huérfanas: una línea sola al pie o al inicio de página.
- Título al pie de página con su texto en la siguiente.
- Figura separada por tres páginas del párrafo que la menciona.
- Dos convenciones decimales en el mismo documento.
- Comillas rectas `"así"` en vez de tipográficas `«así»` o `"así"`.
- Guion (`-`) donde va raya de diálogo o guion largo (`—`), o raya donde va
  guion de rango (`–`).
- URLs sin partir que desbordan el margen.
- Tablas con líneas verticales.
- Figuras de distinta familia tipográfica entre sí.
- Numeración de ecuaciones correlativa global (`(47)`) en lugar de por capítulo.

---

## 7 · Implementación en las plantillas

| Qué | LaTeX | Typst |
|---|---|---|
| Tablas sin verticales | `booktabs`: `\toprule \midrule \bottomrule` | `table(stroke: none)` + `table.hline()` |
| Leyenda de tabla arriba | `\captionsetup[table]{position=top}` | `show figure.where(kind: table): set figure.caption(position: top)` |
| Alineación decimal | `siunitx`: `S[table-format=1.3]` | columnas `right` + decimales fijos en el generador |
| Partición de palabras | activa con `babel`+`spanish` | `set text(hyphenate: true)` |
| Microtipografía | `\usepackage[protrusion=true]{microtype}` | activa por defecto |
| URLs que se parten | `\usepackage{xurl}` | automático |
| Figuras vectoriales | `\includegraphics{fig.pdf}` | `image("fig.svg")` |

Para generar figuras consistentes con el documento: fije en el *script* de
Python o R la familia serif, el cuerpo en puntos y el ancho en pulgadas igual al
ancho de texto real (en A4 con los márgenes de las plantillas, ≈ 13 cm ≈ 5,1 in),
y exporte a PDF o SVG. No reescale la figura al insertarla.
