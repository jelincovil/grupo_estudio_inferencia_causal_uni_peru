use std::collections::{BTreeMap, BTreeSet, HashSet};

use argument_document::{
    Availability, Claim, ClaimKind, EvidenceKind, EvidenceSpec, Expression, Kernel, LoadedArgument,
    ProfileId, Rule,
};

use crate::profile::{allows, evidence_is_computational, evidence_is_formal_anchor};
use crate::report::{
    ArgumentFinding, CheckReport, ClaimFinding, Decision, Diagnostic, Severity, StatusAxis,
};
use crate::{CHECKER_VERSION, LIMITS};

pub fn check(kernel: &Kernel) -> CheckReport {
    let claims = index_claims(kernel);
    let evidence = index_evidence(kernel);
    let mut diagnostics = Vec::new();

    let assumption_cycles = assumption_cycle_claims(kernel);
    for claim_id in &assumption_cycles {
        diagnostics.push(error(
            "cycle",
            claim_id,
            &format!(
                "assumption obligations form a cycle through `{claim_id}`; break the mutual assumption or discharge it by derivation"
            ),
        ));
    }

    for spec in &kernel.evidence_specs {
        match claims.get(&spec.claim_id) {
            None => diagnostics.push(error(
                "evidence_target",
                &spec.id,
                &format!(
                    "evidence target `{}` is not a claim in this document; point spec `{}` at an existing claim id",
                    spec.claim_id, spec.id
                ),
            )),
            Some(claim) if claim.scope != spec.scope => diagnostics.push(error(
                "evidence_scope",
                &spec.id,
                &format!(
                    "evidence `{}` scope differs from claim `{}` scope; align the evidence scope or the claim's scope",
                    spec.id, spec.claim_id
                ),
            )),
            Some(_) => {}
        }
        if matches!(
            spec.availability,
            Availability::Pending | Availability::Candidate
        ) {
            // Availability is a request, not a verdict. Recorded so it cannot
            // be reread later as support.
        }
    }

    for claim in &kernel.claims {
        for assumption in &claim.assumptions {
            if !claims.contains_key(assumption) {
                diagnostics.push(error(
                    "missing_assumption",
                    &claim.id,
                    &format!("assumption `{assumption}` does not resolve to a claim in this document; add the claim or fix the assumption id"),
                ));
            }
        }
    }

    let mut findings = Vec::new();
    for argument in &kernel.arguments {
        findings.push(judge_argument(
            kernel.profile,
            argument,
            &claims,
            &evidence,
            &assumption_cycles,
            &mut diagnostics,
        ));
    }

    let claim_findings = claim_statuses(kernel, &findings, &assumption_cycles);
    sort_diagnostics(&mut diagnostics);
    findings.sort_by(|left, right| left.id.cmp(&right.id));

    let decision = if diagnostics
        .iter()
        .any(|item| item.severity == Severity::Error)
    {
        Decision::Rejected
    } else if findings
        .iter()
        .any(|item| item.structure == StatusAxis::Valid)
    {
        Decision::StructurallyValid
    } else {
        Decision::Pending
    };
    let pending_count = pending_obligations(&findings, &claim_findings);

    CheckReport {
        checker_version: CHECKER_VERSION.to_string(),
        profile: kernel.profile.as_str().to_string(),
        decision,
        claim_supported: false,
        pending_count,
        diagnostics,
        arguments: findings,
        claims: claim_findings,
        limits: LIMITS.to_string(),
    }
}

