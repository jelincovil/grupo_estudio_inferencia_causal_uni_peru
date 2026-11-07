# Guía de notación y fórmulas

Convenciones matemáticas para los documentos del grupo. La base es la norma
**ISO 80000-2** (*Quantities and units — Part 2: Mathematics*), que es lo que
siguen las editoriales científicas serias, complementada con el uso establecido
en inferencia causal.

---

## 1 · Redonda o cursiva: la regla que ordena todo

| Va en **cursiva** | Va en **redonda** (recta) |
|---|---|
| Variables y parámetros: *x*, *n*, *τ*, *σ* | Operadores y funciones con nombre: sin, log, exp, max, tr |
| Índices que varían: *x<sub>i</sub>* | Índices que son etiquetas: *C*<sub>máx</sub>, *τ*<sub>ATE</sub> |
| Variables aleatorias: *Y*, *A*, *X* | Constantes: e, π, i (unidad imaginaria) |
| Funciones definidas por el autor: *m*(*x*), *e*(*x*) | Diferencial: d*x* |
| | Unidades: 5 kg, 3 s |
| | Operadores de esperanza y varianza: E[·], Var(·), Cov(·) |

El caso que más se equivoca: **el subíndice**. Si es un índice que recorre
valores, va en cursiva (*x<sub>i</sub>*); si es una palabra o abreviatura que
*nombra* algo, va en redonda (*τ̂*<sub>DML</sub>, no *τ̂*<sub>*DML*</sub>).

El segundo caso: `E`, `Var`, `Cov` son operadores, no variables. En LaTeX se
declaran con `\DeclareMathOperator`; en Typst con `math.op`. Las dos plantillas
del repositorio ya los traen definidos — úselos, no escriba `E` suelto.

---

## 2 · Notación causal canónica

Use esta y no otra, para que el documento sea legible por cualquiera formado en
el área:

| Objeto | Notación | Comentario |
|---|---|---|
| Tratamiento | *A* ∈ {0,1} | *A* (del inglés *action*) o *T*; **no** *D* ni *X* |
| Covariables previas al tratamiento | *X* ∈ ℝ<sup>*d*</sup> | «Previas» es parte de la definición, no un detalle |
| Resultado observado | *Y* | |
| Resultados potenciales | *Y*(1), *Y*(0) | Alternativa aceptada: *Y*<sup>*a*=1</sup> |
| Consistencia / SUTVA | *Y* = *Y*(*A*) | Es un supuesto, aunque parezca una definición |
| ATE | *τ* = E[*Y*(1) − *Y*(0)] | |
| ATT | *τ*<sub>ATT</sub> = E[*Y*(1) − *Y*(0) \| *A* = 1] | |
| CATE | *τ*(*x*) = E[*Y*(1) − *Y*(0) \| *X* = *x*] | |
| Propensión | *e*(*x*) = P(*A* = 1 \| *X* = *x*) | |
| No confusión condicional | (*Y*(0), *Y*(1)) ⫫ *A* \| *X* | `\perp\!\!\!\perp` / `perp` |
| Positividad (*overlap*) | 0 < *e*(*x*) < 1 c.s. | «casi seguramente» no es decorativo |
| Intervención de Pearl | E[*Y* \| do(*A* = 1)] | `do` en **redonda** |

**Estimando, estimador, estimación son tres cosas distintas** y la notación debe
distinguirlas, porque confundirlas es confundir el objetivo con el método:

| | Símbolo | Qué es |
|---|---|---|
| Estimando | *τ* | El número poblacional que se quiere conocer. Se define **antes** de ver datos |
| Estimador | *τ̂* | La regla que lo aproxima: una función de la muestra |
| Estimación | *τ̂* = 2,003 | El valor que la regla devolvió en estos datos |

Cuando hay varios estimadores, distíngalos por subíndice en redonda:
*τ̂*<sub>DM</sub>, *τ̂*<sub>IPW</sub>, *τ̂*<sub>OLS</sub>, *τ̂*<sub>DML</sub>.

---

## 3 · Ecuaciones en la página

**Numeración.** Por capítulo —`(2.4)`, `(3.1)`— nunca correlativa global. Numere
solo lo que referencia.

**Puntuación.** La ecuación es un elemento de la oración:

> El estimando de interés es el efecto promedio del tratamiento,
> $$ \tau = \mathrm{E}[Y(1) - Y(0)], $$
> identificable bajo los supuestos de la sección 3.3.

Coma porque la oración sigue. Punto si termina. Nada si la gramática no lo pide.
Una ecuación desplegada sin puntuación ni conexión gramatical con el texto es un
objeto suelto.

**Alineación.** Para desarrollos de varias líneas, alinee por el signo de
relación (`align` / `aligned`), nunca por el margen izquierdo:

```
\begin{align}
  \hat\tau &= \frac{\sum_i \tilde A_i \tilde Y_i}{\sum_i \tilde A_i^2}, \\
  \tilde A_i &= A_i - \hat e(X_i), \qquad
  \tilde Y_i = Y_i - \hat m(X_i).
\end{align}
```

