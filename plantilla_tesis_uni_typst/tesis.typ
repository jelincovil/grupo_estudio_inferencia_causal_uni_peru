// =====================================================================
// tesis.typ — Documento principal
// Plantilla de tesis (Maestría/Doctorado) · UNI · versión Typst
// ---------------------------------------------------------------------
// Edite el bloque «metadatos» y reemplace el contenido de cada
// capítulo. Compile con:  typst compile tesis.typ
// =====================================================================
#import "plantilla.typ": *

// ---------------------------------------------------------------------
// METADATOS DEL DOCUMENTO (edite aquí)
// ---------------------------------------------------------------------
#let meta = (
  titulo: "Estimación de efectos causales en datos observacionales mediante doble aprendizaje automático",
  autor: "Nombre del Autor",
  asesor: "Dr. Nombre del Asesor",
  grado: "Doctor en Ciencias con Mención en Estadística",
  tipo-doc: "Tesis doctoral",
  universidad: "Universidad Nacional de Ingeniería",
  facultad: "Facultad de Ingeniería Económica, Estadística y Ciencias Sociales",
  unidad: "Unidad de Posgrado",
  ciudad: "Lima — Perú",
  anio: 2026,
  logo: "logo_uni.png",
  palabras-clave: ("inferencia causal", "doble aprendizaje automático",
                   "propensión al tratamiento", "datos observacionales",
                   "validación cruzada"),
  resumen-corto: "Marco de estimación de efectos causales con aprendices flexibles, parcialado y validación cruzada, evaluado por simulación Monte Carlo.",
  repositorio: "https://github.com/jelincovil/grupo_estudio_inferencia_causal_uni_peru",
)

// ---------------------------------------------------------------------
// ACTIVACIÓN DE LA PLANTILLA
// ---------------------------------------------------------------------
#show: tesis.with(meta)

// =====================================================================
// PORTADA
// =====================================================================
#portada(meta)

// =====================================================================
// PRELIMINARES (numeración romana)
// =====================================================================
#show: frontmatter

#prefacio([Dedicatoria])[
  #align(right)[
    #block(width: 72%)[
      #text(style: "italic", size: 12.5pt)[
        A quienes preguntan «¿y por qué?» antes que «¿para qué?»: a mis
        maestros, que convirtieron la curiosidad en método, y a mi familia,
        que sostuvo cada página de este camino.
      ]
    ]
  ]
]

#prefacio([Agradecimientos])[
  A mi asesor, por la exigencia rigurosa y la generosidad intelectual con que
  orientó cada etapa de esta investigación; a los integrantes del Grupo de
  Estudio de Inferencia Causal de la Universidad Nacional de Ingeniería, cuyo
  seminario semanal dio origen a las preguntas que aquí se desarrollan; y a
  la Unidad de Posgrado de la Facultad de Ingeniería Económica, Estadística y
  Ciencias Sociales, por el espacio académico y el apoyo institucional
  recibidos.

  Agradezco igualmente a los revisores anónimos, cuyas observaciones
  mejoraron sustancialmente la presentación de los resultados, y a la
  comunidad de software libre, en cuyas herramientas —Python, NumPy y
  scikit-learn— se apoya íntegramente la parte computacional de este
  trabajo.
]

#prefacio([Resumen])[
  Esta tesis desarrolla y evalúa un marco para la estimación de efectos
  causales a partir de datos observacionales cuando las relaciones entre las
  covariables, el tratamiento y el resultado son desconocidas y
  potencialmente no lineales. El marco combina el principio de parcialado
  (*partialling-out*) con aprendices flexibles de potenciación de gradiente
  por histogramas y validación cruzada, en el espíritu del doble aprendizaje
  automático (DML), y se contrasta sistemáticamente con la diferencia de
  medias, la ponderación por probabilidad inversa (IPW) y el ajuste lineal
  por mínimos cuadrados ordinarios.

  Los experimentos de simulación, calibrados con un confusor no lineal, una
  covariable de colas pesadas y un tamaño de efecto conocido
  ($tau = 2$), muestran que el ajuste lineal incurre en un sesgo persistente
  cercano a $+0.95$ aun con $n = 5000$ observaciones, mientras que el
  estimador DML reduce su sesgo de $+0.258$ (con $n = 250$) a $+0.003$
  (con $n = 5000$) y alcanza la menor raíz del error cuadrático medio en
  todos los escenarios. La cobertura de los intervalos construidos con la
  función de influencia se aproxima al nivel nominal del 95 % a medida que
  crece el tamaño de muestra, en tanto que la cobertura del ajuste lineal
  colapsa a 0 % porque el error estándar decrece más rápido que el sesgo. La
  distribución bootstrap del estimador de ajuste lineal, estrecha pero
  centrada lejos del valor verdadero, ilustra que la precisión no compensa
  la ausencia de validez causal.

  #v(0.8em)
  #line(length: 30%, stroke: 0.5pt + regla)
  #v(0.5em)
  #text(size: 11pt)[
    #strong[Palabras clave:] inferencia causal, doble aprendizaje
    automático, propensión al tratamiento, datos observacionales, validación
    cruzada.
  ]
]

#prefacio([Abstract])[
  This thesis develops and evaluates a framework for estimating causal
  effects from observational data when the relationships among covariates,
  treatment, and outcome are unknown and potentially nonlinear. The framework
  combines the partialling-out principle with flexible histogram
  gradient-boosting learners and cross-fitting, in the spirit of double
  machine learning (DML), and is systematically compared against the
  difference-in-means estimator, inverse probability weighting (IPW), and
  ordinary least squares adjustment.

  Monte Carlo experiments calibrated with a nonlinear confounder, a
  heavy-tailed covariate, and a known effect size ($tau = 2$) show that
  linear adjustment incurs a persistent bias of about $+0.95$ even with
  $n = 5000$ observations, whereas the DML estimator reduces its bias from
  $+0.258$ (at $n = 250$) to $+0.003$ (at $n = 5000$) and attains the
  lowest root mean squared error in every scenario. Confidence intervals
  built from the influence function approach the nominal 95 % level as the
  sample size grows, while the coverage of linear adjustment collapses to
  0 % because its standard error shrinks faster than its bias. The bootstrap
  distribution of the linear-adjustment estimator—narrow yet centered away
  from the true value—illustrates that precision is no substitute for causal
  validity.

  #v(0.8em)
  #line(length: 30%, stroke: 0.5pt + regla)
  #v(0.5em)
  #text(size: 11pt)[
    #strong[Keywords:] causal inference, double machine learning, propensity
    score, observational data, cross-fitting.
  ]
]

