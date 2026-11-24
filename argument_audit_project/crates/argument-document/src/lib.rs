//! Typed documents for an argument kernel.
//!
//! The types are corpus-agnostic. A book is one profile and one source
//! system, not a special case baked into the checker. Nothing in this crate
//! assigns `lean_checked`, `reproduced`, or `claim_supported`. Those are
//! outputs of a checker that has actually admitted evidence, and this loader
//! refuses documents that try to write them in.

mod canonical;
mod dup;
mod error;
mod model;
mod receipt;

pub use canonical::{canonical_kernel, evidence_spec_sha256, kernel_sha256, sha256_hex};
pub use dup::reject_duplicate_keys;
pub use error::ContractError;
pub use model::{
    is_id, Availability, Claim, ClaimKind, Corpus, CorpusKind, EvidenceKind, EvidenceSpec,
    Expression, Kernel, LoadedArgument, ProfileId, Rule, SchemaVersion, SourceLocator, ID_HELP,
};
pub use receipt::{Artifact, CheckStatus, ObservedCheck, Receipt, Termination, TrustMode};
