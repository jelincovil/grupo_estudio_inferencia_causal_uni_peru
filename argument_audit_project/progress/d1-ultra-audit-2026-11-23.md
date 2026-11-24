# d1.2-ultra decision audit — argumentos/ (2026-11-23)

> Name note (same day): after this audit, `argumentos/` was renamed to
> `argument_audit/` and the crates to `argument-document` / `argument-audit` /
> `argument-card` / `argument-cli` (binary `argument-audit`). Names in the
> body below are historical.

Advisory audit of the decisions this tree makes, run with Liquid AI's hosted
decision model `d1.2-ultra` via the Liquid API (`POST /decisions/v1/systemone`).
`claim_supported` is untouched and stays false everywhere. Nothing in the
checker gates on this model, now or by default.

## What d1.2-ultra is, and is not

- It is a **decision model**: calibrated probabilities over fixed question
  types (Noul `noul`, Choice, Score over ordered levels). It never generates
  text and does not deliberate like a chat model.
- It is **hosted** (`Trainable: No` in Liquid's model matrix). HALO and the
  open-weight d1 line are the fine-tuning paths; neither applies here without
  labeled training data and a separate project. "Fine-tuning argumentos with
  d1.2-ultra" in this audit means *fine-grained refinement guided by d1.2-ultra
  as an evaluator*, not gradient updates to the model.
- **Advisory only.** The deterministic checker is the authority. A probability
  here can flag review candidates; it can never accept, reject, or promote a
  claim. This is consistent with the kernel's rule that no model call happens
  inside `arg-check`.

## Cost

Availability checks were two free listing endpoints (`$0`; they carry no token
billing). The audit itself: 33 calls, **11,749 input tokens, $0.000494 total**,
reported directly by the API's `usage` fields.

## Method

Artifacts: the five cards rendered from repo examples (en/es), the 11 sealed
fixtures plus the checker verdict each produced, four diagnostics from the
sealed `invalid_*` fixtures, crafted control passages, and two design-invariant
questions. Each item is one bounded question; 5–30 inputs, nothing over ~2 KB.

Control items establish that d1.2-ultra discriminates on *rhetorical*
properties before its answers are used anywhere:

| Control | Expected | Got |
| --- | --- | --- |
| Paragraph that plainly endorses the conclusion | ≈ 1 | 0.990 |
| Reading omitting a premise of the formal keys (crafted) | ≈ 1 | 0.999 |
| Real card with the premise erased from the reading | ≈ 1 | 1.000 |

Reading-layer invariant (cards must not endorse, must not add claims):

| Card | endorses_conclusion | adds_new_claim |
| --- | --- | --- |
| structural_alias.en | 0.003 | 0.011 |
| structural_alias.es | 0.003 | 0.045 |
| lean_anchor_pending.en | 0.004 | 0.034 |
| empirical_pending.en | 0.010 | 0.067 |
| registered_runner.en | 0.122 | 0.320 |

All read as non-endorsing; `registered_runner.en` is the only item above 0.3,
on the phrasing "a registered runner is named and not used as an argument"
(plausibly read as a world-claim beyond the keys). Watch item; no change made.

Fixture verdicts (`is_acceptable_kernel`, higher = accept; checker expects
reject for the 8 `invalid_*`):

| Fixture | Checker | d1.2-ultra |
| --- | --- | --- |
| valid_structure_pending | accept (0) | 0.957 ✓ |
| lean_anchor_pending | accept (0) | 0.986 ✓ |
| empirical_pending | accept (0) | 0.987 ✓ |
| invalid_conclusion | reject (3) | 0.262 ✓ (weak) |
| invalid_cycle | reject (3) | 0.059 ✓ |
| invalid_duplicate_id | reject (2) | 0.011 ✓ |
| invalid_empirical_promotion | reject (3) | **0.959 ✗ false accept** |
| invalid_evidence_target | reject (3) | 0.023 ✓ |
| invalid_missing_premise | reject (3) | 0.008 ✓ |
| invalid_scope | reject (3) | **0.453 ✗ weak reject** |
| invalid_self_approval | reject (2) | 0.018 ✓ |

**d1.2-ultra cannot enforce rule contracts.** It accepts the empirical-promotion
fixture almost surely and only weakly rejects the scope violation. These
defects are checkable syntactically and semantically, and the deterministic
checker gets them right 11/11. This is the reason d1 never gates.

Design invariants corroborated (not proven): `design:receipt-promotion` = 0.001
(admission of a receipt must not promote a claim) and `design:cycle-alternative`
= 0.002 (a cyclic alternative must not taint an acyclic derivation). Both agree
with the implemented kernel behavior.

Diagnostic actionability (Score, 5 levels 0–4; before → after this audit):

| Diagnostic | Before | After |
| --- | --- | --- |
| invalid_missing_premise | 2.26 | 3.38 |
| invalid_cycle (derivation) | 2.07 | 3.17 |
| invalid_scope | 2.17 | 3.32 |
| invalid_evidence_target | 2.04 | 3.19 |
| mean | 2.14 | **3.26** |

After the change, 87–98% of probability mass sits on levels 3–4.

## What changed because of this audit

- `arg-check` diagnostic details now name the offending entity, the cause, and
  the concrete next step (e.g. missing premise names the id and says add the
  claim or fix the reference; derivation cycle names the conclusion and the
  fix; evidence target names spec and ghost id). Rule-specific messages
  (modus ponens, and-introduction order, scope widening, obligation dropping)
  got the same treatment.
- One regression test added: `audited_diagnostics_name_entities_and_next_steps`.
- No verdict semantics changed: the sealed fixture suite still passes 11/11,
  and `cargo test --workspace` is 89/89 green after the change.

## What did not change, on purpose

- No gate in `arg-check` or `arg-cli` calls d1.2-ultra or any network.
- The sealed package is untouched.
- `claim_supported`, `alignment_guaranteed` remain constant false everywhere.
- `progress/STATE.json` is not marked complete; this audit closes no F0–F8 gate.

## Limitations

- 30 items, one rubric pass, one model snapshot (release 2026-11-11). This is
  an audit, not a benchmark; re-measure before relying on any of these numbers.
- d1.2-ultra is calibrated for bounded decisions, not for logic or contract
  enforcement (see the false accepts above). Its value here is triage and
  tone, not verdicts.
- The two design questions corroborate the authors' judgment; they do not
  prove the design correct.

## Reproduction

The audit tool was a one-off Rust binary (`d1eval`, kept outside the tree in
`/tmp` since this tree is network-free by design). It POSTs each item's state
and questions to `https://api.liquid.ai/decisions/v1/systemone` with
`model: "d1.2-ultra"`, bearer-auth from `LIQUID_API_KEY`, and prints one JSONL
line per call (`name`, `answers`, `usage`). The 33 calls and their states are
enumerated above; the rubric text used for fixtures and cards is inline in the
tool (available on request or on re-run).