#prefacio([Copia de Documentos])[
  En esta sección se incorporan los documentos exigidos por la Unidad de
  Posgrado: cartas, actas, formatos y constancias. Sustituya el presente
  texto por los documentos correspondientes.

  #aviso([Instrucciones])[
    Para insertar un documento digitalizado, colóquelo en la carpeta
    #path("documentos/") e inclýyalo con
    #raw("#figure(image(\"documentos/acta.pdf\", width: 92%),
caption: [Acta de aprobación del proyecto])", block: false).
    La plantilla se encarga del estilo del pie de figura.
  ]
]

#outline(title: [Índice general], depth: 2)
#outline(target: figure.where(kind: image), title: [Índice de figuras])
#outline(target: figure.where(kind: table), title: [Índice de tablas])

// =====================================================================
// CUERPO DE LA TESIS (numeración arábiga)
// =====================================================================
#show: mainmatter

= Introducción

== Motivación y contexto

Las decisiones públicas más importantes —qué programa social ampliar, qué
tratamiento priorizar, qué política descontinuar— dependen de responder una
pregunta contrafactual: ¿qué habría ocurrido si el individuo no hubiera
recibido la intervención? Cuando la aleatorización es inviable por razones
éticas o presupuestales, la respuesta debe construirse a partir de datos
observacionales, en los que la asignación del tratamiento depende de las
características mismas de las unidades estudiadas. Esa dependencia es la
fuente del sesgo de confusión, y controlarla exige suposiciones sobre la
forma funcional con que las covariables entran en el proceso de asignación
y en el resultado.

En paralelo, las últimas décadas han consolidado aprendices estadísticos de
alta flexibilidad —bosques aleatorios @breiman2001random, redes neuronales
@lecun1998gradient y métodos de regularización @tibshirani1996regression
@zou2005regularization— capaces de aproximar funciones de regresión
arbitrariamente irregulares. La flexibilidad, sin embargo, no es gratuita:
los errores de estimación de esas funciones «nuisance» se trasmiten al
estimador del efecto causal y contaminan su distribución límite. La pregunta
central de esta tesis es cómo conciliar ambas demandas: la flexibilidad
necesaria para ajustar confusión no lineal y la disciplina inferencial que
exige un intervalo de confianza honesto.

== Planteamiento del problema

El problema puede formularse con precisión. Sea $A in {0,1}$ un tratamiento
binario, $Y$ una respuesta continua y $X$ un vector de covariables
observadas antes del tratamiento. Bajo el supuesto de no confusión no
observada, el efecto promedio del tratamiento
$ tau = E[Y(1)] - E[Y(0)] $
es identificable a partir de la distribución observable de $(X, A, Y)$. Los
estimadores clásicos lo explotan bajo restricciones que rara vez se
satisfacen en la práctica: el ajuste por regresión supone formas lineales
para las funciones de resultado, y la ponderación por probabilidad inversa
supone un modelo de propensión correctamente especificado. Cuando el
confusor actúa de manera no lineal, ambos enfoques fallan de manera
sistemática y silenciosa: el error estándar disminuye con la muestra,
pero el sesgo permanece, de modo que la cobertura de los intervalos de
confianza se deteriora precisamente cuando el investigador tiene más
motivos para confiar en ellos.

De ese diagnóstico surgen las preguntas de investigación que guían el
trabajo:

- ¿Bajo qué condiciones sobre los aprendices de las funciones nuisance un
  estimador de $tau$ construido por parcialado preserva la validez
  asintótica?
- ¿Cuál es el costo, en sesgo y cobertura, de ajustar la confusión no
  lineal con modelos lineales mal especificados?
- ¿Cómo se comparan, en escenarios calibrados con confusores no lineales y
  colas pesadas, la diferencia de medias, la ponderación por probabilidad
  inversa, el ajuste lineal y el doble aprendizaje automático?

== Objetivos

=== Objetivo general

Desarrollar y evaluar, mediante simulación Monte Carlo con un generador de
datos calibrado, un marco de estimación de efectos causales en datos
observacionales que combine el parcialado con aprendices flexibles y
validación cruzada, y contrastar su sesgo, su error cuadrático medio y la
cobertura de sus intervalos con los de los estimadores clásicos.

=== Objetivos específicos

+ Formalizar las condiciones de ortogonalidad y velocidad de convergencia
  que garantizan la validez asintótica del estimador por parcialado cuando
  las funciones nuisance se estiman con aprendices flexibles.
+ Diseñar e implementar un protocolo de simulación con confusor no lineal,
  covariable de colas pesadas y tamaño de efecto conocido, reproducible con
  semillas fijas.
+ Cuantificar el sesgo, la raíz del error cuadrático medio y la cobertura
  empírica de los cuatro estimadores en tamaños de muestra
  $n in {250, 1000, 5000}$, y documentar el papel del bootstrap como
  diagnóstico de precisión frente a la validez.

== Hipótesis y variables

=== Hipótesis

- #strong[Hipótesis general:] bajo no confusión no observada y aprendices
  cuya velocidad de convergencia supere $n^(-1/4)$, el estimador de doble
  aprendizaje automático por parcialado es insensible en primer orden a los
  errores de estimación de las funciones nuisance, por lo que su sesgo es
  despreciable y su cobertura se aproxima al nivel nominal, aun cuando la
  confusión sea no lineal.
- #strong[Hipótesis específicas:] (i) el ajuste lineal omiso del término no
  lineal del confusor conserva un sesgo que no decrece con $n$; (ii) la
  ponderación por probabilidad inversa es consistente pero vulnerable a la
  varianza de los pesos y al recorte de propensiones extremas; y (iii) la
  distribución bootstrap de un estimador sesgado converge a su valor
  sesgado, por lo que no constituye evidencia de validez causal.

=== Variables y definiciones operacionales

- #strong[Variable dependiente (respuesta):] $Y$, resultado continuo
  generado como $Y = tau A + m(X) + epsilon$ con $epsilon tilde N(0, 1.5^2)$.
- #strong[Variable de tratamiento:] $A in {0,1}$, con propensidad
  $e(X) = P(A = 1 mid X)$ no lineal en $X_3$.
- #strong[Covariables:] $X = (X_1, ..., X_5)$; $X_1$, $X_2$ y $X_3$ son
  confusores (afectan a $A$ y a $Y$), mientras que $X_4$ y $X_5$ son ruido
  puro; $X_2$ sigue una distribución $t$ de Student con 3 grados de
  libertad estandarizada.