fn judge_argument(
    profile: ProfileId,
    argument: &LoadedArgument,
    claims: &BTreeMap<String, &Claim>,
    evidence: &BTreeMap<String, &EvidenceSpec>,
    assumption_cycles: &BTreeSet<String>,
    diagnostics: &mut Vec<Diagnostic>,
) -> ArgumentFinding {
    let mut codes = Vec::new();
    let mut obligations = Vec::new();
    let push =
        |diagnostics: &mut Vec<Diagnostic>, codes: &mut Vec<String>, code: &str, detail: &str| {
            diagnostics.push(error(code, &argument.id, detail));
            codes.push(code.to_string());
        };

    if !allows(profile, &argument.rule) {
        push(
            diagnostics,
            &mut codes,
            "unsupported_rule",
            &format!(
                "rule `{}` is not in profile `{}`; it stays unsupported and is not pending approval",
                argument.rule.canonical(),
                profile.as_str()
            ),
        );
    }

    let conclusion = match claims.get(&argument.conclusion) {
        Some(claim) => Some(*claim),
        None => {
            push(
                diagnostics,
                &mut codes,
                "missing_conclusion",
                &format!(
                    "conclusion `{}` does not resolve to a claim in this document; add the claim or fix the conclusion reference",
                    argument.conclusion
                ),
            );
            None
        }
    };

    let mut premises = Vec::new();
    for premise_id in &argument.premises {
        match claims.get(premise_id) {
            Some(claim) => premises.push(*claim),
            None => push(
                diagnostics,
                &mut codes,
                "missing_premise",
                &format!("premise `{premise_id}` does not resolve to a claim in this document; add the claim or fix the premise id"),
            ),
        }
    }

    let mut linked = Vec::new();
    for spec_id in &argument.evidence_specs {
        match evidence.get(spec_id) {
            Some(spec) if spec.claim_id == argument.conclusion => linked.push(*spec),
            Some(spec) => push(
                diagnostics,
                &mut codes,
                "evidence_link",
                &format!("evidence `{spec_id}` supports claim `{}`, not conclusion `{}`; link evidence only to its own conclusion", spec.claim_id, argument.conclusion),
            ),
            None => push(
                diagnostics,
                &mut codes,
                "evidence_link",
                &format!("evidence `{spec_id}` does not resolve to an evidence spec in this document; add the spec or fix the reference"),
            ),
        }
    }

    if let Some(conclusion) = conclusion {
        if premises.len() == argument.premises.len()
            && premises
                .iter()
                .any(|premise| premise.scope != conclusion.scope)
            && !matches!(argument.rule, Rule::ConditionalApplication)
        {
            push(
                diagnostics,
                &mut codes,
                "scope_mismatch",
                &format!(
                    "premise scope differs from conclusion `{}`; align the premise scope with the conclusion, or express this as a conditional application with retained obligations",
                    conclusion.id
                ),
            );
        }
        if derivation_cycles(argument, conclusion, claims)
            || assumption_cycles.contains(&conclusion.id)
        {
            push(
                diagnostics,
                &mut codes,
                "cycle",
                &format!(
                    "derivation cycle: the premise chain reaches conclusion `{}`; break the loop or rely on an acyclic alternative argument for this claim",
                    conclusion.id
                ),
            );
        }
        if codes.is_empty() {
            apply_rule(
                argument,
                conclusion,
                &premises,
                &linked,
                diagnostics,
                &mut codes,
                &mut obligations,
            );
        }
    }

    if codes.is_empty() {
        obligations.push(
            "NO_REPLAY: availability pending or candidate is not admitted evidence".to_string(),
        );
    }

    let structure = if codes.is_empty() {
        StatusAxis::Valid
    } else {
        StatusAxis::Invalid
    };
    codes.sort();
    codes.dedup();
    obligations.sort();
    obligations.dedup();
    let mixed_dossier = obligations
        .iter()
        .any(|item| item.starts_with("MIXED_DOSSIER"));
    ArgumentFinding {
        id: argument.id.clone(),
        rule: argument.rule.canonical(),
        structure,
        obligations,
        mixed_dossier,
        diagnostic_codes: codes,
    }
}

