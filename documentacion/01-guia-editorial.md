# Guía editorial

Cómo se redacta cada parte de una tesis del grupo. Las reglas están ordenadas de
mayor a menor consecuencia: las primeras afectan la validez de lo que afirma el
documento; las últimas, su legibilidad.

---

## 1 · Vocabulario causal: la regla que no se negocia

El español académico permite ambigüedad causal con demasiada facilidad. En los
documentos del grupo, estos verbos están **reservados** para afirmaciones
identificadas, con sus supuestos declarados en la misma sección:

> *causa, produce, provoca, reduce, aumenta, mejora, genera, el efecto de,
> el impacto de*

Para asociaciones use: *se asocia con, covaría con, predice, está correlacionado
con, va acompañado de*. Y escriba la dirección cuando la tenga.

| ✗ No | ✓ Sí |
|---|---|
| «El programa redujo la pobreza en 4 puntos.» | «Bajo no confusión condicional y positividad (§3.3), el ATE estimado es −4.0 puntos (IC 95 %: −5.2, −2.8).» |
| «Los resultados muestran que X mejora Y.» | «El estimador doblemente robusto atribuye a X una mejora de 0.31 DE en Y; el supuesto crítico es ___ y su violación en magnitud ___ anularía el signo.» |
| «Existe un efecto significativo.» | «El intervalo al 95 % excluye el cero; la magnitud (0.31) es ___ en términos de la escala de política.» |

«Significativo» a secas está prohibido: o se dice *estadísticamente
significativo* con su nivel, o se dice *relevante en magnitud* con su escala. Son
dos afirmaciones distintas y confundirlas es el error de interpretación más
frecuente en las tesis aplicadas.

---

## 2 · Tiempos verbales por sección

| Sección | Tiempo | Por qué |
|---|---|---|
| Antecedentes, estado del arte | Presente para el conocimiento establecido; pasado para el estudio concreto | «El bootstrap aproxima la distribución…»; «Efron (1979) formalizó…» |
| Metodología | Pasado | Describe lo que se hizo, una vez |
| Resultados | Pasado para los hallazgos; presente para lo que muestran las tablas | «El sesgo se estabilizó en +0.95»; «La Tabla 4.2 resume…» |
| Discusión, conclusiones | Presente | Son afirmaciones vigentes |
| Limitaciones, líneas futuras | Presente / condicional | «El alcance es…»; «Una extensión natural sería…» |

Primera persona: **plural de modestia** («estimamos», «mostramos») o impersonal
(«se estima»), elegido una vez y mantenido en todo el documento. La mezcla
delata un texto escrito por partes y pegado.

---

## 3 · Arquitectura del párrafo

Un párrafo = una afirmación. El patrón que mejor funciona en texto estadístico:

1. **Oración temática** — la afirmación, sin preámbulo.
2. **Evidencia** — la cifra, la tabla, el teorema.
3. **Calificación** — el supuesto, el límite, la condición.
4. **Enlace** — qué sigue de ahí.

Ejemplo (de la demo del repositorio, abreviado):

> *El sesgo es un atributo del diseño, no del tamaño de muestra.* [tema] La
> diferencia de medias sobreestima el efecto en +2.39 de manera prácticamente
> idéntica en los tres escenarios, y el ajuste lineal se estabiliza alrededor de
> +0.95. [evidencia] Omitir el término X₃² deja un canal de confusión que ningún
> volumen de datos cierra. [calificación] Su cobertura lo confirma: 9.3 % con
> n = 250 y 0 % a partir de n = 1000. [enlace]

Longitud: 4–7 oraciones. Un párrafo de dos oraciones casi siempre pertenece al
anterior; uno de doce contiene dos párrafos.

---

## 4 · Secciones difíciles

### Resumen (150–250 palabras, una sola pieza)

Cinco movimientos, una o dos oraciones cada uno:

1. **Problema** — qué no se sabe y por qué importa.
2. **Qué se hizo** — diseño y método, con el estimando nombrado.
3. **Resultado principal** — **con cifras**. Un resumen sin números no informa.
4. **Qué significa** — la interpretación, calificada.
5. **Alcance** — la limitación dominante.

No incluye: citas, referencias a secciones, abreviaturas no definidas, ni la
frase «se presenta un trabajo que…». El *abstract* es traducción, no reescritura:
las mismas cifras, la misma estructura.

