# Argument kernel base

Administrative kernel for typed claims, dependencies, and evidence records.
A book is one corpus. This tree is the place to start; it is not a certified
reading of any book, and it is not `CORPUS_CLOSED`.

The sealed specification stays in `../argumentos_construccion_v1/`. This
crate does not modify it. Schema 1.0 files from that package load as the
`scientific_audit` profile.

## What is general

- Claims point at a `source` (`system` + `key`). `book_key` remains only so
  the existing book fixtures load. A specification, paper, dataset, or case
  uses schema 1.1 and a `corpus` object.
- Rules live in a compiled profile. A document cannot invent a rule.
  - `scientific_audit`: the seven v1 administrative rules (Lean anchor/apply,
    and-introduction, modus ponens, empirical record, conditional application,
    exact alias).
  - `structural`: modus ponens, and-introduction, and exact alias. Use this
    for a corpus that only needs dependency tracking.
- Evidence classes are `lean`, `python`, and `registered`. A registered
  runner can be named. It does not satisfy a Lean anchor or an empirical
  record.
- Status is computed. `approved`, `lean_checked`, `reproduced`, and
  `claim_supported` are rejected if a document tries to write them.
- Cycles are per derivation. A bad alternative does not invalidate a separate
  acyclic argument for the same claim.
- A receipt can be loaded and admitted only as a candidate. This base never
  sets `claim_supported`, even when `execution_passed` and
  `scientific_criterion_met` are both true. Replay is not implemented.

## Reading layer

`argument-card` is the Palomar split for arguments, not for Lean proofs.
Palomar's Comparator holds the formal statement apart from the proof.
A language-model review may say the informal account looks fair, and
that review is not an endorsement. Scores are withheld because they move.

This layer does the same job without a model call:

- `argument-audit render` writes a controlled card. The formal keys are the
  statement. The fenced paragraph is a reading. An optional lexicon is a
  hover gloss for atom ids, written by a person, never inferred.
- The card parses back to the same formal projection in English or Spanish.
  Free prose does not parse into a kernel.
- `argument-audit align` compares a card, or free prose, with the kernel.
  `no_blocking_problem` means the formal keys match. It does not mean the
  wording was endorsed. The report has no score. `claim_supported` and
  `alignment_guaranteed` stay false.

A live language-model reviewer, in the Palomar sense, is not connected.
Replay of a model response is not implemented. The mechanical card is the
part that can be checked now.

## Agent-readable context and explanations

`check --emit-graph` writes a deterministic projection of the kernel and its
matching check report. `explain` answers a local graph query in JSON, either
from that saved graph or by recomputing it from a kernel:

```sh
cargo run -- check --kernel examples/structural_alias.json --emit-graph graph.json
cargo run -- explain --graph graph.json A.alias
cargo run -- explain --kernel examples/structural_alias.json argument:A.alias
```

The query id can be a typed node id or a unique bare claim, argument, or
evidence id. Type prefixes are `claim:`, `argument:`, `evidence_spec:`, and
`finding:`; unresolved references have their own typed nodes. Finding ids are
`finding:<code>:<subject>`, with a deterministic numeric suffix if the same
diagnostic occurs more than once. Look in the graph's `nodes` object for all
available ids. An unknown or ambiguous id exits 2 rather than guessing.

Nodes carry the declared claim source, statement, expression and scope,
argument rule and status, evidence request, or diagnostic. Edges distinguish
`depends_on`, `uses`, `concludes`, `requires_evidence`, `evidence_for`, and
`blocked_by`. An explanation contains:

- `focus` (the requested node) and `subject` (its entity when it is a finding);
- direct findings and findings on one-hop neighbors;
- local edges and related nodes;
- iterative `upstream` and `downstream` traces of declared requirements and
  consumers. Cycles terminate after the first visit; the subject is separate
  from the returned trace nodes.
  Reachability does not propagate invalidity or establish support: an acyclic
  alternative argument remains independent of a rejected alternative.

The graph records the canonical kernel hash and check decision. Both graph
and explanation keep `claim_supported=false` and list `claim_supported` and
`alignment_guaranteed` under `not_established`. A graph file that tries to
write `claim_supported=true` is refused by the reader. Untrusted statement
and evidence-target fields are labelled explicitly, and `source_note` tells
an agent not to interpret document text as instructions. No network, MCP
server, model call, or automatic repair is part of this layer.

Source provenance has a deliberate limit: the graph preserves authored
`source` locators, not input file line/column spans, which the loader does
not retain. A locator is authored metadata, not proof that text exists at
that location. Explain is a read-only query with exit 0 for a valid id; it
does not repeat `check`'s exit-3 verdict gate.

For a concrete rejected case, graph generation still works while `check`
returns its original exit 3:

```sh
cargo run -- check --kernel ../argumentos_construccion_v1/examples/invalid_scope.json --emit-graph graph.json
cargo run -- explain --graph graph.json finding:scope_mismatch:A.Q
```

## Hardening and stress tests

`cargo test --workspace` runs 99 tests. Beyond the unit and sealed-fixture
suites, adversarial integration tests live in `crates/*/tests/robustness.rs`,
`crates/argument-audit/tests/graph.rs`, and
`crates/argument-cli/tests/cli.rs`. They probe the contract boundaries:

- `argument-document/tests/robustness.rs` — duplicate keys (including escape-equivalent
  spellings), escape tricks (lone UTF-16 surrogates, raw control characters),
  leading-zero numbers, trailing data, empty scope keys and control characters
  in scope values, self-approval spellings in any deny-listed field name,
  repository-escaping evidence targets, shared identifier namespaces, hash
  stability and sensitivity.