fn apply_rule(
    argument: &LoadedArgument,
    conclusion: &Claim,
    premises: &[&Claim],
    linked: &[&EvidenceSpec],
    diagnostics: &mut Vec<Diagnostic>,
    codes: &mut Vec<String>,
    obligations: &mut Vec<String>,
) {
    match &argument.rule {
        Rule::ModusPonens => {
            if premises.len() != 2 {
                fail(
                    diagnostics,
                    codes,
                    &argument.id,
                    "mp_arity",
                    "modus ponens requires exactly two premises",
                );
                return;
            }
            let expressions = [
                premises[0].expression.clone(),
                premises[1].expression.clone(),
            ];
            let matches = (0..2).any(|index| {
                implication_matches(
                    &expressions[index],
                    &expressions[1 - index],
                    &conclusion.expression,
                )
            });
            if !matches {
                fail(
                    diagnostics,
                    codes,
                    &argument.id,
                    "mp_conclusion",
                    "modus ponens requires exact AST: P and Implies(P, Q) yield Q; check the antecedent matches premise 1 and the consequent matches the conclusion",
                );
            } else {
                obligations.push("NO_FORMAL_PROOF: exact AST is not a Lean proof".to_string());
            }
        }
        Rule::AndIntro => {
            if premises.len() < 2 {
                fail(
                    diagnostics,
                    codes,
                    &argument.id,
                    "and_arity",
                    "and-introduction requires at least two premises",
                );
                return;
            }
            let expected = Expression::And {
                items: premises
                    .iter()
                    .map(|premise| premise.expression.clone())
                    .collect(),
            };
            if conclusion.expression != expected {
                fail(
                    diagnostics,
                    codes,
                    &argument.id,
                    "and_conclusion",
                    "conclusion expression is not exactly And of the premise expressions in premise order; rebuild the conjunction from the premises in their given order",
                );
            }
            if premises
                .iter()
                .any(|premise| premise.kind != premises[0].kind)
            {
                obligations.push(
                    "MIXED_DOSSIER: formal and non-formal inputs do not become a Lean theorem"
                        .to_string(),
                );
            }
        }
        Rule::AliasExact => {
            if premises.len() != 1 {
                fail(
                    diagnostics,
                    codes,
                    &argument.id,
                    "alias_arity",
                    "alias-exact requires one premise",
                );
                return;
            }
            if premises[0].expression != conclusion.expression
                || premises[0].scope != conclusion.scope
            {
                fail(
                    diagnostics,
                    codes,
                    &argument.id,
                    "alias_mismatch",
                    "alias-exact requires the same expression and the same scope; prose paraphrase is editorial",
                );
            } else {
                obligations.push(
                    "SEMANTIC_MAPPING_UNRESOLVED: natural-language sameness is not this rule"
                        .to_string(),
                );
            }
        }
        Rule::LeanAnchor => {
            if conclusion.kind != ClaimKind::Formal {
                fail(
                    diagnostics,
                    codes,
                    &argument.id,
                    "lean_kind",
                    "a Lean anchor requires a formal claim",
                );
            }
            if !linked
                .iter()
                .any(|spec| evidence_is_formal_anchor(spec.kind))
            {
                fail(
                    diagnostics,
                    codes,
                    &argument.id,
                    "lean_missing",
                    "a Lean anchor requires a lean evidence spec",
                );
            }
            obligations.push("NO_FORMAL_PROOF: no Lean export was admitted".to_string());
        }
        Rule::LeanApply => {
            if conclusion.kind != ClaimKind::Formal {
                fail(
                    diagnostics,
                    codes,
                    &argument.id,
                    "lean_kind",
                    "Lean apply requires a formal claim",
                );
            }
            if premises.is_empty() {
                fail(
                    diagnostics,
                    codes,
                    &argument.id,
                    "lean_apply_premises",
                    "Lean apply needs the instantiated premises",
                );
            }
            if !linked
                .iter()
                .any(|spec| evidence_is_formal_anchor(spec.kind))
            {
                fail(
                    diagnostics,
                    codes,
                    &argument.id,
                    "lean_missing",
                    "Lean apply requires a lean evidence spec for this conclusion",
                );
            }
            obligations.push(
                "NO_FORMAL_PROOF: a linked spec is not a wrapper result; compiling another theorem does not count"
                    .to_string(),
            );
        }
        Rule::EmpiricalRecord => {
            if conclusion.kind != ClaimKind::Empirical {
                fail(
                    diagnostics,
                    codes,
                    &argument.id,
                    "empirical_promotion",
                    "an empirical record cannot promote a non-empirical claim",
                );
            }
            if !linked
                .iter()
                .any(|spec| evidence_is_computational(spec.kind))
            {
                fail(
                    diagnostics,
                    codes,
                    &argument.id,
                    "python_missing",
                    "an empirical record requires a computational evidence spec",
                );
            }
            if linked
                .iter()
                .any(|spec| spec.kind == EvidenceKind::Registered)
            {
                obligations.push(
                    "REGISTERED_RUNNER: a registered runner is not an experimental reproduction"
                        .to_string(),
                );
            }
            obligations.push(
                "EMPIRICAL_NOT_REPRODUCED: the spec records a planned run, not a result"
                    .to_string(),
            );
        }
        Rule::ConditionalApplication => {
            if conclusion.kind != ClaimKind::Conditional {
                fail(
                    diagnostics,
                    codes,
                    &argument.id,
                    "conditional_kind",
                    "conditional application must conclude a conditional claim",
                );
            }
            if premises.is_empty() {
                fail(
                    diagnostics,
                    codes,
                    &argument.id,
                    "conditional_premises",
                    "conditional application needs the theorem and the scenario",
                );
            }
            let mut required = BTreeSet::new();
            for premise in premises {
                required.insert(premise.id.clone());
                required.extend(premise.assumptions.iter().cloned());
                if scope_widens(&premise.scope, &conclusion.scope) {
                    fail(
                        diagnostics,
                        codes,
                        &argument.id,
                        "scope_widened",
                        "a scenario change cannot widen finite to countable or drop a scope key; keep the premise's scope keys and values, or add a new claim",
                    );
                }
            }
            let have: BTreeSet<String> = conclusion.assumptions.iter().cloned().collect();
            if !required.is_subset(&have) {
                fail(
                    diagnostics,
                    codes,
                    &argument.id,
                    "obligation_dropped",
                    "conditional application must retain every premise and assumption as an obligation; restore the missing premise or assumption ids in the conclusion's assumptions",
                );
            }
            obligations.push(
                "SCOPE_NOT_JUSTIFIED: a simulation does not by itself discharge identification or population independence"
                    .to_string(),
            );
        }
        Rule::Unsupported(_) => {}
    }
}

