# Guía Completa - Plantilla LaTeX Tesis Maestria/Doctoral

## 📋 Descripción

Plantilla profesional de LaTeX para tesis validada con pruebas de estrés exhaustivas. Diseñada siguiendo estándares internacionales de publicaciones científicas de alto impacto (IMS, AOAS). **Lista para uso en producción universitaria.**

## ✅ Validación y Robustez

Esta versión ha sido sometida a:
- ✓ Pruebas de estrés con contenido complejo
- ✓ Validación de compilación en múltiples escenarios
- ✓ Verificación de compatibilidad de paquetes
- ✓ Optimización de configuraciones
- ✓ Eliminación de warnings críticos

## 🆕 Mejoras de la Versión

### Nuevas Funcionalidades
1. **Soporte para subfiguras** con paquete `subcaption`
2. **Algoritmos profesionales** con `algorithm2e`
3. **Código fuente con sintaxis** usando `listings`
4. **Unidades científicas** con `siunitx`
5. **Referencias cruzadas inteligentes** con `cleveref`
6. **Tablas largas** con `longtable`
7. **Operadores matemáticos personalizados** (Var, Cov, E, argmax, argmin)

### Mejoras de Robustez
1. **Microtype optimizado** con configuración compatible
2. **Hyperref avanzado** con metadatos completos
3. **Gestión de errores** con `etoolbox`
4. **Configuración de captions mejorada**
5. **Comandos personalizados robustos**

## 📚 Características Detalladas

### 1. Subfiguras

```latex
\begin{figure}[H]
    \centering
    \begin{subfigure}[b]{0.45\textwidth}
        \includegraphics[width=\textwidth]{imagen1.png}
        \caption{Primera subfigura}
        \label{fig:sub1}
    \end{subfigure}
    \hfill
    \begin{subfigure}[b]{0.45\textwidth}
        \includegraphics[width=\textwidth]{imagen2.png}
        \caption{Segunda subfigura}
        \label{fig:sub2}
    \end{subfigure}
    \caption{Figura principal con dos subfiguras}
    \label{fig:principal}
\end{figure}
```

### 2. Algoritmos

```latex
\begin{algorithm}[H]
    \SetAlgoLined
    \KwData{Conjunto de datos $X = \{x_1, ..., x_n\}$}
    \KwResult{Estimador bootstrap $\hat{\theta}$}
    \For{$b = 1$ \KwTo $B$}{
        Generar muestra bootstrap $X^*_b$ con reemplazo\;
        Calcular $\hat{\theta}^*_b = f(X^*_b)$\;
    }
    \Return{$\hat{\theta} = \frac{1}{B}\sum_{b=1}^B \hat{\theta}^*_b$}\;
    \caption{Algoritmo Bootstrap}
\end{algorithm}
```

### 3. Código Fuente

```latex
\begin{lstlisting}[language=Python, caption={Implementación en Python}]
import numpy as np

def bootstrap_mean(data, n_bootstrap=1000):
    means = []
    for _ in range(n_bootstrap):
        sample = np.random.choice(data, size=len(data), replace=True)
        means.append(np.mean(sample))
    return np.array(means)
\end{lstlisting}
```

### 4. Unidades con SIunitx

```latex
La temperatura fue de \SI{25}{\celsius} y la presión de \SI{101.3}{\kilo\pascal}.

El intervalo de confianza es $[\num{0.123}, \num{0.456}]$ con $\alpha = \num{0.05}$.
```

### 5. Referencias Cruzadas Inteligentes

```latex
% En lugar de:
Ver Teorema \ref{thm:clt} en la página \pageref{thm:clt}.

% Usar:
Ver \cref{thm:clt}.  % Produce: "Ver Teorema 2.1"

% Para múltiples referencias:
Ver \cref{thm:clt,def:prob_space,fig:principal}.
% Produce: "Ver Teorema 2.1, Definición 2.1 y Figura 3.1"
```

### 6. Operadores Matemáticos Personalizados

```latex
$$
\E[X] = \mu, \quad \Var(X) = \sigma^2
$$

$$
\hat{\theta} = \argmax_{\theta} \mathcal{L}(\theta; X)
$$

$$
\Cov(X, Y) = \E[(X - \E[X])(Y - \E[Y])]
$$
```

### 7. Comandos Útiles Adicionales

```latex
% Vectores en negrita
\vect{x} = (x_1, x_2, ..., x_n)

% Matrices
\mat{A} = \begin{pmatrix} a_{11} & a_{12} \\ a_{21} & a_{22} \end{pmatrix}

% Notas pendientes (eliminar en versión final)
\TODO{Completar esta sección con más detalles}

% Notas al margen
\nota{Revisar esta afirmación}
```

## 🔧 Compilación

### Método Recomendado

```bash
cd /home/ubuntu/latex_project
pdflatex -interaction=nonstopmode tesis.tex
biber tesis
pdflatex -interaction=nonstopmode tesis.tex
pdflatex -interaction=nonstopmode tesis.tex
```

### Script Automático (Actualizado)

