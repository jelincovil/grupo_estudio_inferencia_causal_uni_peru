use thiserror::Error;

/// A document failed before any rule ran.
///
/// These are input-contract failures (exit 2). They are not mathematical
/// verdicts and they are not evidence failures.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ContractError {
    #[error("document is not valid UTF-8")]
    Utf8,
    #[error("JSON is not an object")]
    NotObject,
    #[error("JSON parse failed: {0}")]
    Json(String),
    #[error("duplicate JSON key `{key}` at {path}")]
    DuplicateKey { path: String, key: String },
    #[error("unsupported schema_version `{0}` (accepted: 1.0, 1.1)")]
    SchemaVersion(String),
    #[error("unknown profile `{0}` (known: scientific_audit, structural)")]
    Profile(String),
    #[error("duplicate identifier `{0}` across claims, arguments, and evidence")]
    DuplicateId(String),
    #[error("identifier `{0}` does not match ^[A-Za-z][A-Za-z0-9_.:-]*$")]
    BadId(String),
    #[error("claim `{0}` has an empty statement")]
    EmptyStatement(String),
    #[error("claim `{0}` uses `and` with fewer than two items")]
    AndArity(String),
    #[error("self-approval rejected: {0}")]
    SelfApproval(String),
    #[error("contract rejected: {0}")]
    Invalid(String),
}

impl ContractError {
    pub(crate) fn from_serde(err: serde_json::Error) -> Self {
        let message = err.to_string();
        if is_self_approval(&message) {
            return Self::SelfApproval(message);
        }
        Self::Json(message)
    }
}

fn is_self_approval(message: &str) -> bool {
    const DENIED: &[&str] = &[
        "approved",
        "structure",
        "formal",
        "empirical",
        "editorial",
        "argumentation",
        "decision",
        "lean_checked",
        "reproduced",
        "claim_supported",
        "status",
        "verified",
    ];
    let Some(field) = unknown_field(message) else {
        return false;
    };
    DENIED.contains(&field)
}

fn unknown_field(message: &str) -> Option<&str> {
    let marker = "unknown field `";
    let start = message.find(marker)? + marker.len();
    let rest = &message[start..];
    let end = rest.find('`')?;
    Some(&rest[..end])
}