fn implication_matches(
    candidate: &Expression,
    other: &Expression,
    conclusion: &Expression,
) -> bool {
    match candidate {
        Expression::Implies {
            antecedent,
            consequent,
        } => antecedent.as_ref() == other && consequent.as_ref() == conclusion,
        _ => false,
    }
}

fn scope_widens(premise: &BTreeMap<String, String>, conclusion: &BTreeMap<String, String>) -> bool {
    const WIDENING: &[(&str, &str)] = &[
        ("finite", "countable"),
        ("finite", "infinite"),
        ("countable", "infinite"),
    ];
    for key in ["domain", "observations"] {
        let Some(from) = premise.get(key) else {
            continue;
        };
        match conclusion.get(key) {
            None => return true,
            Some(to) if WIDENING.contains(&(from.as_str(), to.as_str())) => return true,
            Some(_) => {}
        }
    }
    false
}

fn derivation_cycles(
    argument: &LoadedArgument,
    conclusion: &Claim,
    claims: &BTreeMap<String, &Claim>,
) -> bool {
    let mut visiting = HashSet::new();
    visiting.insert(conclusion.id.clone());
    let mut done = HashSet::new();
    argument
        .premises
        .iter()
        .chain(conclusion.assumptions.iter())
        .any(|next| {
            !done.contains(next) && walk_assumptions(next, &mut visiting, &mut done, claims)
        })
}

enum WalkFrame {
    Enter(String),
    Exit(String),
}

/// Iterative DFS from `id`; true when the walk reaches a node in `visiting`.
/// An explicit stack keeps adversarial assumption chains off the call stack.
fn walk_assumptions(
    id: &str,
    visiting: &mut HashSet<String>,
    done: &mut HashSet<String>,
    claims: &BTreeMap<String, &Claim>,
) -> bool {
    let mut stack = vec![WalkFrame::Enter(id.to_string())];
    while let Some(frame) = stack.pop() {
        match frame {
            WalkFrame::Enter(current) => {
                if visiting.contains(&current) {
                    return true;
                }
                if done.contains(&current) {
                    continue;
                }
                visiting.insert(current.clone());
                stack.push(WalkFrame::Exit(current.clone()));
                if let Some(claim) = claims.get(&current) {
                    for next in &claim.assumptions {
                        stack.push(WalkFrame::Enter(next.clone()));
                    }
                }
            }
            WalkFrame::Exit(current) => {
                visiting.remove(&current);
                done.insert(current);
            }
        }
    }
    false
}

/// Iterative deep-first search over the assumption graph. Explicit stack:
/// an adversarial chain of assumptions must not overflow the call stack.
fn assumption_cycle_claims(kernel: &Kernel) -> BTreeSet<String> {
    let claims = index_claims(kernel);
    let mut cyclic = BTreeSet::new();
    let mut seen: HashSet<&str> = HashSet::new();
    for root in kernel.claims.iter().map(|claim| claim.id.as_str()) {
        if !seen.insert(root) {
            continue;
        }
        let mut stack: Vec<(&str, usize)> = vec![(root, 0)];
        let mut in_stack: HashSet<&str> = HashSet::from([root]);
        while !stack.is_empty() {
            let top = stack.len() - 1;
            let (id, index) = stack[top];
            let Some(claim) = claims.get(id) else {
                in_stack.remove(id);
                stack.pop();
                continue;
            };
            let Some(next) = claim.assumptions.get(index) else {
                in_stack.remove(id);
                stack.pop();
                continue;
            };
            stack[top].1 += 1;
            let next = next.as_str();
            if in_stack.contains(next) {
                if let Some(start) = stack.iter().position(|(item, _)| *item == next) {
                    for (item, _) in &stack[start..] {
                        cyclic.insert((*item).to_string());
                    }
                }
            } else if claims.contains_key(next) && seen.insert(next) {
                in_stack.insert(next);
                stack.push((next, 0));
            }
        }
    }
    cyclic
}

