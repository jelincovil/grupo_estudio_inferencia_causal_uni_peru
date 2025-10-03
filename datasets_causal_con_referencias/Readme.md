# Repositorio de datasets usados por el grupo de Inferencia Causal UNI

## Introducción

## Objetivos

## Descrición dataset A
Este dataset contiene [Datos sobre una droga determinada].
**Enlace al dataset A:** [ch_01_drug_data.csv](https://raw.githubusercontent.com/PacktPublishing/Causal-Inference-and-Discovery-in-Python/refs/heads/main/data/ch_01_drug_data.csv)
// Fuente: Obtenido del repositorio de GitHub "[Causal-Inference-and-Discovery-in-Python)"

## Descrición dataset B
 

## Descripción del dataset 'lalonde'
El Lalonde dataset proviene del estudio clásico de Robert J. LaLonde (1986) en su artículo “Evaluating the Econometric Evaluations of Training Programs with Experimental Data”; véase el link:
Su objetivo era analizar la efectividad de los programas de capacitación laboral en Estados Unidos, y al mismo tiempo comparar la validez de los métodos econométricos observacionales frente a un experimento controlado aleatorio (RCT).

### Origen de los datos
*Experimental:*
Los datos principales provienen del programa National Supported Work Demonstration (NSW), un experimento social en EE.UU. durante los años 1970s, donde los participantes fueron asignados aleatoriamente a recibir o no recibir capacitación laboral.
*No experimental (observacional):*
LaLonde complementó los datos experimentales con información del Panel Study of Income Dynamics (PSID) y del Current Population Survey (CPS) para simular situaciones donde los métodos de inferencia causal se aplican sin un experimento aleatorio.

### Contenido del dataset
Link 1: https://github.com/robjellis/lalonde/blob/master/lalonde_data.csv
Link 2: https://users.nber.org/~rdehejia/data/.nswdata2.html

El dataset más citado (versión nsw de LaLonde) contiene las siguientes variables:
treat: indicador de tratamiento (1 si participó en el programa de entrenamiento, 0 si no).
* age: edad del individuo.
* educ: años de educación formal completados.
* black: indicador racial (1 si es afroamericano).
* hispan: indicador étnico (1 si es hispano).
* married: indicador de estado civil (1 si casado).
* nodegree: indicador de no haber completado secundaria.
* re74: ingresos reales en 1974.
* re75: ingresos reales en 1975.
* re78: ingresos reales en 1978 (resultado principal de interés).
Importancia metodológica
•	El experimento NSW permite una estimación causal limpia del efecto del programa sobre los ingresos futuros.
•	Al comparar el experimento con los datos observacionales (PSID, CPS), LaLonde mostró que muchos métodos econométricos tradicionales fallaban en replicar los resultados experimentales.
•	Este dataset se convirtió en un benchmark estándar en econometría y en métodos de inferencia causal moderna (matching, propensity scores, weighting, doubly robust, machine learning causal).



## Referencias (norma APA)

- Sharma, A., & Kiciman, E. (2020). DoWhy: An End-to-End Library for Causal Inference. arXiv preprint arXiv:2011.04216. 
- Lalonde, R. (1986). Evaluating the econometric evaluations of training programs with experimental data. American Economic Review 76: 604-620.
- Dehejia, R.H. and Wahba, S. (1999). Causal Effects in Nonexperimental Studies: Re-Evaluating the Evaluation of Training Programs. Journal of the American Statistical Association 94: 1053-1062
