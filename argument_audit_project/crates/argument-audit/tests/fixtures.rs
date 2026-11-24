use std::fs;
use std::path::PathBuf;

use argument_audit::{admit_receipt, check, Decision, StatusAxis};
use argument_document::{Kernel, ProfileId, Receipt, TrustMode};

fn sealed_examples() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../argumentos_construccion_v1/examples")
}

fn sealed_available() -> bool {
    sealed_examples().is_dir()
}

macro_rules! skip_sin_paquete_sellado {
    () => {
        if !sealed_available() {
            eprintln!("skipped: sealed package argumentos_construccion_v1 not present in this workspace");
            return;
        }
    };
}

fn load_example(name: &str) -> Kernel {
    let path = sealed_examples().join(name);
    let bytes = fs::read(&path).unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
    Kernel::load(&bytes).unwrap_or_else(|err| panic!("load {name}: {err}"))
}

#[test]
fn sealed_examples_match_expected_accept_reject() {
    skip_sin_paquete_sellado!();
    let dir = sealed_examples();
    let mut seen = 0;
    for entry in fs::read_dir(&dir).expect("examples dir") {
        let entry = entry.expect("entry");
        let name = entry.file_name().to_string_lossy().to_string();
        if !name.ends_with(".json") {
            continue;
        }
        seen += 1;
        let bytes = fs::read(entry.path()).expect("read");
        let expected = !name.starts_with("invalid_");
        let accepted = match Kernel::load(&bytes) {
            Ok(kernel) => check(&kernel).decision == Decision::StructurallyValid,
            Err(_) => false,
        };
        assert_eq!(accepted, expected, "{name}");
    }
    assert_eq!(seen, 11);
}

#[test]
fn pending_evidence_never_promotes_axes() {
    skip_sin_paquete_sellado!();
    let report = check(&load_example("lean_anchor_pending.json"));
    assert_eq!(report.decision, Decision::StructurallyValid);
    assert!(!report.claim_supported);
    let claim = report
        .claims
        .iter()
        .find(|claim| claim.id == "C.risk_return.finite")
        .expect("claim");
    assert_eq!(claim.formal, StatusAxis::Pending);
    assert_ne!(claim.formal, StatusAxis::LeanChecked);
    assert!(report.pending_count > 0);

    let empirical = check(&load_example("empirical_pending.json"));
    let claim = empirical
        .claims
        .iter()
        .find(|claim| claim.id == "C.ssch.risk")
        .expect("claim");
    assert_eq!(claim.empirical, StatusAxis::Pending);
    assert_ne!(claim.empirical, StatusAxis::Reproduced);
}

#[test]
fn permuting_entries_does_not_change_the_report() {
    skip_sin_paquete_sellado!();
    let kernel = load_example("valid_structure_pending.json");
    let first = serde_json::to_string(&check(&kernel)).expect("serialize");
    let mut reversed = kernel.clone();
    reversed.claims.reverse();
    reversed.arguments.reverse();
    for argument in &mut reversed.arguments {
        argument.premises.reverse();
    }
    let second = serde_json::to_string(&check(&reversed)).expect("serialize");
    assert_eq!(first, second);
    assert_eq!(
        kernel.content_sha256().expect("hash"),
        reversed.content_sha256().expect("hash")
    );
}

#[test]
fn an_acyclic_alternative_survives_a_cyclic_one() {
    skip_sin_paquete_sellado!();
    let mut kernel = load_example("valid_structure_pending.json");
    kernel.arguments.push(argument_document::LoadedArgument {
        id: "A.bad".to_string(),
        conclusion: "C.Q".to_string(),
        premises: vec!["C.Q".to_string()],
        rule: argument_document::Rule::ModusPonens,
        evidence_specs: vec![],
    });
    let report = check(&kernel);
    assert_eq!(report.decision, Decision::Rejected);
    let good = report
        .arguments
        .iter()
        .find(|item| item.id == "A.Q")
        .expect("good");
    let bad = report
        .arguments
        .iter()
        .find(|item| item.id == "A.bad")
        .expect("bad");
    assert_eq!(good.structure, StatusAxis::Valid);
    assert_eq!(bad.structure, StatusAxis::Invalid);
    assert!(bad.diagnostic_codes.iter().any(|code| code == "cycle"));
    let claim = report
        .claims
        .iter()
        .find(|item| item.id == "C.Q")
        .expect("claim");
    assert_eq!(claim.structure, StatusAxis::Valid);
}

#[test]
fn structural_profile_refuses_a_lean_rule() {
    skip_sin_paquete_sellado!();
    let mut kernel = load_example("lean_anchor_pending.json");
    kernel.profile = ProfileId::Structural;
    let report = check(&kernel);
    assert_eq!(report.decision, Decision::Rejected);
    assert!(report
        .diagnostics
        .iter()
        .any(|item| item.code == "unsupported_rule"));
    assert!(!report.claim_supported);
}

