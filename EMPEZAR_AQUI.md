# Empiece aquí

No elija plantilla todavía. Primero la pregunta incómoda:

> **¿Su intervalo puede ser estrecho y estar completamente equivocado?**

En este repositorio la respuesta es **sí**, con números que puede verificar en menos de un minuto.

## 1 · La lección (60 s)

```bash
python3 leccion_60s.py
```

Abra [`docs/la_leccion.html`](docs/la_leccion.html) en el navegador si prefiere la imagen antes que la terminal.

Comparta el resultado:

```bash
python3 leccion_60s.py --compartir
```

## 2 · Normativa antes que formato

- [Formato de Presentación de Tesis (DIRCE UNI)](https://dirce.uni.edu.pe/reglamentos-uni)
- Secretaría FIEECS y guías de entrega al repositorio institucional

Detalle curado: [`documentacion/06-biblioteca-enlaces.md`](documentacion/06-biblioteca-enlaces.md)

## 3 · Qué tipo de afirmación defiende

Lea [`documentacion/00-modelo-tesis.md`](documentacion/00-modelo-tesis.md) §1 y complete:

*«Antes de esta tesis no se sabía ______; ahora se sabe porque ______.»*

Si el segundo hueco solo dice «apliqué X a Y», el trabajo es de **maestría** (categoría, no defecto).

## 4 · Estimando y supuestos (una página)

Antes del capítulo 1: parámetro causal nombrado, tabla supuesto × estimador, al menos un análisis de sensibilidad.

Guía: [`documentacion/00-modelo-tesis.md`](documentacion/00-modelo-tesis.md) §2.

## 5 · Elija herramienta

| Si… | Use… |
| :--- | :--- |
| Quiere la demo causal completa ya escrita | [`plantilla_tesis_uni_typst/`](plantilla_tesis_uni_typst/) |
| Prefiere LaTeX clásico (andamio + `\lipsum`) | [`plantilla_tesis_uni_latex/`](plantilla_tesis_uni_latex/) |
| No sabe | [`documentacion/05-equivalencias-latex-typst.md`](documentacion/05-equivalencias-latex-typst.md) |

## 6 · Compile la demo antes de escribir

```bash
./reproduce.sh
```

Si `verificar_cifras.py` falla, arregle el entorno **ahora**, no la semana de la entrega.

## 7 · Escriba con las guías abiertas

| Guía | Para… |
| :--- | :--- |
| [`01-guia-editorial.md`](documentacion/01-guia-editorial.md) | Vocabulario causal, tiempos verbales, números en español |
| [`03-guia-formulas.md`](documentacion/03-guia-formulas.md) | τ vs τ̂, notación |
| [`04-guia-referencias.md`](documentacion/04-guia-referencias.md) | APA, verificación de enlaces |

## 8 · Antes de entregar al asesor

Lista de verificación: [`01-guia-editorial.md`](documentacion/01-guia-editorial.md) §7.

---

*Grupo de Estudio de Inferencia Causal · UNI · Perú*
