//! SP-B content classification: the live-generation clearance gate and a
//! deterministic PII verdict.
//!
//! `rag` stays unaware of classification — it receives `max_confidentiality` as
//! an opaque retrieval parameter (ADR invariant). These *policy* decisions live
//! server-side: the gate is the belt to retrieval's braces (a higher-confidential
//! chunk must never be turned into a live question for a lower-cleared audience),
//! and the PII verdict is a deterministic, no-AI flag for review.

/// Live generation may proceed only if the most-confidential content in scope
/// does not exceed the audience's clearance. Returns `false` to block.
pub fn live_generation_allowed(audience_clearance: i16, content_confidentiality: i16) -> bool {
    content_confidentiality <= audience_clearance
}

/// A kind of personally-identifiable information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PiiKind {
    Email,
    Iban,
    /// A long national identifier (e.g. a 15-digit French NIR).
    NationalId,
    Phone,
}

/// A deterministic PII verdict over a piece of text: which kinds were detected.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PiiVerdict {
    pub kinds: Vec<PiiKind>,
}

impl PiiVerdict {
    pub fn has_pii(&self) -> bool {
        !self.kinds.is_empty()
    }
}

/// Detect PII deterministically (no AI): email addresses, IBANs, long national
/// identifiers, and phone numbers. Conservative and order-stable — it flags
/// content for review, it does not redact.
pub fn classify_pii(text: &str) -> PiiVerdict {
    let mut kinds = Vec::new();
    if has_email(text) {
        kinds.push(PiiKind::Email);
    }
    if has_iban(text) {
        kinds.push(PiiKind::Iban);
    }
    // A run of >=15 digits reads as a national id; 10..15 as a phone number.
    match max_digit_group(text) {
        n if n >= 15 => kinds.push(PiiKind::NationalId),
        n if n >= 10 => kinds.push(PiiKind::Phone),
        _ => {}
    }
    PiiVerdict { kinds }
}

/// A whitespace token containing `@` with a dot-bearing domain after it.
fn has_email(text: &str) -> bool {
    text.split_whitespace().any(|tok| {
        let t = tok.trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '@' && c != '.');
        match t.split_once('@') {
            Some((local, domain)) => {
                !local.is_empty() && domain.contains('.') && !domain.starts_with('.')
            }
            None => false,
        }
    })
}

/// A token shaped like an IBAN: 2 letters, 2 check digits, then alphanumerics,
/// 15..=34 chars total.
fn has_iban(text: &str) -> bool {
    text.split_whitespace().any(|tok| {
        let t: String = tok.chars().filter(|c| c.is_ascii_alphanumeric()).collect();
        let b = t.as_bytes();
        (15..=34).contains(&b.len())
            && b[0].is_ascii_alphabetic()
            && b[1].is_ascii_alphabetic()
            && b[2].is_ascii_digit()
            && b[3].is_ascii_digit()
            && b[4..].iter().all(u8::is_ascii_alphanumeric)
    })
}

/// The largest count of digits inside any maximal "phone-like" group (digits and
/// the usual separators `+ - . ( ) space`).
fn max_digit_group(text: &str) -> usize {
    let mut best = 0usize;
    let mut cur = 0usize;
    for c in text.chars() {
        if c.is_ascii_digit() {
            cur += 1;
            best = best.max(cur);
        } else if !matches!(c, '+' | '-' | '.' | '(' | ')' | ' ') {
            cur = 0;
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_generation_is_gated_by_clearance() {
        assert!(live_generation_allowed(2, 2), "equal level is allowed");
        assert!(live_generation_allowed(3, 1), "higher clearance is allowed");
        assert!(
            !live_generation_allowed(1, 2),
            "content above clearance must be blocked"
        );
    }

    #[test]
    fn pii_verdict_is_deterministic_per_kind() {
        assert_eq!(
            classify_pii("Contact alice@example.com for details").kinds,
            vec![PiiKind::Email]
        );
        assert!(classify_pii("IBAN FR7630006000011234567890189 please").has_pii());
        assert_eq!(
            classify_pii("NIR 1 85 12 75 108 200 25").kinds,
            vec![PiiKind::NationalId]
        );
        assert_eq!(
            classify_pii("call 01 23 45 67 89 today").kinds,
            vec![PiiKind::Phone]
        );
    }

    #[test]
    fn clean_text_has_no_pii() {
        let v = classify_pii("The mitochondrion is the powerhouse of the cell.");
        assert!(!v.has_pii());
        assert!(v.kinds.is_empty());
    }

    #[test]
    fn determinism_same_input_same_verdict() {
        let t = "Email a@b.co and call 0612345678";
        assert_eq!(classify_pii(t), classify_pii(t));
        assert_eq!(classify_pii(t).kinds, vec![PiiKind::Email, PiiKind::Phone]);
    }
}