- #strong[Parámetro de interés:] $tau = 2$, el efecto promedio del
  tratamiento en la población.

== Justificación

#strong[Justificación teórica.] El trabajo articula dos literaturas que
suelen transitar separadas: la teoría de la estimación semiparamétrica, que
establece condiciones de ortogonalidad tipo Neyman, y la práctica del
aprendizaje estadístico moderno @hastie2015statistical. Explicitar esa
articulación en un solo marco verificable contribuye a la formación de
estadísticos capaces de usar herramientas flexibles sin renunciar a la
inferencia.

#strong[Justificación metodológica.] El protocolo de simulación —con
generador calibrado, semillas fijas y métricas estandarizadas de sesgo,
RMSE y cobertura— es íntegramente reproducible y puede reutilizarse como
banco de pruebas para nuevos estimadores causales, en la tradición del
remuestreo introducido por @efron1979bootstrap.

#strong[Justificación práctica.] En el Perú, la evaluación de programas
sociales y de políticas públicas descansa crecientemente en registros
administrativos de gran volumen, cuya estructura de confusión es compleja y
desconocida. Disponer de estimadores robustos a la mala especificación
funcional, y de protocolos honestos para reportar su incertidumbre, tiene
un impacto directo en la calidad de la evidencia usada para decidir.

== Estructura de la tesis

El documento se organiza en seis capítulos y cinco apéndices. El capítulo 2
consolida los fundamentos matemáticos y el estado del arte. El capítulo 3
detalla el diseño metodológico, el generador de datos y los estimadores
comparados, incluido el algoritmo de parcialado con validación cruzada. El
capítulo 4 presenta los resultados descriptivos e inferenciales. El
capítulo 5 discute los hallazgos y los pone en perspectiva con la
literatura. El capítulo 6 concluye y recomienda. Los apéndices contienen la
matriz de consistencia, la matriz de operacionalización, las
demostraciones técnicas, el código fuente y los datos suplementarios del
experimento.

= Marco teórico

== Antecedentes

La inferencia a partir de datos remuestreados tiene un hito fundacional en
el trabajo de @efron1979bootstrap, quien formalizó el bootstrap como
procedimiento general para aproximar la distribución de un estadístico sin
apoyarse en derivaciones analíticas. Medio siglo después, el bootstrap
sigue siendo el instrumento de diagnóstico por excelencia de la variabilidad
estimada, y esta tesis lo emplea precisamente para mostrar sus límites: su
capacidad de revelar la precisión de un estimador, y su impotencia para
detectar el sesgo que hereda de una mala especificación.

En el frente del aprendizaje estadístico, @tibshirani1996regression
introdujo el lasso y @zou2005regularization la red elástica, demostrando
que la penalización puede convertir la estimación en altas dimensiones en
un problema bien planteado; @breiman2001random estableció que el
agregamiento de árboles débilmente correlacionados reduce drásticamente la
varianza sin sacrificar la adaptabilidad. Sobre esas bases se construyeron
los aprendices flexibles que hoy alimentan los métodos de doble aprendizaje
automático. Para los fundamentos de probabilidad e inferencia clásica, el
trabajo se apoya en los tratados de @casella2002statistical y
@wasserman2006all.

== Fundamentos matemáticos

La formalización requiere de tres piezas: el espacio probabilístico sobre
el cual se definen los datos, el lenguaje de los resultados potenciales que
da contenido al parámetro causal, y la teoría asintótica que valida los
intervalos de confianza.

#definicion(nombre: [Espacio de probabilidad], id: "def:prob")[
  Un espacio de probabilidad es una terna $(Omega, cal(F), P)$ donde
  $Omega$ es el espacio muestral, $cal(F)$ es una $sigma$-álgebra sobre
  $Omega$ y $P: cal(F) -> [0,1]$ es una medida de probabilidad.
]

#definicion(nombre: [Resultados potenciales y efecto causal], id: "def:po")[
  Para cada unidad se definen los resultados potenciales $(Y(0), Y(1))$, el
  valor de la respuesta bajo cada nivel de tratamiento. El efecto promedio
  del tratamiento es la diferencia de expectativas
  $ tau = E[Y(1)] - E[Y(0)], $
  y el efecto observado corresponde a $Y = Y(A)$.
]

La identificación de $tau$ a partir de datos observacionales exige
supuestos sobre el mecanismo de asignación.

#definicion(nombre: [Propensidad al tratamiento], id: "def:prop")[
  La propensidad es la probabilidad condicional de recibir el tratamiento,
  $ e(x) = P(A = 1 mid X = x), quad x in RR^d. $
]

Bajo no confusión no observada $(Y(0), Y(1)) perp A mid X$
y positividad $0 < e(x) < 1$, el parámetro $tau$ es identificable, pues
$E[Y mid A = a, X = x]$ recupera $E[Y(a) mid X = x]$. La
estructura de la confusión se representa de manera compacta con grafos
dirigidos acíclicos, como ilustra la @fig:dag: el confusor $C$ abre un
camino espurio entre el tratamiento $A$ y el resultado $Y$ que el
estimador ingenuo atribuye al efecto causal $tau$.

#figure(
  image("figuras/cap2_dag_confusion.png", width: 84%),
  caption: [Grafo dirigido acíclico de la confusión: el confusor $C$ abre
  un camino espurio entre el tratamiento $A$ y el resultado $Y$.],
) <fig:dag>

La validez de los intervalos de confianza descansa, finalmente, en la
teoría asintótica.

#teorema(nombre: [Teorema central del límite], id: "thm:clt")[
  Sea $(X_n)_(n >= 1)$ una sucesión de variables aleatorias independientes
  e idénticamente distribuidas con $E[X_1] = mu$ y $Var(X_1) = sigma^2 <
  infinity$. Entonces
  $ (sqrt(n) (overline(X)_n - mu)) / sigma arrow.r^d N(0, 1), quad
    overline(X)_n = 1/n sum_(i=1)^n X_i, $
  donde $arrow.r^d$ denota convergencia en distribución.
]

#demostracion[
  La demostración clásica procede por funciones características: la
  función característica de $sqrt(n)(overline(X)_n - mu)/sigma$ admite el
  desarrollo de Taylor $1 - t^2/(2n) + o(n^(-1))$, cuyo límite es
  $e^(-t^2/2)$, la función característica de la distribución normal
  estándar; el teorema de continuidad de Lévy concluye. El detalle completo
  se desarrolla en el apéndice C.
]

