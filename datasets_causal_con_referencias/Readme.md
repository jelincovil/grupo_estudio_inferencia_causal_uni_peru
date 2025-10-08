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

## Descrición dataset C

### Descripción del dataset Fev.dat.txt
Un conjunto de datos sobre la relación entre la función respiratoria (medida mediante el volumen espiratorio forzado, VEF1) y el tabaquismo proporciona una herramienta eficaz para investigar una amplia variedad de cuestiones estadísticas. Este artículo ofrece una breve descripción del problema, los datos y diversas cuestiones y análisis que plantea la cuantificación de la relación entre el VEF1 y el tabaquismo.

### Datos y contenido del dataset

El dataset recoge información sobre la función respiratoria medida en niños, más específicamente el volumen espirado forzadamente (“forced expiratory volume”, FEV).
Las variables incluidas incluyen edad (age), altura (height), género (gender), y hábito de fumar (smoke), entre otras variables explicativas posibles.
El número de observaciones es de aproximadamente n = 345 niños de entre 6 y 10 años para uno de los ejemplos de uso del dataset. PennState: Statistics Online Courses
La variable dependiente principal (“respuesta”) es FEV, el volumen que el niño puede exhalar forzadamente, como indicador de la función pulmonar. rstudio-pubs-static.s3.amazonaws.com

### Paper relacionado: Un problema exhalante para la enseñanza de la estadística
Revista de Educación Estadística, Volumen 13, Número 2 (2005), https://jse.amstat.org/v13n2/datasets.kahn.html

Uso en enseñanza / enfoque didáctico
Kahn propone que el dataset puede servir como hilo conductor durante el curso de estadística, permitiendo al profesor introducir de forma progresiva temas como regresión lineal simple, regresión múltiple, interacciones, supuestos de los modelos (residuals, homocedasticidad, normalidad, etc.), diagnóstico de colinealidad, interpretación de coeficientes, posibles efectos no lineales, etc.
Dado que los datos son reales y tienen “imperfecciones” propias de datos de la vida real, permiten discutir problemas reales en modelado y análisis de datos (por ejemplo, valores atípicos, variabilidad, ajustes, etc.).

### El formato del Archivo fev.dat.txt describe lo siguiente:

| Columnas | Variable | Tipo / Etiqueta | Descripción |
|-----------|-----------|----------------|--------------|
| 1–3       | edad      | Discreto (entero positivo, en años) | Edad del individuo en años completos |
| 5–10      | fiebre    | Continua (litros) | Volumen espiratorio forzado (FEV): cantidad de aire exhalado en litros durante el primer segundo de una espiración forzada |
| 12–15     | alto      | Continua (pulgadas) | Altura del individuo medida en pulgadas |
| 19        | sexo      | Discreto (binario: 0 = mujer, 1 = hombre) | Sexo del individuo |
| 25        | fumar     | Discreto (binario: 0 = no fumador, 1 = fumador) | Indica si el individuo fuma o no |


## Referencias (norma APA)

- Sharma, A., & Kiciman, E. (2020). DoWhy: An End-to-End Library for Causal Inference. arXiv preprint arXiv:2011.04216. 
- Lalonde, R. (1986). Evaluating the econometric evaluations of training programs with experimental data. American Economic Review 76: 604-620.
- Dehejia, R.H. and Wahba, S. (1999). Causal Effects in Nonexperimental Studies: Re-Evaluating the Evaluation of Training Programs. Journal of the American Statistical Association 94: 1053-1062
- Kahn, M. J. (2005). An Exhalent Problem for Teaching Statistics. Journal of Statistics Education, 13(2). Recuperado de https://jse.amstat.org/v13n2/datasets.kahn.html
- 