fn claim_statuses(
    kernel: &Kernel,
    findings: &[ArgumentFinding],
    assumption_cycles: &BTreeSet<String>,
) -> Vec<ClaimFinding> {
    let findings_by_id: std::collections::HashMap<&str, usize> = findings
        .iter()
        .enumerate()
        .map(|(index, finding)| (finding.id.as_str(), index))
        .collect();
    let mut statuses = Vec::new();
    for claim in &kernel.claims {
        let concluding: Vec<&ArgumentFinding> = kernel
            .arguments
            .iter()
            .filter(|argument| argument.conclusion == claim.id)
            .filter_map(|argument| {
                findings_by_id
                    .get(argument.id.as_str())
                    .map(|&index| &findings[index])
            })
            .collect();
        let structure = if assumption_cycles.contains(&claim.id) {
            StatusAxis::Invalid
        } else if concluding
            .iter()
            .any(|finding| finding.structure == StatusAxis::Valid)
        {
            StatusAxis::Valid
        } else if !concluding.is_empty() {
            StatusAxis::Invalid
        } else {
            StatusAxis::Pending
        };
        let (formal, empirical) = axes_for(claim, &concluding);
        statuses.push(ClaimFinding {
            id: claim.id.clone(),
            structure,
            formal,
            empirical,
            editorial: StatusAxis::Pending,
            argumentation: StatusAxis::NotRun,
        });
    }
    statuses.sort_by(|left, right| left.id.cmp(&right.id));
    statuses
}

fn axes_for(claim: &Claim, _concluding: &[&ArgumentFinding]) -> (StatusAxis, StatusAxis) {
    // A valid structure does not discharge either axis. Backing stays pending
    // until a future replay witness, which this function does not accept.
    match claim.kind {
        ClaimKind::Formal => (StatusAxis::Pending, StatusAxis::NotApplicable),
        ClaimKind::Empirical => (StatusAxis::NotApplicable, StatusAxis::Pending),
        ClaimKind::Conditional => (StatusAxis::Conditional, StatusAxis::Conditional),
        ClaimKind::Factual => (StatusAxis::NotApplicable, StatusAxis::NotApplicable),
    }
}

fn pending_obligations(findings: &[ArgumentFinding], claims: &[ClaimFinding]) -> u32 {
    let from_arguments = findings
        .iter()
        .filter(|finding| finding.structure == StatusAxis::Valid)
        .map(|finding| finding.obligations.len())
        .sum::<usize>();
    let from_claims = claims
        .iter()
        .filter(|claim| {
            matches!(
                claim.formal,
                StatusAxis::Pending
                    | StatusAxis::Conditional
                    | StatusAxis::Stale
                    | StatusAxis::Failed
            ) || matches!(
                claim.empirical,
                StatusAxis::Pending
                    | StatusAxis::Conditional
                    | StatusAxis::Stale
                    | StatusAxis::Failed
            )
        })
        .count();
    u32::try_from(from_arguments + from_claims).unwrap_or(u32::MAX)
}

fn index_claims(kernel: &Kernel) -> BTreeMap<String, &Claim> {
    kernel
        .claims
        .iter()
        .map(|claim| (claim.id.clone(), claim))
        .collect()
}

fn index_evidence(kernel: &Kernel) -> BTreeMap<String, &EvidenceSpec> {
    kernel
        .evidence_specs
        .iter()
        .map(|spec| (spec.id.clone(), spec))
        .collect()
}

fn fail(
    diagnostics: &mut Vec<Diagnostic>,
    codes: &mut Vec<String>,
    subject: &str,
    code: &str,
    detail: &str,
) {
    diagnostics.push(error(code, subject, detail));
    codes.push(code.to_string());
}

fn error(code: &str, subject: &str, detail: &str) -> Diagnostic {
    Diagnostic {
        code: code.to_string(),
        severity: Severity::Error,
        subject: subject.to_string(),
        detail: detail.to_string(),
    }
}

fn sort_diagnostics(diagnostics: &mut [Diagnostic]) {
    diagnostics.sort_by(|left, right| {
        (&left.code, &left.subject, &left.detail).cmp(&(&right.code, &right.subject, &right.detail))
    });
}
