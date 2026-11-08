# Checklist de lanzamiento (descubrimiento)

Orden recomendado; cada paso es verificable antes del siguiente.

1. [ ] **Topics** en GitHub (15 etiquetas acordadas) — `gh api repos/OWNER/REPO/topics`
2. [ ] **Description** y **homepage** (release `v0.1.0-demo`)
3. [ ] **README** con demostración (`docs/tesis-preview.png`) y inicio rápido
4. [ ] **Release** `v0.1.0-demo` con `tesis.pdf` adjunto
5. [ ] **CI** en verde en `main` tras merge del PR de descubrimiento
6. [ ] PR a [awesome-typst](https://github.com/qjcg/awesome-typst) (texto en `awesome-pr.md`)
7. [ ] Post en comunidad (borradores `post-reddit.md` / `post-hn.md`), solo tras 1–5

## Pendiente de owner

- [ ] Archivo `LICENSE` en raíz (CC BY 4.0) si se desea que GitHub muestre la licencia en metadatos — hoy solo consta en README y plantillas.
