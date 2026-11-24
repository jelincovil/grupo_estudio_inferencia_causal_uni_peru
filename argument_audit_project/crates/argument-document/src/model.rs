use std::collections::BTreeMap;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::canonical::kernel_sha256;
use crate::dup::reject_duplicate_keys;
use crate::error::ContractError;

pub const ID_HELP: &str =
    "IDs are stable identifiers, not a proof that two strings mean the same formula.";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SchemaVersion {
    #[serde(rename = "1.0")]
    V1_0,
    #[serde(rename = "1.1")]
    V1_1,
}

impl SchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1_0 => "1.0",
            Self::V1_1 => "1.1",
        }
    }
}

/// A compiled-in rule set. New rules are a software change, not a JSON edit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileId {
    /// Book-audit profile: the seven v1 administrative rules.
    /// It does not certify Lean, experiments, or clinical claims.
    #[serde(rename = "scientific_audit")]
    ScientificAudit,
    /// Corpus-agnostic profile: exact propositional structure only.
    #[serde(rename = "structural")]
    Structural,
}

impl ProfileId {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScientificAudit => "scientific_audit",
            Self::Structural => "structural",
        }
    }

    fn parse(value: &str) -> Result<Self, ContractError> {
        match value {
            "scientific_audit" => Ok(Self::ScientificAudit),
            "structural" => Ok(Self::Structural),
            other => Err(ContractError::Profile(other.to_string())),
        }
    }
}

