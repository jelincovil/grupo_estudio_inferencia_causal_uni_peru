# Contribuir al repositorio

¡Gracias por tu interés! Este es un **kit de trabajo** (plantillas + guías), no un curso. Cualquier mejora que ayude a un tesista a compilar y escribir mejor se agradece.

## Cómo contribuir

1. **Antes de empezar**, revisa [Discussions](https://github.com/jelincovil/grupo_estudio_inferencia_causal_uni_peru/discussions): muchas dudas ya tienen respuesta ahí.
2. Para cambios grandes (nueva plantilla, nueva sección en `documentacion/`), abre primero un issue con la plantilla «Propuesta de plantilla» o una discusión, así evitamos trabajo duplicado.
3. Para arreglos pequeños (typos, enlaces rotos, metadatos), un PR directo está bien.
4. Los PRs deben tener **CI en verde** (`typst-y-python` y `latex`). Tu PR se fusiona por squash o merge según prefiera el maintainer.

## Entorno de verificación

```bash
./reproduce.sh        # cifras del ejemplo + compilación (si hay typst/latexmk)
cargo test --manifest-path tools/tesis/Cargo.toml   # CLI en Rust
```

## Reglas del repositorio

- **No subas archivos generados** (`*.pdf`, `*.aux`, `*.log`, etc.) salvo el ejemplo versionado `plantilla_tesis_uni_typst/tesis.pdf`, que es parte del producto.
- **No recompiles `tesis.pdf` in situ**; `reproduce.sh` ya compila a directorio temporal.
- **No agregues secretos** ni tokens; el repo es público.
- **Normativa UNI** citada es de terceros: respéctala, no la modifiques ni la repliques como propia.
- Mantén el español correcto y la voz neutra (no "tú" promocional).

## Licencia de tus contribuciones

Al abrir un PR aceptas que tu contribución se distribuya bajo [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/), la misma licencia del repositorio.

## ¿Algo parecido a un código de conducta?

Sí: [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md). Sé respetuoso, especialmente con quienes son nuevos en Typst/LaTeX.