- `argument-audit/tests/robustness.rs` — two- and three-claim assumption cycles,
  a derivation cycle through a distant assumption, unknown rules (rejected,
  never left pending), evidence linked to the wrong claim or the wrong scope,
  scope widening in conditional application (finite-to-countable, dropped keys),
  obligation dropping, and stack-safety on 5k–20k-claim adversarial chains
  (explicit stack, no recursion).
- `argument-card/tests/robustness.rs` — CRLF cards (the premise-coverage check must
  not silently pass), multi-assumption open claims (roundtrip regression),
  expression nesting capped at 128 frames, card atom ids held to the kernel id
  contract, scope values that would break the line format, stale pins, unclosed
  reading fences, lexicon duplicate keys.
- `argument-audit/tests/graph.rs` — deterministic typed nodes and edges,
  local explanations and deep dependency traces, explicitly represented
  unresolved references, distinct ids for identical diagnostics, cycle
  termination, a 2,000-claim chain, source trust labels, and rejection of
  graphs that claim support or contain dangling edges.
- `argument-cli/tests/cli.rs` — exit codes (`0`/`2`/`3`), `--require-verified`,
  the sealed fixture set end to end, a card through `render` → `align`, graph
  emission on a rejected document, and `explain` from a kernel or saved graph.

Fixes landed while building these tests:

- The lexicon loader now rejects duplicate JSON keys, like the kernel and
  receipt loaders already did.
- Scope entries are validated at load: no empty keys, no control characters.
  The card encoder rejects the same plus `=` and `,`, which stay legal in the
  document itself.
- `parse_open` no longer splits the `assumptions=` list on whitespace; cards
  with multiple assumptions on an open claim round-trip correctly.
- The open-claim parser and the card expression parser validate identifiers
  against the kernel contract (`^[A-Za-z][A-Za-z0-9_.:-]*$`); expressions
  nested beyond 128 levels are rejected instead of overflowing the stack.
- `mentioned()` now honors an authored gloss for the atom of a premise or
  assumption claim, as documented; it previously consulted the gloss table
  with the claim id, so the gloss path never fired.
- `align` treats a card with no argument keys but an `argument-card ` header
  (zero-argument card) as a card, not as free prose.
- The premise-coverage check works on CRLF cards; line endings no longer
  silence it.
- The assumption-graph walk and the derivation walk are iterative, so a long
  assumption chain cannot overflow the stack. `ssch::Ratio` reduces through
  i128, so `i64::MIN` normalizes instead of panicking.
- `check` indexes findings by id when computing per-claim status; the previous
  lookup was quadratic in the number of arguments.
- The duplicate-key JSON parser rejects leading zeros (`01`) with a proper
  error.
- Diagnostics name the offending entity, the cause, and the concrete next
  step (measured with `d1.2-ultra`; actionability 2.14 → 3.26 on a 0–4 scale,
  locked in by `audited_diagnostics_name_entities_and_next_steps`).

These are structural guarantees only. They do not execute code, call Lean, or
promote any claim to `claim_supported`.

## d1.2-ultra decision audit (advisory)

`progress/d1-ultra-audit-2026-11-23.md` records a full pass of Liquid AI's
hosted decision model `d1.2-ultra` over this tree's artifacts: rendered cards,
the sealed fixture verdicts, diagnostics, and two design invariants. Verdicts
never gate on this model — the deterministic checker stays authoritative, and
the audit documents where the model fails (it accepts the empirical-promotion
fixture almost surely and only weakly rejects the scope violation). It
corroborates the reading-layer invariant, both design invariants, and it drove
the diagnostic actionability fix above. Total spend: $0.000494, 33 calls.

## What is not done

Lean export, process runners, Typst audit, GLM extraction, and pickaxe are
not in this tree. `progress/STATE.json` does not mark any F0–F8 gate
complete. Green tests here do not close a corpus.

## Commands

From this directory:

```sh
cargo run -- check --kernel ../argumentos_construccion_v1/examples/valid_structure_pending.json
cargo run -- check --kernel examples/structural_alias.json
cargo run -- check --kernel examples/structural_alias.json --emit-graph graph.json
cargo run -- explain --graph graph.json A.alias
cargo run -- fixtures --dir ../argumentos_construccion_v1/examples
cargo run -- render --kernel examples/structural_alias.json --language es
cargo run -- align --kernel examples/structural_alias.json --text card.txt
cargo run -- doctor
```

Exit codes: `0` structural catalog (obligations may remain), `2` bad
contract, `3` invalid derivation or `--require-verified` with anything still
pending. `--require-verified` does not produce evidence; it only refuses to
exit 0.

`cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings`, and
`cargo test --workspace` are the checks for this tree.

## Layout

- `crates/argument-document` — strict load, duplicate-key rejection, canonical hash.
- `crates/argument-audit` — profiles, rule contracts, receipt admission, the
  deterministic context graph and explanations, and the synthetic SSCH
  rational identity (1/5, 4/5, Bayes 19/50). That identity is one finite
  fixture, not a statistical law.
- `crates/argument-card` — controlled card rendering and parsing; no model call.
- `crates/argument-cli` — `argument-audit`.
- `schemas/kernel.schema.json` — shape of v1.1. The loader is the enforcement.
- `examples/` — a non-book corpus and a named external runner.
- `crates/*/tests/` — unit and adversarial integration tests (see
  Hardening and stress tests above).

The Python scripts in the sealed package are that package's own checks.
They are not the checker, and this tree does not call them.
