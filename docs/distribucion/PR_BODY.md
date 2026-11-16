# Plantilla · PR «chore: topics, README y release de demostración»

## Informe Fase 0 (auditoría)

| Ítem | Estado |
|------|--------|
| Topics GitHub | **Confirmado vacío** (`{"names":[]}`) antes del PR |
| Description | **Confirmado:** *«Este es el repositorio del grupo de estudio de Inferencia Causal de la UNI Perú.»* |
| Licencia API | **Discrepancia:** `license.spdx_id` = null; CC BY 4.0 en README/plantillas — sin `LICENSE` en raíz |
| `tesis.pdf` | **Confirmado:** PDF 1.7, 40 páginas |
| `ci.yml` | **Confirmado:** Typst 0.15.1 + LaTeX (latexmk, biber, texlive-lang-spanish) |
| `reproduce.sh` | **Confirmado:** verificar_cifras + compilación opcional |

## Tabla PASS/FAIL (Fase 5)

| Criterio | Estado |
|----------|--------|
| Topics (15 exactos) | Ejecutar `docs/distribucion/aplicar-metadatos.sh` tras merge |
| Description | Igual script |
| README 10 secciones | **PASS** |
| Release `v0.1.0-demo` | **PASS** (creado desde rama) |
| CI Typst + LaTeX | **PASS** (sin duplicar workflow) |
| Sin borrados en diff | **PASS** |
| Sin secretos | **PASS** |

## Pendiente de confirmación del owner

- ~~Añadir `LICENSE` (CC BY 4.0)~~ — **hecho** 2026-11-16: texto oficial completo; GitHub detecta `CC-BY-4.0`.
- ~~`bash docs/distribucion/aplicar-metadatos.sh` tras merge a main~~ — **hecho** 2026-11-16 (15 topics + description + homepage aplicados; además el script tenía un bug `HOME=` que rompía gh → renombrado a `HOMEPAGE`).
- **Renombrar repo (no aplicado):** `plantillas-tesis-uni-peru` · `uni-thesis-templates-typst-latex` — decisión pendiente del owner; ver recomendación en el informe.