El estimador central de esta tesis se apoya en una propiedad de
ortogonalidad que merece enunciarse con precisión.

#proposicion(nombre: [Ortogonalidad del residual de Neyman], id: "prop:neyman")[
  Sean $hat(m)(X)$ y $hat(e)(X)$ estimaciones de la función de resultado no
  tratado $m(X) = E[Y mid A = 0, X]$ y de la propensidad $e(X)$. Si
  ambas convergen en error cuadrático a velocidad $o_p (n^(-1/4))$ y se
  construyen por validación cruzada, el estimador por parcialado
  $ hat(tau) = frac(sum_(i=1)^n hat(A)_i hat(Y)_i, sum_(i=1)^n hat(A)_i^2),
    quad hat(A)_i = A_i - hat(e)(X_i), quad hat(Y)_i = Y_i - hat(m)(X_i) $
  es insensible en primer orden a los errores de estimación de las
  funciones nuisance y satisface $sqrt(n) (hat(tau) - tau) arrow.r^d N(0, V)$
  con $V$ determinada por la función de influencia.
]

#demostracion[
  La clave es que el producto interno muestral de los residuales de
  $A$ con los residuales de $Y$ cancela, en expectativa, los términos de
  primer orden que involucran $hat(m) - m$ y $hat(e) - e$: la derivada
  direccional del funcional en la dirección de los errores nuisance es
  nula por la ortogonalidad de las proyecciones. La validación cruzada
  elimina el sesgo de sobreajuste que aparecería si los mismos datos
  sirvieran para ajustar y para evaluar. ∎ (esbozo; el desarrollo completo
  sigue el argumento estándar del doble aprendizaje automático.)
]

Como se establece en la @def:prob, el marco de trabajo es un espacio de
probabilidad; la @def:prop define la propensidad que estimarán los métodos
de ponderación; y la @prop:neyman garantiza que el @alg:dml del capítulo 3
herede la normalidad asintótica del @thm:clt.

== Definición de términos

#terms(
  terms.item([Modelo estadístico:], [una familia de distribuciones
  $cal(P)_theta$, $theta in Theta$, postulada para el mecanismo generador
  de los datos; la flexibilidad de la familia determina el riesgo de sesgo
  por mala especificación.]),
  terms.item([Estimación:], [el procedimiento que asigna a $theta$ un valor
  $hat(theta)$ a partir de la muestra; sus propiedades relevantes aquí son
  el sesgo, la consistencia y la eficiencia asintótica.]),
  terms.item([Inferencia causal:], [el conjunto de técnicas que identifican y
  estiman efectos contrafactuales —del tipo $E[Y(1)] - E[Y(0)]$— a partir
  de datos observables, bajo supuestos de no confusión y positividad.]),
  terms.item([Doble aprendizaje automático:], [estrategia de estimación que emplea
  aprendices flexibles para dos funciones nuisance —el resultado esperado y
  la propensidad— y combina sus residuales de modo que los errores de
  ambos se cancelen en primer orden.]),
  terms.item([Validación cruzada:], [partición de la muestra en bloques que
  alternan el ajuste y la predicción, usada aquí para evitar el sesgo de
  sobreajuste en las funciones nuisance.]),
  terms.item([Cobertura empírica:], [proporción de réplicas de un experimento de
  simulación en las que el intervalo de confianza construido contiene al
  parámetro verdadero; su distancia al nivel nominal resume la honestidad
  del procedimiento inferencial.]),
)

== Estado del arte

La confluencia entre regularización e inferencia cristalizó en la pasada
década. @hastie2015statistical sistematizaron la estadística con esparsidad
—el lasso y sus generalizaciones— y @friedman2010regularization aportaron
el algoritmo de descenso coordinado que hizo práctico el ajuste de rutas de
regularización completas. La trama de la estimación causal flexible se
completa con la arquitectura de aprendizaje por gradiente de
@lecun1998gradient, hoy materializada en los aprendices de potenciación de
gradiente por histogramas que utiliza esta tesis, y con la perspectiva
bayesiana jerárquica de @gelman2006prior, que recuerda que toda
especificación —frecuentista o bayesiana— incorpora supuestos previos cuya
sensibilidad debe examinarse.

El estado del arte puede resumirse en una lección: ningún aprendiz, por
flexible que sea, resuelve por sí solo el problema causal. Lo que resuelve
el problema es la combinación de tres ingredientes —un funcional
ortogonal, aprendices suficientemente rápidos y validación cruzada—, y es
exactamente esa combinación la que el marco de esta tesis implementa y
evalúa.

= Metodología

== Diseño de la investigación

El estudio es metodológico y de alcance explicativo, basado en simulación
Monte Carlo con generador de datos calibrado. El diseño responde
directamente a las preguntas de investigación: cada condición experimental
—el tamaño de muestra $n$— se cruza con cuatro estimadores evaluados con
idénticas réplicas, de modo que las diferencias observadas son atribuibles
exclusivamente al estimador y no al azar del muestreo.

== Población, muestra y fuente de datos

La «población» de referencia es la distribución generadora
$P_0$ de la terna $(X, A, Y)$, definida por las ecuaciones
$ X tilde N(0, I_5), quad X_2 tilde t_3/sqrt(3), $
$ e(X) = "expit"(1.1 X_1 - 0.7 X_2 + 0.9 (X_3^2 - 1)), quad A tilde
  "Bernoulli"(e(X)), $
$ Y = tau A + m(X) + epsilon, quad m(X) = 1.6 X_1 - 1.1 X_2 + 0.85
  (X_3^2 - 1) + 0.6 X_1 X_2, $
con $tau = 2$, $epsilon tilde N(0, 1.5^2)$ e independencia entre
$epsilon$ y $(X, A)$. La no linealidad en $X_3^2$ es deliberada: imita la
situación, frecuente en registros administrativos, en que el confusor
opera de manera acelerada sobre la asignación y el resultado. Las colas
pesadas de $X_2$ incorporan el realismo de variables económicas con
valores extremos.

Los escenarios experimentales son $n in {250, 1000, 5000}$, con $B = 300$,
$150$ y $60$ réplicas respectivamente. Toda la aleatorización deriva de
semillas fijas reproducibles: la réplica $i$ usa la semilla
$2026 + 17 i$.

== Procedimiento y plan de análisis

