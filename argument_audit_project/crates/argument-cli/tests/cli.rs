//! End-to-end CLI contract tests: exit codes and the never-a-certificate rule.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_argument-audit"))
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn sealed_examples() -> PathBuf {
    workspace_root().join("../argumentos_construccion_v1/examples")
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

fn temp_path(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("argument-audit-cli-test-{}", std::process::id()));
    fs::create_dir_all(&dir).expect("temp dir");
    dir.join(name)
}

#[test]
fn doctor_exits_zero_and_prints_no_secrets() {
    let output = binary().arg("doctor").output().expect("run");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("argument-audit"));
    assert!(stdout.contains("limits:"));
    assert!(!stdout.contains("claim_supported: true"));
}

#[test]
fn a_missing_kernel_exits_two() {
    let output = binary()
        .args(["check", "--kernel", "/definitely/not/here.json"])
        .output()
        .expect("run");
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn a_rejected_kernel_exits_three_and_an_accepted_one_exits_zero() {
    let invalid = temp_path("invalid-kernel.json");
    fs::write(
        &invalid,
        r#"{"schema_version":"1.1","profile":"structural","claims":[{"id":"C.a","kind":"factual","statement":"S.","expression":{"op":"atom","id":"A"},"scope":{"domain":"notes"},"assumptions":[]}],"arguments":[{"id":"A.a","conclusion":"C.a","premises":["C.missing"],"rule":"ALIAS_EXACT","evidence_specs":[]}],"evidence_specs":[]}"#,
    )
    .expect("write");
    let output = binary()
        .args(["check", "--kernel", invalid.to_str().expect("utf8")])
        .output()
        .expect("run");
    assert_eq!(output.status.code(), Some(3));

    let valid = workspace_root().join("examples/structural_alias.json");
    let output = binary()
        .args(["check", "--kernel", valid.to_str().expect("utf8")])
        .output()
        .expect("run");
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn require_verified_refuses_an_exit_zero_when_obligations_remain() {
    skip_sin_paquete_sellado!();
    let pending = sealed_examples().join("valid_structure_pending.json");
    let output = binary()
        .args([
            "check",
            "--kernel",
            pending.to_str().expect("utf8"),
            "--require-verified",
        ])
        .output()
        .expect("run");
    assert_eq!(output.status.code(), Some(3));
}

#[test]
fn the_sealed_fixture_set_passes_end_to_end() {
    skip_sin_paquete_sellado!();
    let dir = sealed_examples();
    let output = binary()
        .args(["fixtures", "--dir", dir.to_str().expect("utf8")])
        .output()
        .expect("run");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("\"package_validation\": \"PASS\""));
}

#[test]
fn an_unknown_reading_language_exits_two() {
    let valid = workspace_root().join("examples/structural_alias.json");
    let output = binary()
        .args([
            "render",
            "--kernel",
            valid.to_str().expect("utf8"),
            "--language",
            "xx",
        ])
        .output()
        .expect("run");
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn a_card_round_trips_through_render_and_align() {
    let valid = workspace_root().join("examples/structural_alias.json");
    let card = temp_path("alias.card");
    let rendered = binary()
        .args([
            "render",
            "--kernel",
            valid.to_str().expect("utf8"),
            "--language",
            "es",
            "--out",
            card.to_str().expect("utf8"),
        ])
        .output()
        .expect("run");
    assert!(rendered.status.success());

    let aligned = binary()
        .args([
            "align",
            "--kernel",
            valid.to_str().expect("utf8"),
            "--text",
            card.to_str().expect("utf8"),
            "--language",
            "es",
        ])
        .output()
        .expect("run");
    assert_eq!(aligned.status.code(), Some(0));
    let stdout = String::from_utf8(aligned.stdout).expect("utf8");
    assert!(
        stdout.contains("\"outcome\": \"no_blocking_problem\""),
        "{stdout}"
    );
    assert!(stdout.contains("\"claim_supported\": false"), "{stdout}");
}

#[test]
fn graph_emission_preserves_the_verdict_and_explain_is_read_only() {
    skip_sin_paquete_sellado!();
    let invalid = sealed_examples().join("invalid_scope.json");
    let graph_path = temp_path("context-graph.json");
    let checked = binary()
        .args([
            "check",
            "--kernel",
            invalid.to_str().expect("utf8"),
            "--emit-graph",
            graph_path.to_str().expect("utf8"),
        ])
        .output()
        .expect("run");
    assert_eq!(checked.status.code(), Some(3));
    let graph: serde_json::Value =
        serde_json::from_slice(&fs::read(&graph_path).expect("graph")).expect("json");
    assert_eq!(graph["claim_supported"], false);
    assert_eq!(graph["decision"], "rejected");
    assert!(graph["source_note"]
        .as_str()
        .expect("source policy")
        .contains("line/column spans are not retained"));
    let finding = graph["nodes"]
        .as_object()
        .expect("nodes")
        .iter()
        .find(|(_, node)| node["code"] == "scope_mismatch")
        .map(|(id, _)| id.clone())
        .expect("scope finding");

    let explained = binary()
        .args(["explain", "--graph", graph_path.to_str().expect("utf8")])
        .arg(finding)
        .output()
        .expect("run");
    assert_eq!(explained.status.code(), Some(0));
    let explanation: serde_json::Value =
        serde_json::from_slice(&explained.stdout).expect("explanation json");
    assert_eq!(explanation["subject"]["entity_id"], "A.Q");
    assert_eq!(explanation["direct_findings"][0]["code"], "scope_mismatch");
    assert_eq!(explanation["claim_supported"], false);
}

#[test]
fn explain_accepts_a_kernel_and_a_unique_bare_entity_id() {
    let kernel = workspace_root().join("examples/structural_alias.json");
    let output = binary()
        .args([
            "explain",
            "--kernel",
            kernel.to_str().expect("utf8"),
            "C.observed",
        ])
        .output()
        .expect("run");
    assert!(output.status.success());
    let explanation: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("explanation json");
    assert_eq!(explanation["subject"]["kind"], "claim");
    assert_eq!(explanation["subject"]["source"]["system"], "notes");
    assert_eq!(
        explanation["downstream"]["edges"][0]["from"],
        "argument:A.alias"
    );
}

#[test]
fn explain_an_unknown_or_ambiguous_id_fails_without_a_verdict_change() {
    let kernel = workspace_root().join("examples/structural_alias.json");
    let unknown = binary()
        .args([
            "explain",
            "--kernel",
            kernel.to_str().expect("utf8"),
            "claim:C.unknown",
        ])
        .output()
        .expect("run");
    assert_eq!(unknown.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&unknown.stderr).contains("unknown graph id"));
}

#[test]
fn align_on_a_card_with_an_extra_argument_exits_three() {
    let valid = workspace_root().join("examples/structural_alias.json");
    let card = temp_path("alias-extra.card");
    let rendered = binary()
        .args([
            "render",
            "--kernel",
            valid.to_str().expect("utf8"),
            "--out",
            card.to_str().expect("utf8"),
        ])
        .output()
        .expect("run");
    assert!(rendered.status.success());
    let extra = "argument: A.extra\nconclusion: C.ghost\nkind: factual\nrule: ALIAS_EXACT\nexpression: atom:GHOST\npremises: -\nevidence: -\nscope: domain=notes\nassumptions: -\nreading:\n<<<\nNot in the kernel.\n>>>\n";
    let mut body = fs::read_to_string(&card).expect("read");
    body.push_str(extra);
    fs::write(&card, body).expect("write");

    let aligned = binary()
        .args([
            "align",
            "--kernel",
            valid.to_str().expect("utf8"),
            "--text",
            card.to_str().expect("utf8"),
        ])
        .output()
        .expect("run");
    assert_eq!(aligned.status.code(), Some(3));
    let stdout = String::from_utf8(aligned.stdout).expect("utf8");
    assert!(
        stdout.contains("\"outcome\": \"blocking_problem\""),
        "{stdout}"
    );
}
