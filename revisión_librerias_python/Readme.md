# Objetivos

En esta carpeta listamos y referenciamos los modulos estudiados por el grupo de inferencia causal.

# Librería Causalinference

La librería **CausalInference de Python** es una herramienta de código abierto diseñada para aplicar los principales métodos de Inferencia Causal en entornos de investigación y ciencia de datos. Su enfoque está en el Marco del Resultado Potencial (Potential Outcomes Framework), permitiendo a los usuarios estimar el efecto promedio de un tratamiento (Average Treatment Effect o ATE) y el efecto promedio del tratamiento en los tratados (Average Treatment Effect on the Treated o ATT) a partir de datos observacionales

CausalInference implementa de manera eficiente los métodos estadísticos más robustos y populares, como el **Propensity Score Matching (PSM)** y la **Double Robust Estimation**. Su objetivo es tomar datos observacionales, donde la asignación al tratamiento no es aleatoria, y ajustarlos para simular un experimento aleatorizado lo mejor posible. Al final de esta sesión, entenderán cómo usar esta librería para estimar con rigor el impacto real de una política, un medicamento o una campaña, aislando el efecto del tratamiento de otras variables confusoras.

### Fundamentos de Inferencia Causal   

| Bloque: Problema Fundamental | Bloque: Supuestos Clave |
| :--- | :--- |
| Queremos estimar el efecto causal promedio (ATE): | |
| $$\tau = \mathbb{E}[Y(1) - Y(0)]$$ | * No interferencia |
| donde $Y(1)$ y $Y(0)$ son resultados potenciales. | * Asignación ignorable |
| | * Solapamiento estricto |


### Pipeline de Flujo de Trabajo con Libreria CausalInference  
Estos 5 pasos representan un  pipeline completo desde la preparación de datos hasta la estimación causal, manteniendo correspondencia entre teoría y código.  

| Paso | Función Clave | Objetivo |
| :---: | :--- | :--- |
| **1** | `CausalModel()` | Inicialización |
| **2** | `est_propensity()` | Balanceo (Propensity Score) |
| **3** | `trim()` | Validación (Solapamiento) |
| **4** | `est_via_*()` | Estimación |
| **5** | `estimates` | Análisis de Resultados |  
  
  
## 1. Paso 1. Configuración e Inicialización
Importamos las librerías necesarias y generamos datos simulados para el ejemplo.  
  
import pandas as pd  
from causalinference import CausalModel  
from causalinference.utils import random_data   
  
### Generar datos simulados: Y (resultado), D (tratamiento), X (covariables)  
Y, D, X = random_data()  

### Inicializar el Modelo Causal  

cm = CausalModel(Y, D, X)  
  
Modelo Inicializado. Observaciones: 5000  

### Estadísticas Iniciales  ###

Summary Statistics  

**Controls** (N_c=2505)  **Treated** (N_t=2495)    

| Variable | Mean | S.d. | Mean | S.d. | Raw-diff |
| :---: | :---: | :---: | :---: | :---: | :---: |
| Y | -0.977 | 1.765 | 4.962 | 3.007 | 5.939 |

<br>  
<br>  

**Controls** (N_c=2505)  **Treated** (N_t=2495)  
| Variable | Mean | S.d. | Mean | S.d. | Nor-diff |
|---|---|---|---|---|---|
| X0 | -0.322 | 0.956 | 0.326 | 0.954 | 0.679 |
| X1 | -0.347 | 0.942 | 0.351 | 0.943 | 0.741 |
| X2 | -0.328 | 0.925 | 0.301 | 0.941 | 0.674 | 

<br>  
<br>  

# Proceso de Estimación y Validación (Pasos 2, 3 y 4)  

## Paso 2. Estimar el **Propensity Score** $e(X)$ 

### Fundamento Matemático

El *propensity score* es la probabilidad condicional de recibir tratamiento:

$$
e(X_i) = \Pr(D_i = 1 \mid X_i = x)
$$
  
Se estima mediante **regresión logística**:

