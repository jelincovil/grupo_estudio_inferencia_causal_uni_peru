# Modelo de tesis · maestría y doctorado

Propuesta de estructura para las tesis del grupo. Respeta la secuencia exigida
por la Unidad de Posgrado (preliminares → introducción con hipótesis y
variables → marco teórico con definición de términos → metodología →
resultados → discusión → conclusiones y recomendaciones → referencias →
apéndices con matrices) y **diferencia por dentro** lo que distingue una tesis
de maestría de una de doctorado.

> **Antes de empezar:** la autoridad sobre márgenes, portada, carátula y
> empastado es el *Formato de Presentación de Tesis de la UNI* y la normativa de
> la FIEECS, no esta guía ni las plantillas. Verifíquelo en las fuentes de
> [`06-biblioteca-enlaces.md`](06-biblioteca-enlaces.md) §1 **antes** de maquetar.
> Esta guía cubre lo que la normativa no regula: qué debe *decir* cada capítulo.

---

## 1 · La diferencia de fondo

No es extensión. Es la **naturaleza de la afirmación que el documento defiende**.

| | Maestría | Doctorado |
|---|---|---|
| Afirmación central | «Apliqué correctamente un método conocido a un problema que importa, y sé por qué podía fallar.» | «Esto no se sabía antes, y aquí está la razón por la que ahora se sabe.» |
| Unidad de contribución | **Un** estudio ejecutado con rigor | **Tres** estudios articulados por una tesis común (monografía o compendio) |
| Obligación teórica | Usar los teoremas correctamente y citar su fuente | Demostrar o extender al menos un resultado propio |
| Estado del arte | Mapa competente del campo (≈ 40–60 referencias) | Síntesis crítica con frontera identificada (≈ 100–150 referencias) |
| Novedad | Puede ser el **objeto** (datos peruanos nuevos, aplicación inédita) | Debe ser el **método, la teoría o la evidencia**, y debe ser falsable |
| Qué se defiende en la sustentación | La ejecución | La frontera |
| Extensión orientativa del cuerpo | 60–100 pp. | 140–220 pp. |

**Prueba de suficiencia doctoral.** Escriba en una sola oración, sin adjetivos:
«Antes de esta tesis no se sabía ___; ahora se sabe porque ___.» Si la oración
no se puede escribir, o si el segundo hueco se llena con «apliqué X a Y», el
trabajo es de maestría —lo cual no es un defecto, es una categoría.

---

## 2 · La capa causal obligatoria

Todo trabajo del grupo, en los dos niveles, declara explícitamente la cadena
completa. Esto va en Metodología y se audita en Discusión; **no** se deja
implícito.

