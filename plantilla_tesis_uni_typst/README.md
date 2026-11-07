# Plantilla de tesis UNI · versión Typst

Plantilla de tesis de **Maestría / Doctorado** para la Universidad Nacional de
Ingeniería (Facultad de Ingeniería Económica, Estadística y Ciencias Sociales),
reescrita en [Typst](https://typst.app) a partir de la plantilla LaTeX del
[Grupo de Estudio de Inferencia Causal UNI Perú](https://github.com/jelincovil/grupo_estudio_inferencia_causal_uni_peru),
con criterio editorial: composición clásica de libro, aperturas de capítulo,
cabeceras vivas, entornos matemáticos con numeración compartida, tablas al
estilo *booktabs* y bibliografía APA localizada al español.

El documento de ejemplo (`tesis.typ`) es una **demo completa y coherente** —
una mini-tesis sobre estimación causal con doble aprendizaje automático—:
todo el texto es real (nada de *lorem ipsum*), las cifras de las tablas y de
las figuras provienen de un experimento Monte Carlo reproducible y la
bibliografía cita obras existentes.

---

## Requisitos

| Herramienta | Versión | Notas |
|---|---|---|
| Typst | 0.15 o superior | `cargo install typst-cli` o binario de [releases](https://github.com/typst/typst/releases) |
| Python (solo para regenerar figuras) | 3.10+ | `pip install -r requirements.txt` |

La tipografía (**New Computer Modern**) viene incluida con Typst: no hay que
instalar fuentes. Compilar en [typst.app](https://typst.app) funciona igual
(suba la carpeta completa).

## Compilación

```bash
./compilar.sh               # produce tesis.pdf
./compilar.sh vigilante     # recompila al guardar
# o directamente:
typst compile tesis.typ
```

## Estructura del paquete

```
tesis_uni_typst/
├── tesis.typ               ← documento principal (contenido: edítelo)
├── plantilla.typ           ← módulo de estilo (tipografía, entornos, índices)
├── referencias.bib         ← base bibliográfica BibLaTeX/BibTeX
├── logo_uni.png            ← emblema institucional (portada)
├── figuras/                ← figuras del ejemplo (PNG 300 dpi + QR en SVG)
├── generar_recursos.py     ← regenera figuras y cifras del experimento demo
├── results.json            ← cifras del experimento Monte Carlo de la demo
├── verificar_cifras.py     ← comprueba coherencia Tabla 4.2 ↔ results.json
├── requirements.txt        ← versiones fijadas del entorno Python
├── compilar.sh             ← script de compilación
└── README.md               ← esta guía
```

## Primeros pasos

1. **Edite el bloque `meta`** al inicio de `tesis.typ` (título, autor, asesor,
   grado, palabras clave, año). La portada, los metadatos del PDF y el resumen
   se actualizan solos.
2. Reemplace el contenido de cada capítulo. La estructura sigue la guía
   metodológica de la Unidad de Posgrado: preliminares (dedicatoria,
   agradecimientos, resumen/abstract, copia de documentos, índices),
   introducción con hipótesis y variables, marco teórico con definición de
   términos, metodología, resultados, discusión, conclusiones y
   recomendaciones, referencias y apéndices con matrices de consistencia y
   operacionalización.
3. Compile. Los índices (general, de figuras, de tablas), la numeración por
   capítulo y las referencias cruzadas se resuelven automáticamente.

## Anatomía de `tesis.typ`

```typst
#import "plantilla.typ": *
#show: tesis.with(meta)   // activa todo el aparato de estilo

#portada(meta)            // portada institucional

#show: frontmatter        // ← preliminares: numeración romana (i, ii, …)
#prefacio([Dedicatoria])[ … ]
#prefacio([Resumen])[ … ]
#outline(title: [Índice general], depth: 2)
#outline(target: figure.where(kind: image), title: [Índice de figuras])
#outline(target: figure.where(kind: table),  title: [Índice de tablas])

#show: mainmatter         // ← cuerpo: numeración arábiga + cabecera viva
= Introducción            // capítulo (apertura editorial con filete)
== Motivación y contexto  // sección 1.1
=== Objetivo general      // subsección 1.1.1

#show: apendices          // ← apéndices: numeración A, B, …
= Matriz de consistencia

#bibliography("referencias.bib", style: "apa",
              title: [Referencias bibliográficas])
```

### Entornos lógico-matemáticos

Numeración compartida por capítulo (Definición 2.1, Teorema 2.2, …), al
estilo *amsthm*. Cada uno admite `nombre:` y `id:` para referencias:

```typst
#definicion(nombre: [Espacio de probabilidad], id: "def:prob")[
  Un espacio de probabilidad es una terna $(Omega, cal(F), P)$…
]

#teorema(nombre: [Teorema central del límite], id: "thm:clt")[ … ]
#demostracion[ … ]        // termina con lápida (QED) automática

#observacion[ … ]         // versión ligera (filete gris, cuerpo menor)
```

Disponibles: `teorema`, `lema`, `corolario`, `proposicion`, `definicion`,
`axioma`, `observacion`, `nota`, `ejemplo` y `demostracion`.

### Figuras, tablas y código

```typst
#figure(image("figuras/mi.png", width: 85%), caption: [ … ]) <fig:mi>

#figure(
  table(
    columns: (2.2fr, 1fr, 1fr),
    table.hline(stroke: 1pt),
    table.header([*Variable*], [*Media*], [*EE*]),
    table.hline(stroke: 0.5pt),
    [Ingreso], [1250.4], [12.1],
    table.hline(stroke: 1pt),
  ),
  caption: [ … ],
) <tab:ingreso>

#figure(```python … ```, caption: [ … ]) <cod:uno>   // listado numerado
```

- Numeración por capítulo: `Figura 2.1`, `Tabla 4.2`, `Código 3.1` (en
  apéndices: `A.1`, `A.2`, …).
- Referencias cruzadas: `@fig:mi`, `@tab:ingreso`, `@thm:clt` — el prefijo
  de capítulo se evalúa en el destino, no en el punto de cita.
- Las tablas se componen sin trazos verticales (estilo *booktabs*) y su
  leyenda va encima; las figuras, debajo.

### Algoritmos y avisos

```typst
#algoritmo([Nombre del algoritmo], id: "alg:uno")[ …pseudocódigo… ]
#aviso([Instrucciones])[ nota editorial para el lector ]
```

### Bibliografía (APA)

Las citas son autor-año y se localizan al español automáticamente:

```typst
Como muestra @efron1979bootstrap, …
#bibliography("referencias.bib", style: "apa", title: [Referencias bibliográficas])
```

## Personalización rápida

Todo vive en `plantilla.typ`:

| Qué | Dónde |
|---|---|
| Paleta (acento, tintas, filetes) | bloque «1 · Paleta y constantes» |
| Interlineado (norma 1,5) | `set par(leading: 0.76em)` — reduzca a `0.65em` para texto corrido |
| Sangría de primera línea | `set par(first-line-indent: (amount: 1.25cm, all: false))` |
| Márgenes de página | `set page(margin: …)` (izq. 3,4 · der. 2,6 · sup. 3,1 · inf. 3 cm) |
| Tipografía | `set text(font: "New Computer Modern")` — cámbiela por `"Libertinus Serif"` o `"Liberation Serif"` (estilo Times) si su unidad lo exige |
| Rótulo de capítulo | `rotulo` en la regla `show heading.where(level: 1)` |

## Reproducibilidad de la demo

### Generador y variables (simulación)

| Símbolo | Definición | Rol |
|---|---|---|
| $X_1,\ldots,X_5$ | $X_1,X_3,X_4,X_5 \sim N(0,1)$; $X_2 \sim t_3/\sqrt{3}$ | Confusores y ruido |
| $e(X)$ | $\mathrm{logit}^{-1}(1.1 X_1 - 0.7 X_2 + 0.9(X_3^2-1))$ | Propensión |
| $A$ | $\mathrm{Bernoulli}(e(X))$ | Tratamiento |
| $Y$ | $\tau A + m(X) + \varepsilon$, $\tau=2$, $\varepsilon \sim N(0,1.5^2)$ | Respuesta |
| Réplica $i$ | semilla `2026 + 17·i` | Reproducibilidad |

Estimando: **ATE** $\tau = E[Y(1)-Y(0)]$. Supuestos del generador: no confusión
condicional, positividad y SUTVA (unidades independientes).

Las cifras de la Tabla 4.2, las figuras de cobertura y bootstrap y el QR
provienen de `generar_recursos.py` (generador calibrado, cuatro estimadores,
semillas fijas):

```bash
OMP_NUM_THREADS=1 python3 generar_recursos.py        # todo (~5 min, 2 núcleos)
OMP_NUM_THREADS=1 python3 generar_recursos.py 250    # un escenario
OMP_NUM_THREADS=1 python3 generar_recursos.py final  # figuras + results.json
```

Si edita el experimento, regenere y actualice las cifras del texto de
`tesis.typ` con los valores de `results.json`, luego ejecute:

```bash
python3 verificar_cifras.py   # debe imprimir OK
```

## Lista de verificación previa a la entrega

- [ ] Metadatos del bloque `meta` completos (título, autor, asesor, año).
- [ ] «Copia de Documentos» reemplazada por los documentos exigidos.
- [ ] Figuras y tablas con leyendas descriptivas propias (no las de la demo).
- [ ] Todas las referencias cruzadas resuelven (compile sin errores).
- [ ] Nombres reales en portada (sin «Nombre del Autor»).
- [ ] El resumen y el abstract reflejan su investigación.
- [ ] La numeración de páginas es la exigida por su unidad.

## Licencia

Esta plantilla hereda la licencia de la obra original
([CC BY 4.0](https://creativecommons.org/licenses/by/4.0/)). El texto de
ejemplo es contenido demostrativo y puede reemplazarse libremente.