### Planteamiento del problema

Lo que se plantea es una **pregunta**, no un tema. «La inferencia causal en datos
observacionales» es un tema. «¿Qué costo en cobertura tiene ajustar confusión no
lineal con un modelo lineal?» es una pregunta. La sección debe terminar con una
o tres preguntas en interrogativo literal.

### Discusión

Cuatro obligaciones, en este orden:

1. **Interpretar**, no repetir. Si un párrafo de Discusión se puede mover a
   Resultados sin que nadie lo note, está de más.
2. **Confrontar** con la literatura: ¿coincide con lo previo? ¿contradice? ¿por qué?
3. **Atacar su propio trabajo.** La amenaza más seria a su conclusión, nombrada
   por usted antes que por el jurado, y la razón por la que la conclusión
   sobrevive —o el reconocimiento de que se debilita.
4. **Delimitar** el alcance: de qué población, bajo qué supuestos, en qué rango.

El punto 3 es contraintuitivo y es el que distingue una discusión madura. Un
jurado que encuentra una debilidad no declarada concluye que el autor no la vio;
uno que la encuentra ya declarada y acotada concluye que el autor domina el
problema.

### Conclusiones

Numeradas, una por objetivo específico, en el mismo orden que §1.3. Cada una
contiene la cifra que la sostiene. No introducen material nuevo ni citan.

---

## 5 · Lo que se elimina en la revisión final

Relleno sin contenido proposicional:

> *es importante señalar que · cabe destacar que · en el presente trabajo se
> procederá a · como es bien sabido · de alguna manera · diversos autores
> señalan (sin decir quiénes) · en los últimos años ha habido un creciente
> interés*

Intensificadores que no miden: *muy, bastante, sumamente, altamente,
extremadamente, significativamente* (en sentido no técnico). Si la magnitud
importa, dé el número; si no importa, no la adjetive.

«Diversos autores señalan» con cita es aceptable; sin cita es una afirmación sin
respaldo con apariencia de respaldo.

---

## 6 · Convenciones de escritura numérica (español)

| Elemento | Convención | Ejemplo |
|---|---|---|
| Separador decimal | **Coma** en texto en español | `0,95` |
| Separador de miles | Espacio fino, **nunca** punto ni coma | `5 000` |
| Porcentaje | Espacio antes del signo | `95 %` |
| Rango | Guion corto, con unidad una vez | `250–5 000` |
| Intervalo de confianza | Explícito | `IC 95 %: [1,8; 2,2]` |
| Cifras significativas | Las que el error estándar justifica | `2,39 (EE 0,07)`, no `2,3846974` |
| Variables y parámetros | Cursiva | *n*, *τ*, *X*₃ |
| Unidades y operadores | Redonda | 5 kg, log, E[·] |

**Decida la convención decimal una vez y aplíquela al 100 % del documento**,
incluidas tablas, figuras y leyendas. La incoherencia más común: texto en coma
decimal y tablas generadas por código en punto decimal. Si las tablas vienen de
Python o R, convierta en el script que las genera, no a mano.

> Nota técnica: `babel` con `spanish` fija la coma decimal en modo matemático. Si
> su plantilla usa `siunitx` con `output-decimal-marker = {.}`, está
> contradiciendo a `babel`. Elija uno.

---

## 7 · Listas de verificación

**Antes de entregar un borrador al asesor**

- [ ] Cada afirmación causal tiene su supuesto en la misma sección.
- [ ] Ninguna cifra del texto se escribió a mano.
- [ ] El resumen tiene números.
- [ ] Las conclusiones están emparejadas con los objetivos.
- [ ] Ningún `TODO`, ningún `lipsum`, ningún «Nombre del Autor».
- [ ] Toda figura y tabla se menciona en el texto por referencia cruzada.
- [ ] El documento compila sin advertencias desde un clon limpio.

**Antes de la sustentación**

- [ ] Puede defender cada supuesto de identificación durante cinco minutos.
- [ ] Sabe qué resultado habría refutado su hipótesis.
- [ ] Conoce la debilidad más seria de su trabajo mejor que el jurado.
- [ ] Puede reproducir cualquier cifra en vivo desde el repositorio.