| Eslabón | Qué se declara | Maestría | Doctorado |
|---|---|---|---|
| **Estimando** | El parámetro causal, en notación de resultados potenciales: ATE, ATT, LATE, CATE. No «el efecto» | Uno, justificado por la pregunta de política | Uno principal + los secundarios que la heterogeneidad exija |
| **Identificación** | Supuestos nombrados uno por uno: no confusión condicional, positividad/*overlap*, SUTVA/consistencia, y los adicionales del diseño | Tabla supuesto ↔ estimador ↔ amenaza | Lo mismo + discusión de qué observación falsaría cada supuesto |
| **Estimación** | El estimador, su error estándar y de dónde sale (función de influencia, bootstrap, sandwich) | Un estimador principal + un comparador | Un estimador principal + al menos dos comparadores de familias distintas |
| **Sensibilidad** | Qué tan equivocada puede estar la conclusión si un supuesto falla | **Al menos una** sensibilidad cuantitativa | **Programa** de sensibilidad: confusión no observada (E-value / límites de Rosenbaum), violación de positividad, especificación alternativa |

La omisión que este grupo ve con más frecuencia es la cuarta. Un intervalo de
confianza estrecho sobre un estimador sesgado es el resultado más peligroso que
una tesis puede reportar, porque es indistinguible de un resultado bueno por
cualquier diagnóstico interno. La demo del repositorio existe precisamente para
hacer visible ese fallo.

---

## 3 · Esqueleto de maestría

```
Preliminares
  Dedicatoria · Agradecimientos · Resumen · Abstract
  Copia de documentos · Índices (general, figuras, tablas)

1  Introducción                                              8–12 pp.
   1.1  Motivación y contexto
   1.2  Planteamiento del problema         ← la pregunta, no el tema
   1.3  Objetivos (general + 3 específicos)
   1.4  Hipótesis y variables              ← exigido por la guía UNI
   1.5  Justificación (teórica · metodológica · práctica)
   1.6  Estructura de la tesis

2  Marco teórico                                            18–28 pp.
   2.1  Antecedentes
   2.2  Fundamentos matemáticos            ← definiciones y teoremas citados
   2.3  Definición de términos             ← exigido por la guía UNI
   2.4  Estado del arte

3  Metodología                                              15–22 pp.
   3.1  Diseño de la investigación
   3.2  Población, muestra y fuente de datos
   3.3  Estimando y supuestos de identificación   ← §2 de esta guía
   3.4  Procedimiento y plan de análisis
   3.5  Técnicas estadísticas
   3.6  Implementación computacional        ← versiones fijadas + repositorio

4  Resultados                                               15–25 pp.
   4.1  Análisis descriptivo
   4.2  Inferencia
   4.3  Sensibilidad                       ← no opcional
   4.4  Validación

5  Discusión                                                 8–12 pp.
6  Conclusiones y recomendaciones                            6–10 pp.
   6.1 Conclusiones  6.2 Recomendaciones  6.3 Limitaciones  6.4 Líneas futuras

Referencias
Apéndices
   A  Matriz de consistencia (ampliada, §5)
   B  Matriz de operacionalización
   C  Demostraciones
   D  Código fuente
   E  Datos suplementarios
```

Cada conclusión de §6.1 debe poder rastrearse a un resultado numerado de §4 y a
un objetivo de §1.3. Si una conclusión no tiene ese par de anclas, no es una
conclusión: es una opinión.

---

## 4 · Esqueleto de doctorado

Igual hasta el capítulo 3, con tres cambios estructurales:

**a) Marco teórico con frontera explícita.** Se añade `2.5 Brecha de
conocimiento`: media página que nombra el trabajo previo más cercano (autor,
año, qué hizo) y dice qué no resolvió. La contribución de la tesis se define
*contra* ese trabajo, no en el vacío.

**b) Tres capítulos de resultados en lugar de uno.**

```
4  Estudio I  — fundamento teórico o metodológico
      propiedades del estimador, demostración, condiciones de regularidad
5  Estudio II — evidencia por simulación
      generador calibrado, escenarios, sesgo / RMSE / cobertura
6  Estudio III — aplicación a datos reales
      registro administrativo o encuesta, con la estructura de confusión
      contrastada contra conocimiento de dominio
7  Discusión integrada
      qué dicen los tres juntos que ninguno dice solo
8  Conclusiones y recomendaciones
```

El capítulo 7 es el que separa una tesis doctoral de tres papers engrapados. Si
se puede borrar sin pérdida, la tesis no está integrada.

**c) Apéndice de reproducibilidad completo.** Semillas, versiones exactas,
tiempo de cómputo, *hash* de los archivos de resultados, y la instrucción exacta
que regenera cada cifra del texto. Véase §6.

En formato **compendio** (tres artículos publicados o sometidos), los capítulos
4–6 son los artículos con un preámbulo que declara la contribución del
candidato en cada uno, y los capítulos 1–3 y 7–8 se escriben *ex profeso* para
la tesis. La integración del capítulo 7 sigue siendo obligatoria.

---

## 5 · Matriz de consistencia ampliada

Las plantillas traen la matriz estándar (problema, objetivos, hipótesis,
variables, metodología). Para un trabajo causal, **añada tres filas**; son las
que un jurado con formación estadística va a interrogar primero:

| Elemento | Formulación |
|---|---|
| Problema | … |
| Objetivo general | … |
| Objetivos específicos | … |
| Hipótesis | … |
| Variables | … |
| Metodología | … |
| **Estimando** | **El parámetro causal en notación formal: τ = E[Y(1) − Y(0)]** |
| **Supuestos de identificación** | **No confusión condicional · positividad · SUTVA · (otros del diseño)** |
| **Criterio de falsación** | **Qué resultado observable haría insostenible la hipótesis** |

La última fila es la más incómoda y la más valiosa. Una hipótesis que ningún
resultado posible podría refutar no es una hipótesis científica, y escribir esa
fila obliga a descubrirlo antes de la sustentación en lugar de durante.

---

## 6 · Requisito de reproducibilidad

Mínimo exigible en los dos niveles:

- [ ] Repositorio público (o institucional con DOI) enlazado desde la tesis, con QR en Metodología.
- [ ] Entorno fijado: `requirements.txt` / `renv.lock` / `environment.yml` con **versiones exactas**, no rangos.
- [ ] Semillas declaradas en el texto, no solo en el código.
- [ ] Un comando que regenera todas las cifras y figuras desde cero.
- [ ] Un verificador que compara las cifras del texto contra los resultados guardados y falla si difieren.
- [ ] Definición operativa de cada variable en una tabla, con unidades.

La plantilla Typst del repositorio implementa los últimos tres puntos
(`generar_recursos.py`, `results.json`, `verificar_cifras.py`): úselos como
patrón, no los reinvente.

Si los microdatos son reservados, publique el código, el diccionario de
variables, los agregados y el procedimiento de solicitud de acceso. Sin
microdatos puede no haber replicación, pero siempre puede haber
**auditabilidad**; no son lo mismo y la tesis debe decir cuál ofrece.

---

## 7 · Hitos de calidad antes de la sustentación

| Hito | Criterio de aprobación |
|---|---|
| Pregunta | Cabe en una oración, con estimando nombrado |
| Identificación | Cada supuesto tiene su amenaza y su diagnóstico |
| Resultados | Cada cifra del texto existe en el archivo de resultados |
| Sensibilidad | Hay al menos un análisis que podría haber contradicho la conclusión |
| Trazabilidad | Cada conclusión apunta a un resultado y a un objetivo |
| Referencias | Cada entrada tiene DOI o handle, y cada enlace se abrió |
| Compilación | El documento compila limpio desde un clon nuevo del repositorio |
| Lenguaje | Ninguna afirmación causal sin su supuesto; ningún `TODO` ni `lipsum` |

---

*Véase también:* [`01-guia-editorial.md`](01-guia-editorial.md) para cómo
redactar cada sección y [`03-guia-formulas.md`](03-guia-formulas.md) para la
notación de estimandos.