El protocolo consta de cinco pasos: (i) calibración del generador y
verificación de la propensidad verdadera; (ii) generación de las réplicas
por escenario; (iii) estimación de $tau$ con los cuatro métodos y de su
error estándar; (iv) construcción de intervalos al 95 % con la
aproximación normal; y (v) agregación de sesgo, raíz del error cuadrático
medio, error estándar medio y cobertura empírica. Adicionalmente, sobre un
conjunto con $n = 1000$ se ejecuta un bootstrap no paramétrico con
$B = 2000$ remuestreos del estimador de ajuste lineal, como diagnóstico de
la relación entre precisión y sesgo.

== Técnicas estadísticas

Los cuatro estimadores comparados se formalizan a continuación. La
diferencia de medias, $ hat(tau)_"DM" = overline(Y)_1 - overline(Y)_0 $,
ignora la confusión y sirve de línea base. La ponderación por probabilidad
inversa con normalización de Hájek usa la propensidad estimada $hat(e)$:
$ hat(tau)_"IPW" = frac(sum_(i=1)^n A_i hat(e)_i^(-1) Y_i, sum_(i=1)^n A_i hat(e)_i^(-1)) - frac(sum_(i=1)^n (1 - A_i)(1 - hat(e)_i)^(-1) Y_i, sum_(i=1)^n (1 - A_i)(1 - hat(e)_i)^(-1)), $
donde $hat(e)$ se ajusta por regresión logística con base
$(1, X_1, X_2, X_3, X_3^2)$ y recorte al intervalo $[0.02, 0.98]$. El
ajuste lineal por mínimos cuadrados ordinarios regresa $Y$ sobre $(1, A,
X)$ y toma el coeficiente de $A$. Finalmente, el estimador de doble
aprendizaje automático por parcialado se calcula como en la
@prop:neyman, con residuales construidos por aprendices de potenciación de
gradiente por histogramas bajo validación cruzada de $K = 5$ particiones;
su error estándar deriva de la función de influencia empírica.

#algoritmo([DML por parcialado con validación cruzada], id: "alg:dml")[
  #set text(size: 10.5pt)
  #strong[Entrada:] datos $(X_i, A_i, Y_i)_(i=1)^n$, particiones $K = 5$,
  aprendices $hat(m)$ y $hat(e)$. \
  #strong[1.] Dividir ${1, ..., n}$ en bloques disjuntos $I_1, ..., I_K$,
  aleatorizados con semilla fija. \
  #strong[2.] #strong[para] $k = 1, ..., K$: ajustar $hat(m)^((-k))$ con
  las observaciones $(X_i, Y_i)$ tales que $i in.not I_k$, y $hat(e)^((-k))$
  con las $(X_i, A_i)$; predecir en $I_k$ y formar los
  residuales cruzados $hat(Y)_i$ y $hat(A)_i$. \
  #strong[3.] Devolver $hat(tau) = frac(sum_i hat(A)_i hat(Y)_i, sum_i hat(A)_i^2)$ y el error estándar $hat(s) = hat(sigma)_psi/sqrt(n)$,
  con $hat(psi)_i = hat(A)_i (hat(Y)_i - hat(tau) hat(A)_i)/overline(hat(A)^2)$. \
  #strong[4.] Reportar el intervalo $hat(tau) plus.minus 1.96 hat(s)$.
]

== Implementación computacional

La implementación usa Python 3.12 con NumPy y scikit-learn. El código
completo —incluido el generador, los cuatro estimadores y el protocolo de
réplicas— está disponible en el repositorio del grupo de estudio; el
apéndice D reproduce las piezas esenciales. El @cod:dml muestra la función
central: los dos aprendices se ajustan por particiones y los residuales
cruzados producen el estimador y su error estándar.

#figure(
  ```python
  import numpy as np
  from sklearn.ensemble import HistGradientBoostingRegressor
  from sklearn.model_selection import KFold

  def dml_parcialado(X, A, Y, semilla=7):
      """Estimador DML por parcialado con cross-fitting (K = 5)."""
      n = len(A)
      Yr, Ar = np.empty(n), np.empty(n)
      folds = KFold(5, shuffle=True, random_state=semilla)
      for tr, te in folds.split(X):
          fy = HistGradientBoostingRegressor(
              max_iter=120, learning_rate=0.08,
              max_leaf_nodes=11, random_state=11)
          fa = HistGradientBoostingRegressor(
              max_iter=120, learning_rate=0.08,
              max_leaf_nodes=11, random_state=13)
          fy.fit(X[tr], Y[tr])
          fa.fit(X[tr], A[tr])
          Yr[te] = Y[te] - fy.predict(X[te])
          Ar[te] = A[te] - fa.predict(X[te])
      den = (Ar ** 2).mean()
      tau_hat = (Ar * Yr).mean() / den
      psi = Ar * (Yr - tau_hat * Ar) / den
      se = np.std(psi, ddof=1) / np.sqrt(n)
      return tau_hat, se
  ```,
  caption: [Estimador DML por parcialado con validación cruzada.],
) <cod:dml>

#align(center)[
  #v(0.9em)
  #image("figuras/qr_repositorio.svg", width: 24%)
  #v(0.5em)
  #text(size: 10pt, fill: gris)[Código y datos del experimento:
  #link(meta.repositorio)]
]

= Resultados

== Análisis descriptivo

La @tab:descriptivos resume el conjunto de referencia con $n = 1000$, extraído del
mismo generador que alimenta el experimento. La propensidad marginal sitúa
al 47 % de las unidades bajo tratamiento, un balance razonable para el
estudio de los estimadores ponderados. La respuesta $Y$ presenta una
desviación estándar de 3.37, coherente con la varianza del término de error
($1.5^2 = 2.25$) más la variabilidad de $m(X)$ y del efecto; el rango
$[-15.9, +14.9]$ delata el efecto combinado de la no linealidad del
confusor y de las colas pesadas de $X_2$, cuyo máximo (7.15) triplica con
creces el de las covariables gaussianas.

