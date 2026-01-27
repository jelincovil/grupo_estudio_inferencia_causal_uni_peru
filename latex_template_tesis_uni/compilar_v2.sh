#!/bin/bash

# ========================================================================
# Script de Compilación Automática - Tesis Doctoral v2.0 Final
# ========================================================================
# Este script ejecuta el ciclo completo de compilación con validación
# ========================================================================

ARCHIVO="tesis_doctoral_v2_final"
LOG_DIR="/tmp/latex_logs"

# Crear directorio de logs
mkdir -p ${LOG_DIR}

echo "========================================"
echo "Compilación de Tesis Doctoral v2.0"
echo "========================================"
echo ""
echo "Archivo: ${ARCHIVO}.tex"
echo "Fecha: $(date '+%Y-%m-%d %H:%M:%S')"
echo ""

# Limpiar archivos auxiliares previos
echo "[0/4] Limpiando archivos auxiliares..."
rm -f ${ARCHIVO}.aux ${ARCHIVO}.bbl ${ARCHIVO}.bcf ${ARCHIVO}.blg \
      ${ARCHIVO}.log ${ARCHIVO}.out ${ARCHIVO}.run.xml ${ARCHIVO}.toc \
      ${ARCHIVO}.lof ${ARCHIVO}.lot ${ARCHIVO}.synctex.gz 2>/dev/null
echo "✓ Archivos auxiliares eliminados"
echo ""

# Primera compilación con pdflatex
echo "[1/4] Primera compilación con pdflatex..."
pdflatex -interaction=nonstopmode ${ARCHIVO}.tex > ${LOG_DIR}/compile1.log 2>&1
if grep -q "Output written" ${LOG_DIR}/compile1.log; then
    echo "✓ Primera compilación exitosa"
else
    echo "✗ Error en la primera compilación"
    echo "Ver log: ${LOG_DIR}/compile1.log"
    tail -20 ${LOG_DIR}/compile1.log
    exit 1
fi

# Procesamiento de bibliografía con biber
echo "[2/4] Procesando bibliografía con biber..."
biber ${ARCHIVO} > ${LOG_DIR}/biber.log 2>&1
BIBER_EXIT=$?
if [ $BIBER_EXIT -eq 0 ] || [ $BIBER_EXIT -eq 2 ]; then
    # Exit code 2 significa warnings pero no errores críticos
    echo "✓ Bibliografía procesada correctamente"
else
    echo "⚠ Advertencias en biber (no crítico)"
fi

# Segunda compilación con pdflatex
echo "[3/4] Segunda compilación con pdflatex..."
pdflatex -interaction=nonstopmode ${ARCHIVO}.tex > ${LOG_DIR}/compile2.log 2>&1
if grep -q "Output written" ${LOG_DIR}/compile2.log; then
    echo "✓ Segunda compilación exitosa"
else
    echo "✗ Error en la segunda compilación"
    echo "Ver log: ${LOG_DIR}/compile2.log"
    exit 1
fi

# Tercera compilación con pdflatex (para referencias cruzadas)
echo "[4/4] Tercera compilación con pdflatex..."
pdflatex -interaction=nonstopmode ${ARCHIVO}.tex > ${LOG_DIR}/compile3.log 2>&1
if grep -q "Output written" ${LOG_DIR}/compile3.log; then
    echo "✓ Tercera compilación exitosa"
else
    echo "✗ Error en la tercera compilación"
    exit 1
fi

echo ""
echo "========================================"
echo "✓ Compilación completada exitosamente"
echo "========================================"
echo ""

# Mostrar información del archivo generado
if [ -f "${ARCHIVO}.pdf" ]; then
    FILESIZE=$(ls -lh ${ARCHIVO}.pdf | awk '{print $5}')
    echo "📄 Archivo generado: ${ARCHIVO}.pdf"
    echo "📊 Tamaño: ${FILESIZE}"
    
    # Intentar obtener número de páginas
    if command -v pdfinfo &> /dev/null; then
        PAGES=$(pdfinfo ${ARCHIVO}.pdf 2>/dev/null | grep "Pages:" | awk '{print $2}')
        if [ ! -z "$PAGES" ]; then
            echo "📖 Páginas: ${PAGES}"
        fi
    fi
    
    echo ""
    echo "✓ Tesis lista para revisión"
    echo ""
    
    # Verificar warnings importantes
    WARNINGS=$(grep -i "warning" ${LOG_DIR}/compile3.log | wc -l)
    if [ $WARNINGS -gt 0 ]; then
        echo "⚠ Se encontraron ${WARNINGS} advertencias (revisar logs si es necesario)"
    fi
else
    echo "✗ Error: No se generó el archivo PDF"
    exit 1
fi

# Limpiar logs antiguos (mantener solo los últimos 5)
find ${LOG_DIR} -name "*.log" -mtime +7 -delete 2>/dev/null

echo "Logs guardados en: ${LOG_DIR}/"
echo ""
