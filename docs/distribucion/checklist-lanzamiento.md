# Checklist de lanzamiento (descubrimiento)

Orden recomendado; cada paso es verificable antes del siguiente.

1. [x] **Topics** en GitHub (15 etiquetas acordadas) — aplicado 2026-11-16 — `gh api repos/jelincovil/grupo_estudio_inferencia_causal_uni_peru/topics`
2. [x] **Description** y **homepage** (release `v0.1.0-demo`) — aplicado 2026-11-16
3. [x] **README** con demostración (`docs/tesis-preview.png`) y inicio rápido
4. [x] **Release** `v0.1.0-demo` con `tesis.pdf` adjunto (sha256 verificado contra el PDF versionado)
5. [x] **CI** en verde en `main` tras merge del PR de descubrimiento
6. [x] PR a [awesome-typst](https://github.com/qjcg/awesome-typst) — abierto 2026-11-16: [qjcg/awesome-typst#279](https://github.com/qjcg/awesome-typst/pull/279) (una sola entrada en la sección General, texto de `awesome-pr.md` adaptado al estilo de la lista)
7. [ ] Post en comunidad (borradores `post-reddit.md` / `post-hn.md`) — **publicación a cargo del owner** (sin cuentas/credenciales del autor no se puede publicar por él); verificados consistentes con el repo (CC BY 4.0, topics, `latexmk`)

## Pendiente de owner

- [x] Archivo `LICENSE` en raíz con el texto oficial CC BY 4.0 — aplicado 2026-11-16 (texto completo de creativecommons.org para que GitHub detecte la licencia; la atribución © 2026 Grupo de Estudio está en `README.md`).