#figure(
  table(
    columns: (2.1fr, 1fr, 1fr, 1fr, 1fr, 1fr),
    align: (left, center, center, center, center, center),
    table.hline(stroke: 1pt),
    table.header(
      [Variable], [*Media*], [*Desv. est.*], [*Mínimo*], [*Mediana*],
      [*Máximo*],
    ),
    table.hline(stroke: 0.5pt),
    [$X_1$ (confusor)], [$-0.05$], [1.00], [$-3.06$], [$+0.00$], [$+2.63$],
    [$X_2$ (confusor, $t_3$)], [$+0.02$], [0.91], [$-4.53$], [$-0.01$], [$+7.15$],
    [$X_3$ (confusor no lineal)], [$+0.00$], [0.97], [$-2.96$], [$+0.02$], [$+3.27$],
    [$X_4$ (ruido)], [$-0.02$], [1.02], [$-2.92$], [$-0.04$], [$+3.06$],
    [$X_5$ (ruido)], [$-0.04$], [0.97], [$-3.65$], [$-0.04$], [$+3.09$],
    [$A$ (tratamiento)], [0.47], [0.50], [0], [0], [1],
    [$Y$ (respuesta)], [$+0.69$], [3.37], [$-15.88$], [$+0.64$], [$+14.93$],
    table.hline(stroke: 1pt),
  ),
  caption: [Estadísticos descriptivos del conjunto de referencia
  ($n = 1000$).],
) <tab:descriptivos>

== Inferencia estadística

La @tab:simulacion condensa el experimento completo: sesgo, raíz del error
cuadrático medio, error estándar medio y cobertura empírica de los cuatro
estimadores en los tres escenarios. Tres patrones merecen lectura cuidadosa.

#figure(
  table(
    columns: (2.15fr, 1fr, 1fr, 1.25fr, 1.35fr),
    align: (left, center, center, center, center),
    table.hline(stroke: 1pt),
    table.header(
      table.cell(colspan: 5, align: center)[
        #text(size: 11pt)[Experimento Monte Carlo: $tau = 2$; intervalos al 95 %]
      ],
    ),
    [Método], [Sesgo], [RMSE], [EE medio], [Cobertura],
    table.hline(stroke: 0.5pt),
    table.cell(colspan: 5, align: left)[#emph[Panel A: $n = 250$ ($B = 300$)]],
    [Diferencia de medias], [$+2.385$], [2.408], [0.320], [0.0 %],
    [IPW (logístico)], [$+0.240$], [0.533], [0.545], [91.7 %],
    [OLS ajustado], [$+0.935$], [0.982], [0.275], [9.3 %],
    [DML (gradiente)], [$+0.258$], [0.424], [0.266], [79.0 %],
    table.hline(stroke: 0.5pt),
    table.cell(colspan: 5, align: left)[#emph[Panel B: $n = 1000$ ($B = 150$)]],
    [Diferencia de medias], [$+2.397$], [2.402], [0.159], [0.0 %],
    [IPW (logístico)], [$+0.181$], [0.299], [0.301], [88.0 %],
    [OLS ajustado], [$+0.955$], [0.966], [0.137], [0.0 %],
    [DML (gradiente)], [$+0.023$], [0.145], [0.124], [89.3 %],
    table.hline(stroke: 0.5pt),
    table.cell(colspan: 5, align: left)[#emph[Panel C: $n = 5000$ ($B = 60$)]],
    [Diferencia de medias], [$+2.396$], [2.397], [0.072], [0.0 %],
    [IPW (logístico)], [$+0.179$], [0.200], [0.139], [78.3 %],
    [OLS ajustado], [$+0.945$], [0.947], [0.061], [0.0 %],
    [DML (gradiente)], [$+0.003$], [0.062], [0.053], [90.0 %],
    table.hline(stroke: 1pt),
  ),
  caption: [Sesgo, raíz del error cuadrático medio (RMSE), error estándar
  medio (EE) y cobertura empírica al 95 %: el sesgo de la diferencia de
  medias y del ajuste lineal es estable en $n$, no decrece con la muestra.],
) <tab:simulacion>

Primero, el sesgo es un atributo del diseño, no del tamaño de muestra. La
diferencia de medias sobreestima el efecto en $+2.39$ de manera
prácticamente idéntica en los tres escenarios, y el ajuste lineal se
estabiliza alrededor de $+0.95$: omitir el término $X_3^2$ deja un canal de
confusión que ningún volumen de datos cierra. Su cobertura lo confirma con crudeza: 9.3 % con $n = 250$ y 0 % a partir de $n = 1000$, porque el error
estándar se reduce a la par de $n^(-1/2)$ mientras el sesgo permanece.

Segundo, la flexibilidad controlada gana donde el sesgo manda. El estimador
DML arranca con un sesgo moderado ($+0.258$ con $n = 250$), reflejo del
coste finito del aprendizaje de las dos funciones nuisance, y lo disipa al
crecer la muestra: $+0.023$ con $n = 1000$ y $+0.003$ con $n = 5000$. Su
RMSE desciende de 0.424 a 0.062 —una reducción aproximada de siete veces—,
y su cobertura sube de 79.0 % a 90.0 %, acercándose al nominal con la
pendiente esperada. La @fig:cobertura vuelve visible la asimetría del
fenómeno: la cobertura del ajuste lineal y de la diferencia de medias se
desploman temprano, la del IPW se erosiona con el tamaño de muestra y solo
la del DML conserva la trayectoria correcta.

#figure(
  image("figuras/cap4_cobertura.png", width: 92%),
  caption: [Cobertura empírica de los intervalos al 95 % según el tamaño de
  muestra; la línea discontinua marca el nivel nominal.],
) <fig:cobertura>

Tercero, la ponderación ilustra el precio de la varianza. El IPW, con
propensidad correctamente especificada, es consistente; pero sus pesos
amplifican las unidades con propensidad extrema, su error estándar casi
duplica al del DML en todos los escenarios, y el recorte defensivo al
$[0.02, 0.98]$ deja un sesgo residual de $+0.18$ que, al encogerse el error
estándar, degrada la cobertura de 91.7 % a 78.3 %. La precisión se compra
aquí con estabilidad.

El bootstrap aporta el contraste final. Sobre el conjunto con $n = 1000$,
la distribución bootstrap del estimador de ajuste lineal (@fig:bootstrap)
es admirablemente estrecha —desviación estándar de 0.138— y perfectamente
sesgada: su media, 2.840, y su intervalo percentil
$[2.565, 3.107]$ excluyen al valor verdadero con total comodidad. El
remuestreo estima la variabilidad del estimador, no su corrección: dos
investigadores con datos idénticos y modelos mal especificados coincidirán
con precisión en la respuesta equivocada.

#figure(
  image("figuras/cap4_bootstrap.png", width: 92%),
  caption: [Distribución bootstrap ($B = 2000$) del estimador de ajuste
  lineal con $n = 1000$: el intervalo percentil al 95 % excluye a
  $tau = 2$.],
) <fig:bootstrap>

== Validación del modelo

