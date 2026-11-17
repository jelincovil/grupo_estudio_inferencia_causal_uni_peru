// Documento de prueba: verifica que el paquete uni-tesis compila.
#import "preview/uni-tesis/0.1.0/lib.typ": *

#let meta = (
  titulo: "Documento de prueba del paquete uni-tesis",
  autor: "Nombre del Autor",
  asesor: "Dr. Nombre del Asesor",
  grado: "Magíster en Ciencias e Ingeniería Estadística",
  tipo-doc: "Tesis de maestría",
  universidad: "Universidad Nacional de Ingeniería",
  facultad: "Facultad de Ingeniería Económica, Estadística y Ciencias Sociales",
  unidad: "Unidad de Posgrado",
  ciudad: "Lima — Perú",
  anio: 2026,
  logo: "../../../../plantilla_tesis_uni_typst/logo_uni.png",
  palabras-clave: ("prueba", "typst"),
  resumen-corto: "Documento mínimo para validar el paquete uni-tesis.",
  repositorio: "https://github.com/jelincovil/grupo_estudio_inferencia_causal_uni_peru",
)

#show: tesis.with(meta)
#portada(meta)

#show: frontmatter
= Resumen
Resumen de prueba del paquete.

#show: mainmatter
= Introducción
#teorema(nombre: "De prueba")[Todo paquete bien formado compila.]
Sea $bold(y) = X beta + epsilon$, con $E[epsilon] = 0$.

#definicion[Definición de prueba del entorno.]

```typ
#teorema(nombre: "X")[…]
```

#show: apendices
= Apéndice A
Contenido del apéndice.
