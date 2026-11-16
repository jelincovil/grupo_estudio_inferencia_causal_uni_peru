# `tesis` — CLI del kit de plantillas UNI

Pequeño binario en Rust para **crear** un proyecto de tesis personalizado y **verificar** que el proyecto está listo para compilar. Las plantillas van embebidas en el binario: no descarga nada.

## Instalación

```bash
git clone https://github.com/jelincovil/grupo_estudio_inferencia_causal_uni_peru.git
cd grupo_estudio_inferencia_causal_uni_peru
cargo install --path tools/tesis
tesis --help
```

## Uso

Crear un proyecto nuevo (Typst recomendado):

```bash
tesis new mi-tesis --plantilla typst \
  --titulo "Mi tesis sobre …" \
  --autor "Nombre Apellido" \
  --asesor "Dra. Asesora X" \
  --anio 2026
cd mi-tesis && ./compilar.sh
```

Sin opciones, copia la plantilla tal cual (con su ejemplo) para editarla a mano:

```bash
tesis new mi-tesis --plantilla latex
```

Verificar un proyecto (detecta placeholders sin sustituir y archivos críticos faltantes):

```bash
tesis check mi-tesis
echo $?   # 0 = listo; 1 = hay problemas; 2 = error de uso
```

## Qué copia `tesis new`

- **Typst:** `tesis.typ`, `tesis_doctorado.typ`, `plantilla.typ`, `referencias.bib`, `logo_uni.png`, `compilar.sh`, `README.md`, `requirements.txt`. **No** copia el PDF de ejemplo, las figuras del demo ni los scripts de Monte Carlo (son del ejemplo DML).
- **LaTeX:** `tesis.tex`, `referencias.bib`, `logo_uni.png`, `compilar.sh`, `README.md`, `GUIA_COMPLETA.md`.
- `--force` sobrescribe los archivos de la plantilla en un directorio existente **sin borrar** tus archivos.

## Sustitución de metadatos

Typst: claves `titulo`, `autor`, `asesor`, `anio` del bloque `#let meta = (...)`.
LaTeX: `\doctitle`, `\docauthor`, `\docadvisor`, `\docyear`.

Valores con comillas, barras invertidas, llaves o saltos de línea se rechazan con un mensaje claro (la plantilla es texto: conviene mantenerla escapada a mano).

## Desarrollo

```bash
cargo test --manifest-path tools/tesis/Cargo.toml
```