```bash
#!/bin/bash
ARCHIVO="tesis"

echo "Compilando tesis doctoral..."
pdflatex -interaction=nonstopmode ${ARCHIVO}.tex > /dev/null 2>&1
biber ${ARCHIVO} > /dev/null 2>&1
pdflatex -interaction=nonstopmode ${ARCHIVO}.tex > /dev/null 2>&1
pdflatex -interaction=nonstopmode ${ARCHIVO}.tex

if [ -f "${ARCHIVO}.pdf" ]; then
    echo "✓ Compilación exitosa: ${ARCHIVO}.pdf"
    ls -lh ${ARCHIVO}.pdf
else
    echo "✗ Error en la compilación"
    exit 1
fi
```

## 📝 Personalización

### Variables Principales

Edita estas líneas al inicio del documento:

```latex
\newcommand{\doctitle}{Título de la Tesis Doctoral}
\newcommand{\docauthor}{Nombre del Autor}
\newcommand{\dockeywords}{palabra clave 1, palabra clave 2, palabra clave 3}
\newcommand{\docsubject}{Tesis Doctoral en Estadística}
\newcommand{\docuniversity}{Universidad Nacional de Ingeniería}
\newcommand{\docfaculty}{Facultad de Ingeniería Económica, Estadística y Ciencias Sociales}
```

### Configuración de Hyperref

Para cambiar colores de enlaces:

```latex
\hypersetup{
    linkcolor=blue,    % Color de enlaces internos
    citecolor=blue,    % Color de citas
    urlcolor=blue      % Color de URLs
}
```

### Configuración de Listas

Para cambiar el espaciado en listas:

```latex
\setlist{nosep}           % Sin espacio entre items
\setlist{itemsep=5pt}     % 5pt entre items
```

## 🎨 Estilos de Código

El paquete `listings` viene preconfigurado con un estilo profesional. Para personalizarlo:

```latex
\lstdefinestyle{mystyle}{
    backgroundcolor=\color{backcolour},   
    commentstyle=\color{codegreen},
    keywordstyle=\color{magenta},
    basicstyle=\ttfamily\footnotesize,
    breaklines=true,
    numbers=left
}
```

Lenguajes soportados: Python, R, C, C++, Java, MATLAB, SQL, Bash, y más.

## 📊 Tablas Profesionales

### Tabla Simple

```latex
\begin{table}[H]
    \centering
    \caption{Resultados del análisis}
    \begin{tabular}{lcc}
        \toprule
        Método & RMSE & R² \\
        \midrule
        Regresión Lineal & 2.34 & 0.85 \\
        Random Forest & 1.89 & 0.92 \\
        \bottomrule
    \end{tabular}
\end{table}
```

### Tabla Larga (Múltiples Páginas)

```latex
\begin{longtable}{lccc}
    \caption{Tabla que ocupa múltiples páginas} \\
    \toprule
    Variable & Media & Mediana & Desv. Est. \\
    \midrule
    \endfirsthead
    
    \multicolumn{4}{c}{{\tablename\ \thetable{} -- continuación}} \\
    \toprule
    Variable & Media & Mediana & Desv. Est. \\
    \midrule
    \endhead
    
    \midrule
    \multicolumn{4}{r}{{Continúa en la siguiente página}} \\
    \endfoot
    
    \bottomrule
    \endlastfoot
    
    X1 & 10.5 & 10.2 & 2.3 \\
    X2 & 15.2 & 15.0 & 3.1 \\
    % ... muchas más filas ...
\end{longtable}
```


## 📦 Paquetes Requeridos

**Esenciales:**
- texlive-latex-base
- texlive-latex-extra
- texlive-bibtex-extra
- texlive-science
- texlive-lang-spanish
- biber

**Opcionales (para funcionalidades avanzadas):**
- texlive-fonts-extra
- texlive-extra-utils

## 🎯 Checklist Pre-Entrega

Antes de entregar tu tesis, verifica:

- [ ] Todos los `\TODO{}` han sido eliminados
- [ ] El paquete `lipsum` ha sido removido (o comentado)
- [ ] Todas las figuras tienen captions descriptivos
- [ ] Todas las tablas tienen captions descriptivos
- [ ] Las referencias bibliográficas están completas
- [ ] Los metadatos del documento están actualizados
- [ ] El documento compila sin errores
- [ ] Los enlaces del PDF funcionan correctamente
- [ ] Las referencias cruzadas son correctas
- [ ] El formato cumple con los requisitos institucionales

## 📄 Estructura de Archivos Recomendada

```
proyecto_tesis/
├── tesis.tex    # Archivo principal
├── referencias.bib                 # Base de datos bibliográfica
├── figuras/                        # Directorio de figuras
│   ├── capitulo1/
│   ├── capitulo2/
│   └── ...
├── diagramas/                      # Diagramas y esquemas
├── logos/                          # Logos institucionales
│   └── logo_uni.png
└── codigo/                         # Código fuente (opcional)
    ├── scripts/
    └── notebooks/
```

## 🌟 Características Profesionales

1. **Metadatos PDF completos** para búsqueda y accesibilidad
2. **Bookmarks automáticos** en el PDF
3. **Enlaces clicables** en referencias y citas
4. **Numeración consistente** de teoremas, definiciones, etc.
5. **Formato de bibliografía** según estándares internacionales
6. **Códigos QR** para enlaces a repositorios
7. **Tipografía optimizada** con microtype
8. **Captions profesionales** con formato consistente

## 📞 Soporte

Para problemas con LaTeX y los paquetes:
- [CTAN](https://ctan.org/)
- [TeX Stack Exchange](https://tex.stackexchange.com/)
- [Overleaf Documentation](https://www.overleaf.com/learn)

---