$$
\log\left(\frac{e(X_i)}{1 - e(X_i)}\right) = \beta_0 + \beta^T X_i
$$

### Implementación en Python de Propensity Score

### Estimar el Propensity Score (regresión logística con términos lineales)
cm.est_propensity(lin='all')
print("\n PASO 2: Propensity Score estimado.")

###  Mostrar resultados reales del modelo logístico
print("\n--- Resultados del Modelo de Propensity Score ---")
print(cm.propensity)

print("\n--- Estadísticas descriptivas después del ajuste ---")
print(cm.summary_stats)

# Resultados del Modelo de Propensity Score

### Estimated Parameters of Propensity Score

| Variable    | Coef. | S.e. | z      | P> abs(z) | [95% Conf. int.] |
|-------------|-------|------|--------|-------|------------------|
| Intercept   | -0.062 | 0.079 | -0.780 | 0.435 | -0.217 a 0.094  |
| X0          | 1.122  | 0.096 | 11.725 | 0.000 | 0.935 a 1.310   |
| X1          | 1.035  | 0.098 | 10.593 | 0.000 | 0.844 a 1.227   |
| X2          | 1.108  | 0.099 | 11.227 | 0.000 | 0.915 a 1.301   |

#### Estadísticas descriptivas después del ajuste

## Summary Statistics

### Diferencias brutas (Raw-diff)

| Variable | Controls (N=515) | Treated (N=485) | Raw-diff |
|          | Mean    | S.d.   | Mean   | S.d.   |          |
|----------|---------|--------|--------|--------|----------|
| Y        | -1.010  | 1.694  | 5.008  | 2.924  | 6.019    |

### Diferencias normalizadas (Nor-diff)

| Variable | Controls (N=515) | Treated (N=485) | Nor-diff |
|          | Mean    | S.d.   | Mean   | S.d.   |          |
|----------|---------|--------|--------|--------|----------|
| X0       | -0.324  | 0.892  | 0.370  | 0.992  | 0.737    |
| X1       | -0.269  | 0.914  | 0.325  | 0.898  | 0.655    |
| X2       | -0.374  | 0.901  | 0.293  | 0.937  | 0.726    |


## PASO 3: Trimming y Validación  

### ⚖️ Fundamento Matemático

Para garantizar **solapamiento** (*overlap*) se requiere que los *propensity scores* no tomen valores extremos:

$$
0 < \eta \leq e(X_i) \leq 1 - \eta < 1
$$

Eliminamos observaciones con *propensity score* extremo, definiendo el subconjunto:

$$
S = \{ i : \alpha \leq \hat{e}(X_i) \leq 1 - \alpha \}
$$

donde $\alpha$ es el **cutoff** (usualmente $0.1$).

### Implementación en Python de Trimming y Validación
cm.trim()   
print("\n PASO 3: Trimming aplicado (Muestra ajustada para solapamiento).")    

### Después del trimming (debe mostrar un N_c y N_t más bajos)    
print("\n--- Observaciones DESPUÉS del Trimming ---")    
print(cm.summary_stats)    

### Observaciones **ANTES del Trimming**   

### Summary Statistics — Outcome (Y)  

<br>

| Variable | Group | Mean | S.d. | Raw-diff |
|-----------|--------|------|------|-----------|
| **Y** | Controls (Nₙc=378) | -0.476 | 1.366 |  |
|  | Treated (Nₜ=396) | 3.967 | 2.199 | **4.443** |

<br>

### Summary Statistics — Covariables (X)  

| Variable | Group | Mean | S.d. | Nor-diff |
|-----------|--------|------|------|-----------|
| **X0** | Controls (Nₙc=378) | -0.276 | 0.814 |  |
|  | Treated (Nₜ=396) | 0.147 | 0.861 | **0.504** |
| **X1** | Controls (Nₙc=378) | -0.133 | 0.862 |  |
|  | Treated (Nₜ=396) | 0.198 | 0.866 | **0.383** |
| **X2** | Controls (Nₙc=378) | -0.150 | 0.916 |  |
|  | Treated (Nₜ=396) | 0.160 | 0.957 | **0.331** |

  

