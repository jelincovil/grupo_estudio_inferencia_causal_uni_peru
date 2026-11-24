use argument_document::{EvidenceSpec, Receipt, Termination, TrustMode};
use serde::Serialize;

/// The strongest outcome this base will emit for a receipt.
///
/// `claim_supported` is a constant false. Replaying a process, or an
/// explicit attestation policy, has to be a different type that this
/// function does not accept from JSON.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Admission {
    pub receipt_id: String,
    pub spec_id: String,
    pub execution_consistent: bool,
    pub hash_matches_spec: bool,
    pub trust_mode: String,
    pub stale: bool,
    pub execution_admitted: bool,
    pub scientific_criterion_met: Option<bool>,
    pub claim_supported: bool,
    pub reason: String,
}

pub fn admit_receipt(
    receipt: &Receipt,
    spec: &EvidenceSpec,
    expected_spec_hash: &str,
) -> Admission {
    let hash_matches = receipt.spec_id == spec.id && receipt.spec_hash == expected_spec_hash;
    let execution_consistent = !receipt.execution_passed
        || (receipt.termination == Termination::Completed && receipt.exit_status == Some(0));
    let locally = receipt.trust_mode == TrustMode::LocallyReproduced;
    let stale = !hash_matches;
    let execution_admitted =
        locally && hash_matches && execution_consistent && receipt.execution_passed;
    let reason = if !execution_consistent {
        "execution_inconsistent"
    } else if stale {
        "spec_hash_mismatch"
    } else if receipt.trust_mode == TrustMode::UntrustedImport {
        "untrusted_import"
    } else if receipt.trust_mode == TrustMode::AttestedByConfiguredPolicy {
        "attestation_policy_not_configured"
    } else if !receipt.execution_passed {
        "execution_not_passed"
    } else {
        "replay_not_witnessed"
    };
    Admission {
        receipt_id: receipt.id.clone(),
        spec_id: receipt.spec_id.clone(),
        execution_consistent,
        hash_matches_spec: hash_matches,
        trust_mode: trust_label(receipt.trust_mode).to_string(),
        stale,
        execution_admitted,
        scientific_criterion_met: receipt.scientific_criterion_met,
        claim_supported: false,
        reason: reason.to_string(),
    }
}

fn trust_label(mode: TrustMode) -> &'static str {
    match mode {
        TrustMode::LocallyReproduced => "locally_reproduced",
        TrustMode::UntrustedImport => "untrusted_import",
        TrustMode::AttestedByConfiguredPolicy => "attested_by_configured_policy",
    }
}
