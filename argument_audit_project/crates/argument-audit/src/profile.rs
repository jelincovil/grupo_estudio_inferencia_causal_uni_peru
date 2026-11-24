use argument_document::{EvidenceKind, ProfileId, Rule};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Profile {
    pub id: ProfileId,
}

impl Profile {
    pub const fn from_id(id: ProfileId) -> Self {
        Self { id }
    }
}

pub fn allows(profile: ProfileId, rule: &Rule) -> bool {
    match profile {
        ProfileId::ScientificAudit => matches!(
            rule,
            Rule::LeanAnchor
                | Rule::LeanApply
                | Rule::AndIntro
                | Rule::ModusPonens
                | Rule::EmpiricalRecord
                | Rule::ConditionalApplication
                | Rule::AliasExact
        ),
        ProfileId::Structural => {
            matches!(rule, Rule::ModusPonens | Rule::AndIntro | Rule::AliasExact)
        }
    }
}

pub fn evidence_is_formal_anchor(kind: EvidenceKind) -> bool {
    matches!(kind, EvidenceKind::Lean)
}

pub fn evidence_is_computational(kind: EvidenceKind) -> bool {
    matches!(kind, EvidenceKind::Python)
}
