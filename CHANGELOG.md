# Changelog

Formato: [Keep a Changelog](https://keepachangelog.com/es-ES/1.1.0/). Versionado: los releases principales usan `vMAJOR.MINOR.PATCH`; el primer release público es `v0.1.0-demo`.

## [Unreleased]

### Added
- CLI `tesis` (Rust, `tools/tesis/`): `tesis new` crea un proyecto personalizado a partir de la plantilla elegida y `tesis check` detecta placeholders sin sustituir. Incluido en CI (`cargo test`).
- Banner del repositorio y README reescrito: comparación Typst↔LaTeX, FAQ, enlace a Discussions y Star History.
- Archivos comunitarios: `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`, `CITATION.cff`, plantilla de PR y configuración de templates.
- GitHub Discussions habilitado para preguntas y propuestas.
- `.mailmap` para unificar identidades locales y del dueño en `git shortlog`.

### Fixed
- `reproduce.sh` compila a directorio temporal (no ensucia el árbol) y cubre `tesis.typ` y `tesis_doctorado.typ`.
- `.gitignore`: la regla `https/` nunca coincidía con la ruta real `https:/`; corregida y añadida `.cursor/`.
- CI: `permissions` mínimos, `concurrency` con cancelación, runner `ubuntu-24.04`, actions pineadas por SHA (`checkout` v4.4.0, `setup-python` v5.6.0).
- `aplicar-metadatos.sh`: `HOME=` sombreaba la variable `HOME` que `gh` necesita; renombrado a `HOMEPAGE`.
- `LICENSE`: texto oficial CC BY 4.0 (la versión anterior no era detectada por GitHub); atribución © 2026 movida al README.
- README raíz: placeholders de LinkedIn sustituidos, sección Licencia con atribución.
- README LaTeX: `latexmk` declarado en requisitos, tilde en el título, texto sin claims de "alto impacto".
- Typst README: ruta del paquete corregida a `plantilla_tesis_uni_typst/`.

### Removed
- Rama `chore/discovery-improvements` (totalmente fusionada) borrada local y remota.
- Directorio basura `https:/` (artefacto del IDE) eliminado.

## [0.1.0-demo] — primer release público

### Added
- Plantilla Typst completa con ejemplo real (tesis DML), PDF compilado versionado y `tests` de cifras (`verificar_cifras.py`).
- Plantilla LaTeX estructural con `latexmk` y `biber`.
- Guías en `documentacion/` (redacción, tablas, fórmulas, equivalencias, enlaces UNI) y listado `ejemplos_tesis/` con enlaces verificados.
- CI `verificar-plantillas` (Typst + LaTeX) y release `v0.1.0-demo` con el PDF adjunto.