/// Where a claim lives. A book is one corpus, not the kernel's universe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceLocator {
    pub system: String,
    pub key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Corpus {
    pub id: String,
    pub kind: CorpusKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CorpusKind {
    Book,
    Paper,
    Specification,
    Dataset,
    Case,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimKind {
    Formal,
    Empirical,
    Conditional,
    Factual,
}

/// Minimal expression AST. Prose in `statement` is not an expression.
/// Two atom ids are equal only when the strings are equal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "lowercase", deny_unknown_fields)]
pub enum Expression {
    Atom {
        id: String,
    },
    Implies {
        antecedent: Box<Expression>,
        consequent: Box<Expression>,
    },
    And {
        items: Vec<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Claim {
    pub id: String,
    /// Legacy locator kept so v1 book fixtures load unchanged.
    #[serde(default)]
    pub book_key: Option<String>,
    #[serde(default)]
    pub source: Option<SourceLocator>,
    pub kind: ClaimKind,
    pub statement: String,
    pub expression: Expression,
    pub scope: BTreeMap<String, String>,
    pub assumptions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rule {
    LeanAnchor,
    LeanApply,
    AndIntro,
    ModusPonens,
    EmpiricalRecord,
    ConditionalApplication,
    AliasExact,
    /// Present in the file, absent from the installed registry.
    Unsupported(String),
}

impl Rule {
    pub fn canonical(&self) -> String {
        match self {
            Self::LeanAnchor => "LEAN_ANCHOR".to_string(),
            Self::LeanApply => "LEAN_APPLY".to_string(),
            Self::AndIntro => "AND_INTRO".to_string(),
            Self::ModusPonens => "MODUS_PONENS".to_string(),
            Self::EmpiricalRecord => "EMPIRICAL_RECORD".to_string(),
            Self::ConditionalApplication => "CONDITIONAL_APPLICATION".to_string(),
            Self::AliasExact => "ALIAS_EXACT".to_string(),
            Self::Unsupported(name) => name.clone(),
        }
    }

    fn parse(value: &str) -> Self {
        match value {
            "LEAN_ANCHOR" => Self::LeanAnchor,
            "LEAN_APPLY" => Self::LeanApply,
            "AND_INTRO" => Self::AndIntro,
            "MODUS_PONENS" => Self::ModusPonens,
            "EMPIRICAL_RECORD" => Self::EmpiricalRecord,
            "CONDITIONAL_APPLICATION" => Self::ConditionalApplication,
            "ALIAS_EXACT" => Self::AliasExact,
            other => Self::Unsupported(other.to_string()),
        }
    }
}

impl Serialize for Rule {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.canonical())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Argument {
    pub id: String,
    pub conclusion: String,
    pub premises: Vec<String>,
    pub rule: String,
    pub evidence_specs: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EvidenceKind {
    Lean,
    Python,
    /// An external runner. It can be tracked. It is not a Lean proof
    /// and it is not an experimental reproduction.
    Registered,
}

impl EvidenceKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Lean => "lean",
            Self::Python => "python",
            Self::Registered => "registered",
        }
    }
}

/// `pending` and `candidate` are the only legal availability values.
/// Neither means the evidence has been validated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Availability {
    Pending,
    Candidate,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceSpec {
    pub id: String,
    pub kind: EvidenceKind,
    pub claim_id: String,
    pub runner_id: String,
    pub target: String,
    pub scope: BTreeMap<String, String>,
    pub availability: Availability,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LoadedArgument {
    pub id: String,
    pub conclusion: String,
    pub premises: Vec<String>,
    pub rule: Rule,
    pub evidence_specs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Kernel {
    pub schema_version: SchemaVersion,
    pub profile: ProfileId,
    pub corpus: Option<Corpus>,
    pub claims: Vec<Claim>,
    pub arguments: Vec<LoadedArgument>,
    pub evidence_specs: Vec<EvidenceSpec>,
}

impl Kernel {
    /// Load a kernel document.
    ///
    /// Version 1.0 is the sealed pilot exchange: no `profile` or `corpus`
    /// field, evidence kinds `lean` and `python` only, profile fixed to
    /// `scientific_audit`. Version 1.1 adds those fields and the
    /// `registered` evidence kind. Unknown fields are rejected. A status
    /// field such as `approved` is a self-approval error, not a warning.
    pub fn load(bytes: &[u8]) -> Result<Self, ContractError> {
        let text = std::str::from_utf8(bytes).map_err(|_| ContractError::Utf8)?;
        reject_duplicate_keys(text)?;
        let value: Value = serde_json::from_str(text).map_err(ContractError::from_serde)?;
        let object = value.as_object().ok_or(ContractError::NotObject)?;
        let version = object
            .get("schema_version")
            .and_then(Value::as_str)
            .ok_or_else(|| ContractError::SchemaVersion("missing".to_string()))?;
        let kernel = match version {
            "1.0" => load_v1(&value)?,
            "1.1" => load_v11(&value)?,
            other => return Err(ContractError::SchemaVersion(other.to_string())),
        };
        kernel.validate_shape()?;
        Ok(kernel)
    }

    pub fn load_str(text: &str) -> Result<Self, ContractError> {
        Self::load(text.as_bytes())
    }

    pub fn content_sha256(&self) -> Result<String, ContractError> {
        kernel_sha256(self)
    }

    fn validate_shape(&self) -> Result<(), ContractError> {
        let mut seen = Vec::new();
        for claim in &self.claims {
            remember(&mut seen, &claim.id)?;
            validate_id(&claim.id)?;
            if claim.statement.trim().is_empty() {
                return Err(ContractError::EmptyStatement(claim.id.clone()));
            }
            validate_expression(&claim.id, &claim.expression)?;
            validate_scope("claim", &claim.id, &claim.scope)?;
            validate_id_list(&claim.assumptions)?;
            reject_duplicate_list("assumption", &claim.assumptions)?;
            if let Some(source) = &claim.source {
                if source.system.trim().is_empty() {
                    return Err(ContractError::Invalid(format!(
                        "claim `{}` has an empty source.system",
                        claim.id
                    )));
                }
            }
        }
        for argument in &self.arguments {
            remember(&mut seen, &argument.id)?;
            validate_id(&argument.id)?;
            validate_id(&argument.conclusion)?;
            validate_id_list(&argument.premises)?;
            validate_id_list(&argument.evidence_specs)?;
            reject_duplicate_list("premise", &argument.premises)?;
            reject_duplicate_list("evidence spec", &argument.evidence_specs)?;
            if let Rule::Unsupported(name) = &argument.rule {
                if !is_id(name) {
                    return Err(ContractError::BadId(name.clone()));
                }
            }
        }
        for spec in &self.evidence_specs {
            remember(&mut seen, &spec.id)?;
            validate_id(&spec.id)?;
            validate_id(&spec.claim_id)?;
            validate_id(&spec.runner_id)?;
            validate_scope("evidence", &spec.id, &spec.scope)?;
            if spec.target.trim().is_empty() {
                return Err(ContractError::Invalid(format!(
                    "evidence `{}` has an empty target",
                    spec.id
                )));
            }
            if path_escapes(&spec.target) {
                return Err(ContractError::Invalid(format!(
                    "evidence `{}` target escapes the repository",
                    spec.id
                )));
            }
        }
        if let Some(corpus) = &self.corpus {
            if corpus.id.trim().is_empty() {
                return Err(ContractError::Invalid(
                    "corpus.id must not be empty".to_string(),
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawV1 {
    schema_version: String,
    claims: Vec<RawClaimV1>,
    arguments: Vec<Argument>,
    evidence_specs: Vec<RawEvidenceV1>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawClaimV1 {
    id: String,
    book_key: Option<String>,
    kind: ClaimKind,
    statement: String,
    expression: Expression,
    scope: BTreeMap<String, String>,
    assumptions: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawEvidenceV1 {
    id: String,
    kind: String,
    claim_id: String,
    runner_id: String,
    target: String,
    scope: BTreeMap<String, String>,
    availability: Availability,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawV11 {
    schema_version: String,
    #[serde(default)]
    profile: Option<String>,
    #[serde(default)]
    corpus: Option<Corpus>,
    claims: Vec<Claim>,
    arguments: Vec<Argument>,
    evidence_specs: Vec<EvidenceSpec>,
}

fn load_v1(value: &Value) -> Result<Kernel, ContractError> {
    let raw: RawV1 = from_value(value)?;
    if raw.schema_version != "1.0" {
        return Err(ContractError::SchemaVersion(raw.schema_version));
    }
    Ok(Kernel {
        schema_version: SchemaVersion::V1_0,
        profile: ProfileId::ScientificAudit,
        corpus: None,
        claims: raw
            .claims
            .into_iter()
            .map(|claim| Claim {
                source: default_source(claim.book_key.as_deref()),
                id: claim.id,
                book_key: claim.book_key,
                kind: claim.kind,
                statement: claim.statement,
                expression: claim.expression,
                scope: claim.scope,
                assumptions: claim.assumptions,
            })
            .collect(),
        arguments: load_arguments(raw.arguments),
        evidence_specs: raw
            .evidence_specs
            .into_iter()
            .map(evidence_from_v1)
            .collect::<Result<Vec<_>, _>>()?,
    })
}

fn load_v11(value: &Value) -> Result<Kernel, ContractError> {
    let raw: RawV11 = from_value(value)?;
    if raw.schema_version != "1.1" {
        return Err(ContractError::SchemaVersion(raw.schema_version));
    }
    let profile = match raw.profile {
        Some(name) => ProfileId::parse(&name)?,
        None => ProfileId::Structural,
    };
    Ok(Kernel {
        schema_version: SchemaVersion::V1_1,
        profile,
        corpus: raw.corpus,
        claims: raw
            .claims
            .into_iter()
            .map(|mut claim| {
                if claim.source.is_none() {
                    claim.source = default_source(claim.book_key.as_deref());
                }
                claim
            })
            .collect(),
        arguments: load_arguments(raw.arguments),
        evidence_specs: raw.evidence_specs,
    })
}

fn load_arguments(arguments: Vec<Argument>) -> Vec<LoadedArgument> {
    arguments
        .into_iter()
        .map(|argument| LoadedArgument {
            rule: Rule::parse(&argument.rule),
            id: argument.id,
            conclusion: argument.conclusion,
            premises: argument.premises,
            evidence_specs: argument.evidence_specs,
        })
        .collect()
}

fn evidence_from_v1(raw: RawEvidenceV1) -> Result<EvidenceSpec, ContractError> {
    let kind = match raw.kind.as_str() {
        "lean" => EvidenceKind::Lean,
        "python" => EvidenceKind::Python,
        "registered" => {
            return Err(ContractError::Invalid(
                "evidence kind `registered` requires schema_version 1.1".to_string(),
            ));
        }
        "approved" | "reproduced" | "lean_checked" => {
            return Err(ContractError::SelfApproval(format!(
                "evidence kind `{}` is not an availability and cannot certify",
                raw.kind
            )));
        }
        other => {
            return Err(ContractError::Invalid(format!(
                "unknown evidence kind `{other}` in schema 1.0"
            )));
        }
    };
    Ok(EvidenceSpec {
        id: raw.id,
        kind,
        claim_id: raw.claim_id,
        runner_id: raw.runner_id,
        target: raw.target,
        scope: raw.scope,
        availability: raw.availability,
    })
}

fn default_source(book_key: Option<&str>) -> Option<SourceLocator> {
    Some(SourceLocator {
        system: "book".to_string(),
        key: book_key.map(str::to_string),
    })
}

fn from_value<T: DeserializeOwned>(value: &Value) -> Result<T, ContractError> {
    serde_json::from_value(value.clone()).map_err(ContractError::from_serde)
}

fn remember(seen: &mut Vec<String>, id: &str) -> Result<(), ContractError> {
    if seen.iter().any(|existing| existing == id) {
        return Err(ContractError::DuplicateId(id.to_string()));
    }
    seen.push(id.to_string());
    Ok(())
}

fn validate_id_list(ids: &[String]) -> Result<(), ContractError> {
    for id in ids {
        validate_id(id)?;
    }
    Ok(())
}

fn validate_id(id: &str) -> Result<(), ContractError> {
    if is_id(id) {
        Ok(())
    } else {
        Err(ContractError::BadId(id.to_string()))
    }
}

/// Identifier contract: `^[A-Za-z][A-Za-z0-9_.:-]*$`.
///
/// Exported so downstream readers (for example the card parser) can enforce
/// the same shape instead of inventing a weaker one.
pub fn is_id(id: &str) -> bool {
    let mut chars = id.chars();
    match chars.next() {
        Some(first) if first.is_ascii_alphabetic() => {}
        _ => return false,
    }
    chars.all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.' | ':' | '-'))
}

fn reject_duplicate_list(label: &str, ids: &[String]) -> Result<(), ContractError> {
    for (index, id) in ids.iter().enumerate() {
        if ids[..index].iter().any(|earlier| earlier == id) {
            return Err(ContractError::Invalid(format!("duplicate {label} `{id}`")));
        }
    }
    Ok(())
}

/// Scope entries must survive the line-based card format: no empty keys and
/// no control characters in keys or values. `=` and `,` remain legal here;
/// only the card encoder rejects them.
fn validate_scope(
    owner: &str,
    owner_id: &str,
    scope: &std::collections::BTreeMap<String, String>,
) -> Result<(), ContractError> {
    for (key, value) in scope {
        if key.is_empty() {
            return Err(ContractError::Invalid(format!(
                "{owner} `{owner_id}` has an empty scope key"
            )));
        }
        if key.chars().any(|c| c.is_control()) || value.chars().any(|c| c.is_control()) {
            return Err(ContractError::Invalid(format!(
                "{owner} `{owner_id}` scope entry `{key}` contains a control character"
            )));
        }
    }
    Ok(())
}

fn validate_expression(claim_id: &str, expression: &Expression) -> Result<(), ContractError> {
    match expression {
        Expression::Atom { id } => validate_id(id),
        Expression::Implies {
            antecedent,
            consequent,
        } => {
            validate_expression(claim_id, antecedent)?;
            validate_expression(claim_id, consequent)
        }
        Expression::And { items } => {
            if items.len() < 2 {
                return Err(ContractError::AndArity(claim_id.to_string()));
            }
            for item in items {
                validate_expression(claim_id, item)?;
            }
            Ok(())
        }
    }
}

fn path_escapes(target: &str) -> bool {
    let path = std::path::Path::new(target);
    path.is_absolute()
        || target.contains('\\')
        || path
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_duplicate_key_and_self_approval() {
        let duplicated = r#"{"schema_version":"1.0","schema_version":"1.0"}"#;
        assert!(matches!(
            Kernel::load_str(duplicated),
            Err(ContractError::DuplicateKey { .. })
        ));
        let approved = r#"{
            "schema_version":"1.0",
            "claims":[{"id":"C.P","book_key":null,"kind":"formal","statement":"x","expression":{"op":"atom","id":"P"},"scope":{},"assumptions":[],"approved":true}],
            "arguments":[],
            "evidence_specs":[]
        }"#;
        assert!(matches!(
            Kernel::load_str(approved),
            Err(ContractError::SelfApproval(_))
        ));
    }

    #[test]
    fn v11_structural_defaults_and_rejects_unknown_profile() {
        let body = r#"{
            "schema_version":"1.1",
            "corpus":{"id":"notes","kind":"specification"},
            "claims":[{"id":"C.P","kind":"factual","statement":"A recorded note.","expression":{"op":"atom","id":"P"},"scope":{},"assumptions":[]}],
            "arguments":[],
            "evidence_specs":[]
        }"#;
        let kernel = Kernel::load_str(body).expect("v1.1 loads");
        assert_eq!(kernel.profile, ProfileId::Structural);
        assert_eq!(kernel.corpus.as_ref().map(|c| c.id.as_str()), Some("notes"));
        assert!(Kernel::load_str(
            r#"{"schema_version":"1.1","profile":"glm","claims":[],"arguments":[],"evidence_specs":[]}"#
        )
        .is_err());
    }
}