La validación verificó tres frentes. En el frente numérico, las semillas
fijas garantizan reproducibilidad exacta y la convergencia del RMSE del DML
es compatible con la velocidad $n^(-1/2)$ una vez descontado el sesgo
inicial. En el frente de sensibilidad, duplicar el recorte de propensiones
al $[0.05, 0.95]$ incrementa el sesgo del IPW y reduce su varianza, lo que
confirma que el recorte es un compromiso y no una solución. En el frente
inferencial, la sub-cobertura residual del DML (90.0 % con $n = 5000$) es
compatible con la aproximación normal de la función de influencia y con la
penalización finita de los aprendices; intervalos de tipo $t$ bootstrap
reducen la brecha al costo de duplicar el cómputo.

= Discusión

Los resultados articulan una lección central: en datos observacionales con
confusión no lineal, la validez inferencial no la decide la cantidad de
datos sino la arquitectura del estimador. El ajuste lineal falló de la
manera más peligrosa posible —con intervalos estrechos y centrados lejos
de la verdad— porque el error de especificación del confusor no se
promedia: se hereda. La diferencia de medias, que en el experimento
sobreestimó el efecto en más de una unidad, recuerda que la asociación
marginal solo es estimación causal bajo un supuesto de intercambiabilidad
que los datos administrativos rara vez satisfacen.

La comparación entre IPW y DML cualifica el lugar común de que «ponderar
bastaría». Con propensidad correctamente especificada, el IPW fue
consistente; pero su varianza, inflada por los pesos extremos que imponen
las colas de $X_2$, y el sesgo del recorte defensivo lo colocaron en desventaja
sistemática frente al parcialado. La doble robustez no es una sutileza
técnica: es la diferencia entre un estimador que se degrada con gracia y
otro que colapsa cuando el modelo de propensidad falla en las colas.

El experimento bootstrap merece una lectura epistemológica. La
distribución de remuestreo del estimador sesgado era limpia, gaussiana y
reproducible; ningún diagnóstico interno del procedimiento habría señalado
el problema. La validación de supuestos causales no puede, por tanto,
delegarse en la estadística computacional: exige conocimiento del dominio,
diagnósticos de balance y propensidad, y sensibilidad declarada. Esta
conclusión conecta con la perspectiva de @gelman2006prior sobre la
necesidad de examinar la sensibilidad de las suposiciones estructurales, y
delimita honestamente lo que la simulación puede afirmar: los patrones aquí
documentados son propiedades del generador calibrado, y su extrapolación a
datos reales exige replicar el protocolo con la estructura de confusión del
caso concreto.

= Conclusiones y recomendaciones

== Conclusiones

+ El estimador de doble aprendizaje automático por parcialado, con
  aprendices de potenciación de gradiente y validación cruzada de cinco
  particiones, eliminó el sesgo de confusión no lineal al crecer la
  muestra ($+0.258$ con $n = 250$; $+0.003$ con $n = 5000$) y alcanzó el
  menor RMSE en todos los escenarios, en congruencia con la hipótesis
  general.
+ El ajuste lineal omiso del término no lineal del confusor exhibió un
  sesgo estable cercano a $+0.95$ y una cobertura que colapsa de 9.3 % a
  0 %, confirmando la primera hipótesis específica: la mala especificación
  no se diluye con datos.
+ La ponderación por probabilidad inversa fue consistente pero frágil: su
  cobertura descendió de 91.7 % a 78.3 % por la combinación de varianza de
  pesos y recorte de propensiones, en la dirección anticipada por la
  segunda hipótesis específica.
+ La distribución bootstrap del estimador mal especificado convergió a su
  propio sesgo con intervalo $[2.565, 3.107]$ excluyente del valor
  verdadero, verificando la tercera hipótesis: el remuestreo cuantifica la
  precisión, no la validez.

== Recomendaciones

+ Para estimaciones causales con covariables numerosas o relaciones
  desconocidas, adoptar por defecto un estimador ortogonal con validación
  cruzada, reservando el ajuste lineal para contextos con teoría funcional
  firme.
+ Reportar siempre, junto al intervalo de confianza, los diagnósticos de
  propensidad (distribución, extremos y efecto del recorte) y de balance
  de covariables.
+ En evaluaciones con registros administrativos, incorporar de manera
  explícita transformaciones no lineales de los confusores candidatos
  ($X_3^2$ en el generador de este trabajo) al conjunto de ajuste, y
  someter el estimador a un banco de sensibilidad con especificaciones
  alternativas.
+ Preregistrar el protocolo de análisis —estimador, ajustes y métricas—
  antes de acceder a los datos, para que la credibilidad no dependa de la
  coincidencia entre hipótesis y resultado.

== Limitaciones

El alcance es el de un estudio de simulación: la validez externa depende
de que la estructura de confusión del generador —un único confusor no
lineal y una covariable de colas pesadas— represente al caso de aplicación.
El número de réplicas en el escenario grande ($B = 60$) acota la precisión
de las coberturas reportadas (error estándar aproximado de 4 puntos
porcentuales), y la evaluación se circunscribió a efectos promedio, sin
explorar heterogeneidad por subpoblaciones.

== Líneas futuras de investigación

Tres extensiones continúan naturalmente este trabajo: la estimación
eficiente con funcionales doblemente robustos (AIPW) y su comparación con
el parcialado bajo especificaciones mixtas; la extensión a efectos
heterogéneos (CATE) con inferencia válida por bosques causales; y la
aplicación del protocolo completo a un registro administrativo peruano
de programa social, donde la estructura de confusión puede contrastarse
con conocimiento de dominio.

// =====================================================================
// REFERENCIAS BIBLIOGRÁFICAS (estilo APA)
// =====================================================================
#bibliography("referencias.bib", style: "apa", title: [Referencias bibliográficas])

// =====================================================================
// APÉNDICES (numeración A, B, …)
// =====================================================================
#show: apendices

= Matriz de consistencia

La @tab:consistencia alinea los elementos estructurales de la
investigación; en la versión final del sustentante, cada celda debe
corresponder literalmente con el texto de los capítulos 1 y 3.

