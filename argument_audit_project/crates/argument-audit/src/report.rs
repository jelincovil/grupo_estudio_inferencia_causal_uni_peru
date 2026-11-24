use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Decision {
    /// Every argument is structurally valid. Obligations may remain.
    /// This is a catalog result, not a verified corpus.
    StructurallyValid,
    /// At least one argument or obligation cycle is invalid.
    Rejected,
    /// Nothing was accepted and nothing was structurally invalid,
    /// for example a document with no arguments.
    Pending,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Error,
    Obligation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Diagnostic {
    pub code: String,
    pub severity: Severity,
    pub subject: String,
    pub detail: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatusAxis {
    Valid,
    Invalid,
    Pending,
    LeanChecked,
    Conditional,
    NotApplicable,
    Failed,
    Stale,
    Reproduced,
    ExactGenerated,
    AgentReviewed,
    Mismatch,
    NotRun,
    GroundedIn,
    GroundedOut,
    Undecided,
    BudgetExceeded,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ArgumentFinding {
    pub id: String,
    pub rule: String,
    pub structure: StatusAxis,
    pub obligations: Vec<String>,
    pub mixed_dossier: bool,
    pub diagnostic_codes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ClaimFinding {
    pub id: String,
    pub structure: StatusAxis,
    pub formal: StatusAxis,
    pub empirical: StatusAxis,
    pub editorial: StatusAxis,
    pub argumentation: StatusAxis,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CheckReport {
    pub checker_version: String,
    pub profile: String,
    pub decision: Decision,
    /// Always false in this checker. Kept so a caller cannot confuse a
    /// green structure check with a certified claim.
    pub claim_supported: bool,
    pub pending_count: u32,
    pub diagnostics: Vec<Diagnostic>,
    pub arguments: Vec<ArgumentFinding>,
    pub claims: Vec<ClaimFinding>,
    pub limits: String,
}

impl CheckReport {
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == Severity::Error)
    }
}