## PASO 4: Estimación de Efectos Causaless  

### Fundamento Matemático de Estimadores:

****************
### 1. Weighting (IPW) — *Inverse Probability Weighting*

El estimador IPW (ponderación por probabilidad inversa) utiliza los *propensity scores* para reponderar las observaciones y estimar el ATE:

<br>
  
$\widehat{\tau}{IPW} = \frac{1}{n} \sum{i=1}^{n} \left( \frac{D_i Y_i}{\widehat{e}(X_i)} - \frac{(1 - D_i) Y_i}{1 - \widehat{e}(X_i)} \right)$

<br>

donde:

- $D_i \in \{0,1\}$ indica si la unidad $i$ recibió el tratamiento,  
- $Y_i$ es el resultado observado,  
- $\widehat{e}(X_i) = \Pr(D_i = 1 \mid X_i)$ es el *propensity score* estimado.

 **Interpretación:** el IPW repondera cada individuo por la probabilidad inversa de recibir el tratamiento que efectivamente recibió, equilibrando las covariables entre grupos tratados y no tratados.

***************
### 2. Matching — *Nearest Neighbor Matching*

El estimador por emparejamiento (*matching*) compara unidades tratadas y no tratadas con valores similares de covariables o *propensity score*:

$$
\widehat{\tau}_{Matching}
= 
\frac{1}{N_T} 
\sum_{i: D_i = 1}
\left(
Y_i - \frac{1}{|J(i)|}
\sum_{j \in J(i)} Y_j
\right)
$$

********


donde:

- $J(i)$ es el conjunto de vecinos más cercanos al individuo $i$ en el grupo de control,  
- $|J(i)|$ es el número de vecinos usados (por ejemplo, 1 en el caso *1-NN*),  
- $N_T$ es el número total de unidades tratadas.

 **Interpretación:** el efecto del tratamiento se obtiene promediando las diferencias de resultados entre cada tratado y sus controles emparejados.

---

### 3. Regresión (OLS) — *Outcome Regression Estimator*

El estimador por regresión lineal ajusta un modelo para el resultado en función del tratamiento y las covariables:

$$
Y_i = \beta_0 + \tau D_i + \beta^\top X_i + \varepsilon_i
$$

El efecto causal promedio estimado (ATE) es el coeficiente $\widehat{\tau}$ asociado a la variable de tratamiento $D_i$:

$$
\widehat{\tau}_{OLS} = \widehat{\beta}_D
$$

 **Interpretación:** este enfoque asume que, condicionalmente a $X$, la relación entre $D$ y $Y$ es lineal, y que no hay variables omitidas que confundan el efecto.

---

### 4. Blocking — *Stratification (Subclassification) Estimator*

El método de *blocking* o estratificación divide el espacio de los *propensity scores* en $K$ bloques homogéneos:

$$
\widehat{\tau}_{Blocking}
=
\sum_{k=1}^{K}
\frac{n_k}{n}
\left(
\overline{Y}_{1k} - \overline{Y}_{0k}
\right)
$$

donde:

- $n_k$ es el número de observaciones en el bloque $k$,  
- $\overline{Y}_{1k}$ y $\overline{Y}_{0k}$ son las medias de $Y$ para tratados y controles dentro del bloque $k$.

 **Interpretación:** el efecto se estima dentro de cada estrato y luego se promedia ponderando por el tamaño de cada bloque, garantizando comparabilidad local.


***************

### Implementación en Python de Estimadores:
print("\n PASO 4: Ejecutando Estimadores Causaless...")  
cm.est_via_weighting()  
cm.est_via_matching(matches=1)  
cm.est_via_ols(adj=2)  

print(cm.estimates)

### Ejecutando Estimadores Causales...(Paso 4)

---
### Treatment Effect Estimates: Weighting

| Effect | Est. | S.e. | $\lvert z\rvert$ | P > $\lvert z\rvert$ | [95% Conf. Interval] |
|:-------|-----:|-----:|-----------------:|--------------------:|:---------------------:|
| **ATE** | 2.902 | 0.090 | 32.275 | 0.000 | [2.725 , 3.078] |

