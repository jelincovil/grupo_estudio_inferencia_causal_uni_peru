# Plantilla LaTeX de Tesis Maestria/Doctoral 

## 📋 Descripción

Plantilla profesional de LaTeX para tesis doctorales diseñada siguiendo los estándares de publicaciones científicas de alto impacto (IMS, AOAS). Optimizada para investigaciones en Estadística, Matemáticas y Ciencias de Datos.

## ✨ Características Destacadas

- ✅ **Soporte bilingüe** (Español/Inglés) con comandos personalizados
- ✅ **Metadatos PDF automáticos** para accesibilidad y búsqueda
- ✅ **Códigos QR integrados** para enlazar repositorios y datos
- ✅ **Entornos matemáticos profesionales** (teoremas, definiciones, demostraciones)
- ✅ **Bibliografía con BibLaTeX/Biber** estilo autor-año
- ✅ **Tablas científicas con booktabs** (sin líneas verticales)
- ✅ **Gestión jerárquica de imágenes** por carpetas
- ✅ **Estructura modular** fácil de personalizar

## 📁 Estructura de Archivos

```
latex_project/
├── tesis_doctoral_internacional.tex    # Archivo principal
├── referencias.bib                     # Base de datos bibliográfica
├── compilar.sh                         # Script de compilación automática
├── GUIA_DE_USO.md                      # Guía detallada de uso
├── README.md                           # Este archivo
├── logo_uni.png                        # Logo institucional
└── figuras/                            # Directorio para imágenes (crear)
```

## 🚀 Inicio Rápido

### 1. Personalizar Metadatos

Edita las siguientes líneas en `tesis_maestria-doctoral_internacional.tex`:

```latex
\newcommand{\doctitle}{Título de la Tesis Doctoral}
\newcommand{\docauthor}{Nombre del Autor}
\newcommand{\dockeywords}{palabra clave 1, palabra clave 2, palabra clave 3}
\newcommand{\docsubject}{Tesis Doctoral en Estadística}
```

### 2. Compilar el Documento

#### Opción A: Script automático (recomendado)
```bash
./compilar.sh
```

#### Opción B: Compilación manual
```bash
pdflatex tesis_doctoral_internacional.tex
biber tesis_doctoral_internacional
pdflatex tesis_doctoral_internacional.tex
pdflatex tesis_doctoral_internacional.tex
```

### 3. Agregar Referencias

Edita el archivo `referencias.bib` con tus citas bibliográficas en formato BibLaTeX.

## 📚 Documentación

Consulta `GUIA_DE_USO.md` para:
- Uso de entornos matemáticos
- Inserción de figuras y tablas
- Gestión de citas bibliográficas
- Personalización avanzada
- Solución de problemas comunes

## 🔧 Requisitos del Sistema

### Paquetes de TeX Live necesarios:
```bash
sudo apt-get install -y \
    texlive-latex-base \
    texlive-latex-extra \
    texlive-bibtex-extra \
    texlive-science \
    texlive-lang-spanish \
    biber
```

## 📖 Ejemplo de Uso

### Definir un teorema:
```latex
\begin{theorem}[Teorema Central del Límite]
Sea $(X_n)_{n \geq 1}$ una sucesión de variables aleatorias...
$$
\frac{\sqrt{n}(\bar{X}_n - \mu)}{\sigma} \xrightarrow{d} N(0,1)
$$
\end{theorem}
```

### Citar una referencia:
```latex
Según \textcite{efron1979bootstrap}, el método bootstrap...
```

### Insertar código QR:
```latex
\qrlink{https://github.com/usuario/proyecto}{Código fuente en GitHub}
```

## 🎯 Estándares Implementados

Esta plantilla implementa las siguientes mejores prácticas:

1. **Arquitectura modular** con separación clara de configuraciones
2. **Tipografía científica** optimizada para legibilidad
3. **Gestión bibliográfica moderna** con BibLaTeX/Biber
4. **Accesibilidad digital** con metadatos PDF completos
5. **Reproducibilidad** mediante enlaces a repositorios externos
6. **Estética profesional** siguiendo journals de alto impacto

## 📝 Licencia

Creative Commons CC BY 4.0

## 🤝 Contribuciones

Esta plantilla está diseñada para ser adaptable a diferentes instituciones y disciplinas. Siéntete libre de personalizarla según tus necesidades.

## 📧 Soporte

Para preguntas sobre LaTeX y los paquetes utilizados, consulta:
- [CTAN - Comprehensive TeX Archive Network](https://ctan.org/)
- [TeX Stack Exchange](https://tex.stackexchange.com/)
- [Overleaf Documentation](https://www.overleaf.com/learn)

---

