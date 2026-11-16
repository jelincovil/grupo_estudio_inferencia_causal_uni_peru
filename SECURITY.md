# Seguridad

## Alcance

Este repositorio contiene plantillas, guías y un CLI (`tools/tesis/`). No es un servicio en línea; aun así tratamos los reportes con seriedad.

## Reportar una vulnerabilidad

- **Preferido:** usa *Report a vulnerability* en la pestaña **Security** del repositorio (GitHub Security Advisories). El dueño habilitará reportes privados si aún no están activos; si el botón no está disponible, abre un issue con la etiqueta `seguridad` **sin detalles técnicos sensibles** y pide un canal privado.
- Incluye: archivo afectado, ejemplo mínimo y, si es posible, una corrección propuesta.

## Respuesta

- Reconocimiento del reporte en un plazo razonable (hasta ~7 días en periodo lectivo).
- Corrección y aviso en `CHANGELOG.md` (sección *Security*) cuando aplique.
- Sin recompensas económicas; reconocimiento público opcional en el changelog (previa coordinación).

## Buenas prácticas para usuarios

- No pegues tokens ni claves en issues ni PRs.
- No ejecutes `./reproduce.sh` ni scripts de terceros sin leerlos (este repo es texto plano: revísalo primero).
- El PDF de ejemplo es un artefacto compilado; verifícalo con `sha256sum plantilla_tesis_uni_typst/tesis.pdf` si dudas de su integridad (coincide con el adjunto del release `v0.1.0-demo`).