---

###  Treatment Effect Estimates: Matching

| Effect | Est. | S.e. | $\lvert z\rvert$ | P > $\lvert z\rvert$ | [95% Conf. Interval] |
|:-------|-----:|-----:|-----------------:|--------------------:|:---------------------:|
| **ATE** | 3.158 | 0.156 | 20.236 | 0.000 | [2.852 , 3.464] |
| **ATC** | 2.675 | 0.193 | 13.847 | 0.000 | [2.297 , 3.054] |
| **ATT** | 3.620 | 0.175 | 20.697 | 0.000 | [3.277 , 3.962] |

---

### Treatment Effect Estimates: OLS

| Effect | Est. | S.e. | $\lvert z\rvert$ | P > $\lvert z\rvert$ | [95% Conf. Interval] |
|:-------|-----:|-----:|-----------------:|--------------------:|:---------------------:|
| **ATE** | 2.897 | 0.078 | 37.351 | 0.000 | [2.745 , 3.049] |
| **ATC** | 2.364 | 0.085 | 27.850 | 0.000 | [2.198 , 2.531] |
| **ATT** | 3.406 | 0.087 | 39.210 | 0.000 | [3.236 , 3.576] |



## PASO 5: Análisis de Resultados

print("\n PASO 5: Resultados Finales de Estimación Causal (ATE, ATT, ATC) ---")
print(cm.estimates)

### Resultados Finales de Estimación Causal: $ATE$, $ATT$, $ATC$ - Paso 5

---

### Treatment Effect Estimates: Weighting

| Effect | Est. | S.e. | $\lvert z\rvert$ | $P > \lvert z\rvert$ | [95% Conf. Interval] |
|:-------|-----:|-----:|-----------------:|--------------------:|:---------------------:|
| **ATE** | 3.068 | 0.095 | 32.270 | 0.000 | [2.881 , 3.254] |

---

###  Treatment Effect Estimates: Matching

| Effect | Est. | S.e. | $\lvert z\rvert$ | $P > \lvert z\rvert$ | [95% Conf. Interval] |
|:-------|-----:|-----:|-----------------:|--------------------:|:---------------------:|
| **ATE** | 3.337 | 0.150 | 22.294 | 0.000 | [3.043 , 3.630] |
| **ATC** | 2.878 | 0.177 | 16.225 | 0.000 | [2.530 , 3.226] |
| **ATT** | 3.782 | 0.172 | 21.940 | 0.000 | [3.444 , 4.120] |

---

### Treatment Effect Estimates: OLS

| Effect | Est. | S.e. | $\lvert z\rvert$ | $P > \lvert z\rvert$ | [95% Conf. Interval] |
|:-------|-----:|-----:|-----------------:|--------------------:|:---------------------:|
| **ATE** | 3.081 | 0.080 | 38.499 | 0.000 | [2.924 , 3.237] |
| **ATC** | 2.573 | 0.087 | 29.436 | 0.000 | [2.401 , 2.744] |
| **ATT** | 3.574 | 0.091 | 39.320 | 0.000 | [3.396 , 3.752] |


<br>

Sobre los resultados de estimación causal (ATE, ATT, ATC), los resultados de los tres métodos **Weighting, Matching y OLS** muestran un alto grado de consistencia en la magnitud y dirección del efecto estimado, lo cual refuerza la validez empírica del hallazgo causal. A continuación se realiza un análisis detallado por estimador y una comparación general.

A nivel de Metodos de evaluación:

1. Método Weighting (Ponderación por probabilidad de tratamiento inversa)  
- El estimador de efecto promedio del tratamiento (ATE) es de $3.068$, con un error estándar de $0.095$, y un estadístico $|!z!| = 32.27$ ($p < 0.001$).
Este método busca reponderar la muestra para balancear las covariables entre grupos tratados y de control, simulando así un experimento aleatorizado.  
- El valor alto de $|!z!|$ y el estrecho intervalo de confianza $[2.881, 3.254]$ indican alta eficiencia y robustez estadística, sugiriendo que el reponderado logró un adecuado balance entre grupos. Sin embargo, la interpretación causal depende críticamente de que las probabilidades de tratamiento se estimen correctamente y de que se cumpla el supuesto de positividad (overlap adecuado entre grupos).  

