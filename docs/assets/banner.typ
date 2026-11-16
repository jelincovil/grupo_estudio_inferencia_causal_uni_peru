// Banner del repositorio (README / vista previa social).
// Compilar: typst compile docs/assets/banner.typ docs/assets/banner.png
#set page(width: 1280pt, height: 640pt, margin: 0pt)
#set text(font: "New Computer Modern", fill: white)

#rect(width: 100%, height: 100%, fill: rgb("0b1f3a"))
#place(left + top, dx: 0pt, dy: 500pt, rect(width: 1280pt, height: 6pt, fill: rgb("ffb700")))

#place(left + top, dx: 90pt, dy: 150pt)[
  #text(size: 26pt, fill: rgb("ffb700"), tracking: 2pt)[GRUPO DE ESTUDIO · UNI · PERÚ]
]

#place(left + top, dx: 86pt, dy: 215pt)[
  #text(size: 78pt, weight: "bold")[Plantillas de\ tesis UNI]
]

#place(left + top, dx: 90pt, dy: 435pt)[
  #text(size: 30pt, fill: rgb("dce6f5"))[Typst · LaTeX · APA · Licencia CC BY 4.0]
]

#place(left + bottom, dx: 90pt, dy: -68pt)[
  #text(size: 17pt, fill: rgb("8fa8c8"))[github.com/jelincovil/grupo_estudio_inferencia_causal_uni_peru]
]
