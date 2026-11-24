//! Exact rational identity used by the sealed package as a synthetic fixture.
//!
//! This is one finite instance. It is not a statistical law, not a clinical
//! result, and not evidence that a simulator is externally valid.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ratio {
    num: i64,
    den: i64,
}

impl Ratio {
    pub fn new(num: i64, den: i64) -> Option<Self> {
        if den == 0 {
            return None;
        }
        // i128 so i64::MIN and signs are all handled without panicking.
        let mut num = i128::from(num);
        let mut den = i128::from(den);
        if den < 0 {
            num = -num;
            den = -den;
        }
        let divisor = gcd(num.unsigned_abs(), den as u128);
        Some(Self {
            num: i64::try_from(num / divisor as i128).ok()?,
            den: i64::try_from(den / divisor as i128).ok()?,
        })
    }

    pub const fn numer(self) -> i64 {
        self.num
    }

    pub const fn denom(self) -> i64 {
        self.den
    }

    fn mul(self, other: Self) -> Option<Self> {
        Self::new(
            self.num.checked_mul(other.num)?,
            self.den.checked_mul(other.den)?,
        )
    }

    fn add(self, other: Self) -> Option<Self> {
        let num = self
            .num
            .checked_mul(other.den)?
            .checked_add(other.num.checked_mul(self.den)?)?;
        let den = self.den.checked_mul(other.den)?;
        Self::new(num, den)
    }

    fn neg(self) -> Option<Self> {
        Self::new(self.num.checked_neg()?, self.den)
    }
}

impl std::fmt::Display for Ratio {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}/{}", self.num, self.den)
    }
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let next = a % b;
        a = b;
        b = next;
    }
    a.max(1)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SschInstance {
    pub experiment: &'static str,
    pub scope: &'static str,
    pub risk0: Ratio,
    pub risk1: Ratio,
    pub bayes_risk: Ratio,
    pub return0: Ratio,
    pub return1: Ratio,
}

/// Risks of following the signal, and the Bayes risk, by exact enumeration.
///
/// P0(X=1)=1/5, P1(X=1)=4/5, false-positive loss 1, false-negative loss 4,
/// prior of theta=1 equal to 3/10. Synthetic. Not clinical evidence.
pub fn ssch_one_decision_exact_v1() -> Option<SschInstance> {
    let p0_x1 = Ratio::new(1, 5)?;
    let p0_x0 = Ratio::new(4, 5)?;
    let p1_x1 = Ratio::new(4, 5)?;
    let p1_x0 = Ratio::new(1, 5)?;
    let prior1 = Ratio::new(3, 10)?;
    let prior0 = Ratio::new(7, 10)?;
    let loss_fp = Ratio::new(1, 1)?;
    let loss_fn = Ratio::new(4, 1)?;
    let zero = Ratio::new(0, 1)?;

    // Action follows the signal: a = x.
    let risk0 = p0_x0.mul(zero)?.add(p0_x1.mul(loss_fp)?)?;
    let risk1 = p1_x0.mul(loss_fn)?.add(p1_x1.mul(zero)?)?;
    let bayes = prior0.mul(risk0)?.add(prior1.mul(risk1)?)?;
    Some(SschInstance {
        experiment: "ssch_one_decision_exact_v1",
        scope: "exact finite synthetic instance",
        risk0,
        risk1,
        bayes_risk: bayes,
        return0: risk0.neg()?,
        return1: risk1.neg()?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_the_sealed_rational_instance() {
        let got = ssch_one_decision_exact_v1().expect("finite arithmetic");
        // Independent check: 7/10 * 1/5 + 3/10 * 4/5 = 7/50 + 12/50.
        let expected = Ratio::new(7, 50)
            .unwrap()
            .add(Ratio::new(12, 50).unwrap())
            .unwrap();
        assert_eq!(got.risk0, Ratio::new(1, 5).unwrap());
        assert_eq!(got.risk1, Ratio::new(4, 5).unwrap());
        assert_eq!(got.bayes_risk, Ratio::new(19, 50).unwrap());
        assert_eq!(got.bayes_risk, expected);
        assert_eq!(got.return0, got.risk0.neg().unwrap());
        assert_eq!(got.return1, got.risk1.neg().unwrap());
    }
}