#figure(
  table(
    columns: (2.6fr, 6fr),
    align: (left, left),
    table.hline(stroke: 1pt),
    table.header([*Elemento*], [*Formulación*]),
    table.hline(stroke: 0.5pt),
    [Problema],
    [¿Cómo estimar el efecto causal $tau$ en datos observacionales cuando
    la confusión es no lineal y las formas funcionales son desconocidas?],
    [Objetivo general],
    [Desarrollar y evaluar por simulación un marco de estimación por
    parcialado con aprendices flexibles y validación cruzada.],
    [Objetivos específicos],
    [(i) Formalizar la ortogonalidad del estimador; (ii) diseñar el
    protocolo de simulación calibrado; (iii) cuantificar sesgo, RMSE y
    cobertura frente a los estimadores clásicos.],
    [Hipótesis],
    [El estimador DML preserva validez asintótica bajo confusión no lineal;
    el ajuste lineal omiso conserva sesgo estable; el bootstrap no detecta
    sesgo de especificación.],
    [Variables],
    [Respuesta $Y$ continua; tratamiento binario $A$; covariables $X_1$–
    $X_5$ (confusores: $X_1$, $X_2$, $X_3$); parámetro $tau = 2$.],
    [Metodología],
    [Simulación Monte Carlo, $n in {250, 1000, 5000}$, réplicas con semilla
    fija, cuatro estimadores, métricas de sesgo, RMSE, EE y cobertura.],
    table.hline(stroke: 1pt),
  ),
  caption: [Matriz de consistencia de la investigación.],
) <tab:consistencia>

= Matriz de operacionalización de variables

La @tab:operacionalizacion detalla variables, dimensiones, indicadores e
instrumentos de cálculo.

#figure(
  table(
    columns: (2.2fr, 2.2fr, 2.6fr, 3fr),
    align: (left, left, left, left),
    table.hline(stroke: 1pt),
    table.header([*Variable*], [*Dimensión*], [*Indicador*],
      [*Instrumento / cálculo*]),
    table.hline(stroke: 0.5pt),
    [Efecto del tratamiento ($tau$)], [Impacto promedio],
    [Diferencia de medias potenciales],
    [Estimadores DM, IPW, OLS y DML sobre el generador calibrado.],
    [Propensidad ($e(X)$)], [Asignación],
    [Probabilidad condicional de $A = 1$],
    [Regresión logística con base $(1, X_1, X_2, X_3, X_3^2)$; recorte
    $[0.02, 0.98]$.],
    [Confusión no lineal], [Estructura del generador],
    [Término $0.85 (X_3^2 - 1)$ en $m(X)$ y $0.9 (X_3^2 - 1)$ en el logit],
    [Diferencia de cobertura entre OLS y DML por escenario.],
    [Colas pesadas], [Distribución de $X_2$],
    [Máximo y curtosis de $X_2$ frente a gaussianas],
    [Estadísticos descriptivos (@tab:descriptivos).],
    [Validez inferencial], [Cobertura],
    [Proporción de intervalos que contienen a $tau$],
    [Intervalos normales al 95 % con error estándar por función de
    influencia.],
    table.hline(stroke: 1pt),
  ),
  caption: [Matriz de operacionalización de variables.],
) <tab:operacionalizacion>

= Demostraciones técnicas

Se desarrolla aquí la demostración esbozada en el @thm:clt. Sea $phi(t) =
E[e^(i t X_1)]$ la función característica de $X_1$, que existe para todo
$t$ real porque $|e^(i t X_1)| = 1$. Para la suma estandarizada
$Z_n = sqrt(n) (overline(X)_n - mu)/sigma$,
$ phi_(Z_n)(t) = (e^(-i t mu/sigma) phi(t/(sigma sqrt(n))))^n
  -> e^(-t^2/2) quad (n -> infinity), $
donde el paso central usa el desarrollo $phi(s) = 1 + i mu s -
sigma^2 s^2/2 + o(s^2)$ válido por la existencia del segundo momento. El
límite es la función característica de la ley normal estándar, y el teorema
de continuidad de Lévy concluye la convergencia en distribución. La
condición de Lindeberg —automáticamente satisfecha bajo idéntica
distribución con varianza finita— extiende el resultado a sumandos no
idénticos y gobierna la calidad de la aproximación en muestras moderadas,
responsable última de la sub-cobertura residual observada en el DML con
$n = 250$.

= Código fuente

El @cod:boot reproduce la rutina de intervalos bootstrap usada en la
validación; el generador completo y el protocolo de réplicas están en el
repositorio del grupo de estudio.

#figure(
  ```python
  import numpy as np

  def bootstrap_ci(data, n_bootstrap=2000, alpha=0.05, semilla=99):
      """Intervalo de confianza percentil bootstrap para la media."""
      rng = np.random.default_rng(semilla)
      n = len(data)
      medias = np.empty(n_bootstrap)
      for b in range(n_bootstrap):
          muestra = rng.choice(np.arange(n), size=n, replace=True)
          medias[b] = data[muestra].mean()
      lo = np.percentile(medias, 100 * alpha / 2)
      hi = np.percentile(medias, 100 * (1 - alpha / 2))
      return lo, hi, medias
  ```
  ,
  caption: [Intervalo de confianza bootstrap percentil. La misma rutina,
  aplicada al estimador de ajuste lineal, produce la distribución de la
  @fig:bootstrap.],
) <cod:boot>

= Datos suplementarios

La @tab:diseno recoge los parámetros del diseño experimental, de modo que
cualquier lector pueda replicar el generador sin consultar el código.

#figure(
  table(
    columns: (3.4fr, 4fr),
    align: (left, left),
    table.hline(stroke: 1pt),
    table.header([*Componente*], [*Especificación*]),
    table.hline(stroke: 0.5pt),
    [Efecto verdadero], [$tau = 2$],
    [Propensidad], [$"expit"(1.1 X_1 - 0.7 X_2 + 0.9 (X_3^2 - 1))$],
    [Función de resultado], [$m(X) = 1.6 X_1 - 1.1 X_2 + 0.85 (X_3^2 - 1)
    + 0.6 X_1 X_2$],
    [Error], [$epsilon tilde N(0, 1.5^2)$, independiente de $(X, A)$],
    [Covariables], [$X_1, X_3, X_4, X_5 tilde N(0,1)$; $X_2 tilde
    t_3/sqrt(3)$],
    [Escenarios], [$(n, B)$: $(250, 300)$, $(1000, 150)$, $(5000, 60)$],
    [Semillas], [Réplica $i$: $2026 + 17 i$; bootstrap: $99$],
    [Aprendices DML], [Potenciación de gradiente por histogramas, 120
    iteraciones, 11 hojas máximas, $K = 5$ particiones],
    [Recorte de propensidad], [$[0.02, 0.98]$],
    table.hline(stroke: 1pt),
  ),
  caption: [Parámetros del diseño experimental del estudio Monte Carlo.],
) <tab:diseno>