#[test]
fn conditional_application_keeps_obligations_and_rejects_widening() {
    let kept = Kernel::load_str(
        r#"{
          "schema_version":"1.1",
          "profile":"scientific_audit",
          "claims":[
            {"id":"C.H","book_key":null,"kind":"formal","statement":"H","expression":{"op":"atom","id":"H"},"scope":{"domain":"finite"},"assumptions":[]},
            {"id":"C.S","book_key":null,"kind":"conditional","statement":"H in this scenario","expression":{"op":"atom","id":"S"},"scope":{"domain":"finite","scenario":"lab"},"assumptions":["C.H"]}
          ],
          "arguments":[{"id":"A.S","conclusion":"C.S","premises":["C.H"],"rule":"CONDITIONAL_APPLICATION","evidence_specs":[]}],
          "evidence_specs":[]
        }"#,
    )
    .expect("kept");
    let report = check(&kept);
    assert_eq!(report.decision, Decision::StructurallyValid, "{report:?}");
    assert!(report.arguments[0]
        .obligations
        .iter()
        .any(|item| item.contains("SCOPE_NOT_JUSTIFIED")));

    let widened = Kernel::load_str(
        r#"{
          "schema_version":"1.1",
          "profile":"scientific_audit",
          "claims":[
            {"id":"C.H","book_key":null,"kind":"formal","statement":"H","expression":{"op":"atom","id":"H"},"scope":{"domain":"finite"},"assumptions":[]},
            {"id":"C.S","book_key":null,"kind":"conditional","statement":"widened","expression":{"op":"atom","id":"S"},"scope":{"domain":"countable"},"assumptions":["C.H"]}
          ],
          "arguments":[{"id":"A.S","conclusion":"C.S","premises":["C.H"],"rule":"CONDITIONAL_APPLICATION","evidence_specs":[]}],
          "evidence_specs":[]
        }"#,
    )
    .expect("widened");
    let report = check(&widened);
    assert_eq!(report.decision, Decision::Rejected);
    assert!(report
        .diagnostics
        .iter()
        .any(|item| item.code == "scope_widened"));
}

#[test]
fn a_registered_runner_does_not_satisfy_an_empirical_record() {
    let kernel = Kernel::load_str(
        r#"{
          "schema_version":"1.1",
          "profile":"scientific_audit",
          "claims":[{"id":"C.run","kind":"empirical","statement":"named only","expression":{"op":"atom","id":"run_named"},"scope":{"experiment":"batch-7"},"assumptions":[]}],
          "arguments":[{"id":"A.run","conclusion":"C.run","premises":[],"rule":"EMPIRICAL_RECORD","evidence_specs":["E.run"]}],
          "evidence_specs":[{"id":"E.run","kind":"registered","claim_id":"C.run","runner_id":"lab.batch","target":"batch-7","scope":{"experiment":"batch-7"},"availability":"candidate"}]
        }"#,
    )
    .expect("loads");
    let report = check(&kernel);
    assert_eq!(report.decision, Decision::Rejected);
    assert!(report
        .diagnostics
        .iter()
        .any(|item| item.code == "python_missing"));
    assert!(!report.claim_supported);
}

#[test]
fn receipt_cannot_support_a_claim() {
    skip_sin_paquete_sellado!();
    let kernel = load_example("empirical_pending.json");
    let spec = &kernel.evidence_specs[0];
    let hash = "ab".repeat(32);
    let body = format!(
        r#"{{
          "schema_version":"1.0",
          "id":"R.ssch",
          "spec_id":"E.ssch",
          "spec_hash":"{hash}",
          "source_commit":"local",
          "environment_hash":"{hash}",
          "checker_version":"0.1.0",
          "command_argv":["argument-audit","check"],
          "working_directory":".",
          "input_hashes":{{}},
          "started_at":"2026-11-04T00:00:00Z",
          "finished_at":"2026-11-04T00:00:01Z",
          "exit_status":0,
          "termination":"completed",
          "stdout_sha256":"{hash}",
          "stderr_sha256":"{hash}",
          "artifacts":[],
          "execution_passed":true,
          "scientific_criterion_met":true,
          "observed_checks":[],
          "trust_mode":"locally_reproduced"
        }}"#
    );
    let receipt = Receipt::load(body.as_bytes()).expect("receipt");
    assert_eq!(receipt.trust_mode, TrustMode::LocallyReproduced);
    let admission = admit_receipt(&receipt, spec, &hash);
    assert!(!admission.claim_supported);
    assert_eq!(admission.reason, "replay_not_witnessed");
    assert!(admission.execution_admitted);
    assert_eq!(admission.scientific_criterion_met, Some(true));
}
