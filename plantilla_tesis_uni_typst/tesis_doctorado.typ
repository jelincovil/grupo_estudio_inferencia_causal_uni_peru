// =====================================================================
// tesis_doctorado.typ — Esqueleto doctoral (compendio / tres estudios)
// Véase documentacion/00-modelo-tesis.md §4. Compile:
//   typst compile tesis_doctorado.typ
// La demo causal completa está en tesis.typ (nivel maestría / artículo único).
// =====================================================================
#import "plantilla.typ": *

#let meta = (
  titulo: "Título de la tesis doctoral — sustituya por su contribución integrada",
  autor: "Nombre del Autor",
  asesor: "Dr. Nombre del Asesor",
  grado: "Doctorado en Ciencias e Ingeniería Estadística (verifique en su programa)",
  tipo-doc: "Tesis doctoral",
  universidad: "Universidad Nacional de Ingeniería",
  facultad: "Facultad de Ingeniería Económica, Estadística y Ciencias Sociales",
  unidad: "Unidad de Posgrado",
  ciudad: "Lima — Perú",
  anio: 2026,
  logo: "logo_uni.png",
  palabras-clave: ("inferencia causal", "tesis doctoral", "reproducibilidad"),
  resumen-corto: "Síntesis de tres estudios articulados por una contribución causal común.",
  repositorio: "https://github.com/jelincovil/grupo_estudio_inferencia_causal_uni_peru",
)

#show: tesis.with(meta)
#portada(meta)
#show: frontmatter

#prefacio([Resumen])[
  Redacte un resumen que integre los tres estudios: problema común, aporte
  metodológico o teórico, evidencia y límite principal. Incluya el
  estimando causal central y los supuestos de identificación compartidos.
  #strong[Palabras clave:] (edite).
]

#prefacio([Abstract])[
  English abstract mirroring the integrated contribution (not three separate
  abstracts pasted together).
]

#prefacio([Copia de Documentos])[
  Documentos exigidos por la Unidad de Posgrado. Normativa: @uni2023reglamentoposgrado
  y `documentacion/06-biblioteca-enlaces.md`.
]

#outline(title: [Índice general], depth: 2)

#show: mainmatter

= Introducción
== Motivación y contexto
// …

== Planteamiento del problema
Declare el estimando ($tau$ u otro) y por qué ningún estudio previo lo resolvió solo.

== Objetivos
=== Objetivo general
=== Objetivos específicos (uno por estudio + integración)

== Hipótesis y variables

== Justificación

== Estructura de la tesis
Los capítulos 4–6 corresponden a los estudios I–III; el capítulo 7 articula lo
que ninguno dice por separado.

= Marco teórico
== Antecedentes
== Fundamentos matemáticos
== Definición de términos
== Estado del arte

== Brecha de conocimiento <brecha>
Nombre el trabajo más cercano (autor, año, qué hizo) y lo que *no* resolvió.
La contribución de esta tesis se define *contra* ese trabajo.

#aviso([Prueba de suficiencia doctoral])[
  Una oración: «Antes de esta tesis no se sabía ___; ahora se sabía porque ___.»
  Si el segundo hueco es solo «apliqué X a Y», reencuadre el alcance.
]

= Metodología transversal
== Diseño global (monografía / compendio)
== Estimando y supuestos compartidos
== Reproducibilidad y repositorio
Enlace, semillas, `requirements.txt`, `reproduce.sh`.

= Estudio I — Fundamento teórico o metodológico
Propiedades del estimador, condiciones de regularidad, demostración o extensión
propia. Rótulo de evidencia: teoría / simulación según corresponda.

= Estudio II — Evidencia por simulación
Generador calibrado, escenarios, sesgo / RMSE / cobertura, sensibilidad
obligatoria (`documentacion/00-modelo-tesis.md` §2).

= Estudio III — Aplicación a datos reales
Registro o encuesta; estructura de confusión contrastada con dominio.
Trazabilidad: cada cifra en archivo de resultados verificado.

= Discusión integrada
Qué aprenden los tres estudios *juntos* que ninguno enseña solo. Si este
capítulo puede borrarse sin pérdida, la tesis no está integrada.

= Conclusiones y recomendaciones
== Conclusiones
== Recomendaciones
== Limitaciones
== Líneas futuras

#bibliography("referencias.bib", style: "apa", title: [Referencias bibliográficas])

#show: apendices

= Apéndice de reproducibilidad
Semillas, versiones, comando de regeneración, `verificar_cifras.py` (véase demo en `tesis.typ`).

= Matriz de consistencia
Incluya estimando, supuestos de identificación y criterio de falsación.

= Matriz de operacionalización

= Demostraciones técnicas (Estudio I)

= Código fuente

= Datos suplementarios