**Fracciones en línea.** Use `a/b` en texto corrido, no `\frac`: `\frac` en línea
produce tipos minúsculos y rompe el interlineado.

**Lo que no se hace:** `$$...$$` en LaTeX (usar `\[...\]` o `equation`);
operadores escritos como variables (`sin` en cursiva); ecuaciones que desbordan
el margen sin partir; definir un símbolo dos páginas después de usarlo.

---

## 4 · Tabla de notación

Todo documento con más de ~15 símbolos lleva una **tabla de notación** en los
preliminares o como apéndice A. Formato: símbolo · definición · dónde se
introduce.

| Símbolo | Definición | §
|---|---|---|
| *A* | Tratamiento binario, *A* ∈ {0,1} | 1.4 |
| *X* | Vector de covariables previas al tratamiento, *X* ∈ ℝ<sup>5</sup> | 1.4 |
| *e*(*x*) | Propensión, P(*A* = 1 \| *X* = *x*) | 2.2 |
| *m*(*x*) | E[*Y* \| *A* = 0, *X* = *x*] | 2.2 |
| *τ* | ATE, E[*Y*(1) − *Y*(0)] | 1.4 |
| *τ̂* | Estimador por parcialado de *τ* | 3.5 |
| *ψ<sub>i</sub>* | Función de influencia empírica | 3.5 |

Un símbolo = un significado en todo el documento. Reutilizar *n* para tamaño de
muestra y para número de iteraciones es un error de redacción, no un atajo.

---

## 5 · Sintaxis comparada

| Objeto | LaTeX | Typst |
|---|---|---|
| Esperanza | `\E[Y]` (con `\DeclareMathOperator{\E}{E}`) | `E[Y]` (con `#let E = math.op("E")`) |
| Varianza | `\Var(X)` | `Var(X)` |
| Resultado potencial | `Y(1)` o `Y^{a=1}` | `Y(1)` o `Y^(a=1)` |
| Independencia condicional | `Y \perp\!\!\!\perp A \mid X` | `Y perp A mid X` |
| Convergencia en distribución | `\xrightarrow{d}` | `arrow.r^d` |
| Convergencia en probabilidad | `\xrightarrow{p}` | `arrow.r^p` |
| Estimador con gorro | `\hat{\tau}` | `hat(tau)` |
| Media muestral | `\bar{X}_n` | `overline(X)_n` |
| Vector en negrita | `\vect{x}` | `vect(x)` |
| σ-álgebra | `\mathcal{F}` | `cal(F)` |
| Reales | `\mathbb{R}^d` | `RR^d` |
| Fracción desplegada | `\frac{a}{b}` | `a/b` o `frac(a, b)` |
| Suma con límites | `\sum_{i=1}^{n}` | `sum_(i=1)^n` |
| Casi seguramente | `\text{c.s.}` | `"c.s."` |
| Texto en modo matemático | `\text{...}` | `"..."` |
| Operador con límites debajo | `\DeclareMathOperator*{\argmax}` | `math.op("arg max", limits: true)` |

**Cuidado en Typst:** el espacio es significativo y el texto entre comillas se
compone en redonda automáticamente, que es lo que se quiere para etiquetas:
`tau_"ATE"` da *τ*<sub>ATE</sub> con el subíndice en redonda, correcto según §1.
En LaTeX el equivalente es `\tau_{\mathrm{ATE}}`.

---

## 6 · Teoremas y demostraciones

- **Numeración compartida por capítulo** para todos los enunciados: Definición
  2.1, Teorema 2.2, Proposición 2.3. Así `Teorema 2.2` y `Definición 2.2` no
  pueden coexistir, y el lector localiza cualquier enunciado por su número sin
  saber de qué tipo es. Ambas plantillas ya lo hacen.
- Cada enunciado con **nombre** cuando es conocido: *Teorema 2.2 (Teorema central
  del límite)*.
- La demostración termina con lápida (∎ o □). Si se omite o se esboza, **dígalo
  explícitamente**: «esbozo», «el desarrollo completo está en el apéndice C».
  Un esbozo presentado como demostración completa es un defecto de honestidad,
  no de estilo.
- Cite la fuente de todo teorema que no demuestre. Reproducir un resultado
  clásico sin atribución es presentarlo implícitamente como propio.

---

## 7 · Errores frecuentes

| Error | Corrección |
|---|---|
| `P(A=1\|X)` escrito con `\|` recto | `\mid` en LaTeX, `mid` en Typst — espaciado correcto |
| `E(Y)` con paréntesis | `E[Y]` con corchetes; los paréntesis son para funciones |
| Afirmar «*Y* ⫫ *A*» cuando se quiere decir condicional | Siempre escribir el condicionamiento: `\| X` |
| «el efecto» sin decir cuál | Nombrar el estimando: ATE, ATT, LATE, CATE |
| *τ* y *τ̂* usados indistintamente | Gorro siempre que sea estimado |
| Positividad escrita como `0 ≤ e(x) ≤ 1` | Es `0 < e(x) < 1`; con igualdad el supuesto se pierde |
| `\frac` dentro de texto corrido | `a/b` |
| Ecuación huérfana sin puntuación | Puntuar según la oración |