<br>

2. Método Matching (Emparejamiento de unidades comparables)  
- Los resultados del método de emparejamiento son coherentes con los de weighting, aunque presentan una ligera mayor variabilidad.
   - $ATE = 3.337$  
   - $ATT = 3.782$  
   - $ATC = 2.878$
- El hecho de que $ATT > ATC$ sugiere que el tratamiento tiene un efecto más fuerte sobre los individuos tratados que sobre los no tratados, lo que podría reflejar heterogeneidad en la respuesta causal.
- El incremento en el error estándar (por ejemplo, $0.172$ en el ATT) se explica por la reducción efectiva del tamaño muestral al realizar los emparejamientos y por la dependencia inducida entre observaciones pareadas.
- Aun así, la alta significancia estadística ($p < 0.001$ en todos los casos) confirma que el efecto causal es sólido y positivo.  

<br>

3. Método OLS (Regresión lineal controlada)
- El método OLS produce resultados muy cercanos:
   - $ATE = 3.081$  
   - $ATT = 3.574$  
   - $ATC = 2.573$  
- El **bajo error estándar** ($0.080$ en el ATE) y el **estadístico z extremadamente alto** ($|!z!| = 38.499$) reflejan una **alta eficiencia**, aunque cabe recordar que el OLS asume linealidad y homocedasticidad, y no garantiza control perfecto del sesgo por confusión si las covariables no se especifican adecuadamente.
- El patrón $ATT > ATE > ATC$ es consistente con lo observado en el método Matching, reforzando la hipótesis de **efectos de tratamiento heterogéneos**, posiblemente debidos a diferencias sistemáticas en las características de los grupos tratados y de control.  


## Comparación general entre métodos  

| Método     | Precisión (Error Est.) | Robustez causal | Supuestos clave | Interpretación |
|:----------|:----------------------:|:---------------:|:----------------|:---------------|
| Weighting  | Alta                   | Alta si overlap adecuado | Correcta especificación del modelo de propensión | ATE eficiente y balanceado |
| Matching   | Moderada               | Muy robusto a la forma funcional | Buena calidad de emparejamiento | ATE/ATT/ATC heterogéneos |
| OLS        | Muy alta               | Sensible a especificación lineal | Linealidad y ausencia de confusores no observados | Efectos consistentes, pero dependientes del modelo |


## Conclusión general

- Los tres métodos convergen hacia un efecto causal positivo de magnitud aproximada a 3 unidades, lo que confiere consistencia interna y externa a los resultados.
El método Weighting presenta un balance óptimo entre eficiencia y validez causal, mientras que Matching confirma la estabilidad del efecto bajo una estrategia más no paramétrica. Finalmente, OLS ofrece un contraste paramétrico de alta precisión, aunque dependiente de los supuestos del modelo.

- En conjunto, estos hallazgos son indicativos de una relación causal robusta y estable entre el tratamiento y el resultado, con heterogeneidad leve en la intensidad del efecto según el subgrupo analizado (tratados vs. no tratados).

<br>

## Referencias 
- https://causalinferenceinpython.org/causalinference.html  

- Rosenbaum, P. R., & Rubin, D. B. (1983). The central role of the propensity score in observational studies for causal effects. Biometrika, 70(1), 41-55.
  (Esta referencia introduce el Propensity Score, un componente clave de los métodos implementados en la librería).  

- Imbens, G. W., & Rubin, D. B. (2015). Causal inference for statistics, social, and biomedical sciences: An introduction. Cambridge University Press.
  (Este libro es una referencia fundamental sobre el Marco del Resultado Potencial, que es el modelo conceptual que CausalInference implementa).

## Referencias 
