use std::collections::BTreeMap;

use serde::Deserialize;
use serde_json::Value;

use crate::dup::reject_duplicate_keys;
use crate::error::ContractError;

/// A receipt is a candidate record of a run.
///
/// Loading one does not admit it. `execution_passed` is not
/// `scientific_criterion_met`, and neither is `claim_supported`.
/// There is deliberately no field that can set a claim to proved.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Receipt {
    pub schema_version: String,
    pub id: String,
    pub spec_id: String,
    pub spec_hash: String,
    pub source_commit: String,
    pub environment_hash: String,
    pub checker_version: String,
    pub command_argv: Vec<String>,
    pub working_directory: String,
    pub input_hashes: BTreeMap<String, String>,
    pub started_at: String,
    pub finished_at: String,
    pub exit_status: Option<i32>,
    pub termination: Termination,
    pub stdout_sha256: String,
    pub stderr_sha256: String,
    pub artifacts: Vec<Artifact>,
    pub execution_passed: bool,
    pub scientific_criterion_met: Option<bool>,
    pub observed_checks: Vec<ObservedCheck>,
    pub trust_mode: TrustMode,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Artifact {
    pub path: String,
    pub sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ObservedCheck {
    pub id: String,
    pub status: CheckStatus,
    pub detail: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckStatus {
    Pass,
    Fail,
    Skip,
    Pending,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Termination {
    Completed,
    Timeout,
    ResourceLimit,
    LaunchFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustMode {
    LocallyReproduced,
    UntrustedImport,
    AttestedByConfiguredPolicy,
}

impl Receipt {
    pub fn load(bytes: &[u8]) -> Result<Self, ContractError> {
        let text = std::str::from_utf8(bytes).map_err(|_| ContractError::Utf8)?;
        reject_duplicate_keys(text)?;
        let value: Value = serde_json::from_str(text).map_err(ContractError::from_serde)?;
        let receipt: Self = serde_json::from_value(value).map_err(ContractError::from_serde)?;
        receipt.validate_shape()?;
        Ok(receipt)
    }

    pub fn load_all(bytes: &[u8]) -> Result<Vec<Self>, ContractError> {
        let text = std::str::from_utf8(bytes).map_err(|_| ContractError::Utf8)?;
        reject_duplicate_keys(text)?;
        let value: Value = serde_json::from_str(text).map_err(ContractError::from_serde)?;
        let items = value.as_array().ok_or_else(|| {
            ContractError::Invalid("receipt file must be a JSON array".to_string())
        })?;
        items
            .iter()
            .cloned()
            .map(|item| {
                let receipt: Self =
                    serde_json::from_value(item).map_err(ContractError::from_serde)?;
                receipt.validate_shape()?;
                Ok(receipt)
            })
            .collect()
    }

    fn validate_shape(&self) -> Result<(), ContractError> {
        if self.schema_version != "1.0" {
            return Err(ContractError::SchemaVersion(self.schema_version.clone()));
        }
        if self.id.trim().is_empty() || self.spec_id.trim().is_empty() {
            return Err(ContractError::Invalid(
                "receipt id and spec_id are required".to_string(),
            ));
        }
        for hash in [
            &self.spec_hash,
            &self.environment_hash,
            &self.stdout_sha256,
            &self.stderr_sha256,
        ] {
            require_sha256(hash)?;
        }
        for hash in self.input_hashes.values() {
            require_sha256(hash)?;
        }
        for artifact in &self.artifacts {
            require_sha256(&artifact.sha256)?;
            if artifact.path.trim().is_empty() {
                return Err(ContractError::Invalid(
                    "artifact path must not be empty".to_string(),
                ));
            }
        }
        if self.command_argv.is_empty() || self.command_argv.iter().any(|part| part.is_empty()) {
            return Err(ContractError::Invalid(
                "command_argv must be a non-empty argv, never a shell string".to_string(),
            ));
        }
        if self.execution_passed
            && !(self.termination == Termination::Completed && self.exit_status == Some(0))
        {
            return Err(ContractError::Invalid(
                "execution_passed requires termination=completed and exit_status=0".to_string(),
            ));
        }
        Ok(())
    }
}

fn require_sha256(value: &str) -> Result<(), ContractError> {
    let ok = value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase());
    if ok {
        Ok(())
    } else {
        Err(ContractError::Invalid(format!(
            "expected lowercase SHA-256, found `{value}`"
        )))
    }
}
