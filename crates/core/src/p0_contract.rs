//! Contract-only P0 slice for source-grounded collective sessions.
//!
//! This module is deliberately pure and stub-shaped: it proves the product
//! contract without calling Wrench, Gear, Bolt, Biscuit, model providers, or any
//! UI/server runtime. The real integrations must replace these shaped refs, not
//! expand this module into durable ingestion, memory, orchestration, artifact, or
//! authorization infrastructure.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

pub const P0_FIXTURE_SCHEMA: &str = "rumble_lm.p0_source_grounded_session_fixture.v0.1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P0Fixture {
    pub schema: String,
    pub workspace_id: String,
    pub session: P0Session,
    pub source_set: P0SourceSet,
    pub generation_request: P0GenerationRequest,
    pub activities: Vec<P0Activity>,
    pub citations: Vec<P0Citation>,
    pub responses: P0Responses,
    pub export_manifest: P0ExportManifest,
    pub delegations: Vec<P0Delegation>,
    pub audit_log_sample: Vec<P0AuditEvent>,
    pub sovereignty: P0Sovereignty,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P0Session {
    pub session_id: String,
    pub analytics_mode: AnalyticsMode,
    pub provider_policy_ref: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnalyticsMode {
    AggregateOnly,
    IndividualProfiles,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P0SourceSet {
    pub revision: u32,
    pub status: SourceSetStatus,
    /// Must stay `rumble-lm-selection-only`: source truth is Wrench/Gear-owned.
    pub owner: String,
    pub source_refs: Vec<P0SourceRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SourceSetStatus {
    Open,
    Ready,
    Locked,
    Stale,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P0SourceRef {
    pub source_ref: String,
    pub source_chunk_refs: Vec<String>,
    pub provenance: P0Provenance,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P0Provenance {
    pub owner: String,
    pub produced_by: String,
    pub hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P0GenerationRequest {
    pub source_set_required: bool,
    pub citation_required: bool,
    pub provider_policy_ref: String,
    pub delegation_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P0Activity {
    pub activity_id: String,
    pub status: ActivityStatus,
    pub generated: bool,
    pub grounding_mode: GroundingMode,
    pub claims: Vec<P0Claim>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ActivityStatus {
    Draft,
    Validated,
    Published,
    Running,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum GroundingMode {
    SourceGrounded,
    FacilitatorAuthored,
    Unsupported,
    Mixed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P0Claim {
    pub claim_id: String,
    pub claim_class: ClaimClass,
    pub citation_refs: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ClaimClass {
    SourceDerived,
    FacilitatorAuthored,
    ParticipantDerived,
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P0Citation {
    pub citation_id: String,
    pub source_ref: String,
    pub source_chunk_ref: String,
    pub support_level: SupportLevel,
    pub status: CitationStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SupportLevel {
    Strong,
    Partial,
    Weak,
    Contradicted,
    NotReviewed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CitationStatus {
    Candidate,
    Validated,
    Rejected,
    Stale,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P0Responses {
    pub raw_responses_included: bool,
    pub cross_session_learner_profile_created: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P0ExportManifest {
    pub audience: ExportAudience,
    pub included_data_classes: Vec<String>,
    pub artifact_ref: String,
    pub checksum_sha256: String,
    pub privacy_gate_passed: bool,
    pub citation_gate_passed: bool,
    pub revocation_ref: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ExportAudience {
    FacilitatorOnly,
    Participants,
    AdminAudit,
    MachineReadable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P0Delegation {
    pub delegation_ref: String,
    pub holder: DelegationHolder,
    pub action: DelegatedAction,
    pub workspace: String,
    pub session_id: String,
    pub expires_at: String,
    pub revocation_ref: String,
    pub forbidden_capabilities: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DelegationHolder {
    WrenchLoader,
    GearMemory,
    GearDepot,
    Bolt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DelegatedAction {
    SourceAttach,
    SourceRead,
    RunRequest,
    ExportCreate,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P0AuditEvent {
    pub event_name: String,
    pub metadata_keys: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P0Sovereignty {
    pub mandatory_us_saas: bool,
    pub opaque_storage: bool,
    pub blocking_license_dependency: bool,
    pub silent_third_party_model_fallback: bool,
    pub pii_in_logs: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P0ValidationReport {
    pub valid: bool,
    pub findings: Vec<P0Finding>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P0Finding {
    pub code: String,
    pub target: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P0StubWorkflowProof {
    pub valid: bool,
    pub fixture_valid: bool,
    pub steps: Vec<P0StubStep>,
    pub execution: P0StubExecution,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P0StubStep {
    pub name: String,
    pub ok: bool,
    pub evidence: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P0StubExecution {
    pub ui_executed: bool,
    pub wrench_called: bool,
    pub gear_called: bool,
    pub bolt_called: bool,
    pub biscuit_runtime_called: bool,
    pub llm_provider_called: bool,
    pub durable_storage_written: bool,
}

fn push(
    findings: &mut Vec<P0Finding>,
    code: &'static str,
    target: impl Into<String>,
    message: &'static str,
) {
    findings.push(P0Finding {
        code: code.into(),
        target: target.into(),
        message: message.into(),
    });
}

/// Run a deterministic vertical P0 stub over the valid fixture.
///
/// This simulates the user-visible workflow shape while proving no external
/// runtime is called. It is a contract gate, not product persistence.
pub fn run_p0_stub_workflow() -> P0StubWorkflowProof {
    let fixture = valid_p0_fixture();
    let report = validate_p0_fixture(&fixture);
    let steps = vec![
        P0StubStep {
            name: "create_session".into(),
            ok: !fixture.session.session_id.is_empty(),
            evidence: "session id exists in contract fixture".into(),
        },
        P0StubStep {
            name: "attach_sources".into(),
            ok: !fixture.source_set.source_refs.is_empty()
                && fixture.source_set.owner == "rumble-lm-selection-only",
            evidence: "Rumble stores source-set refs/snapshots only".into(),
        },
        P0StubStep {
            name: "generate_activity_draft".into(),
            ok: fixture.generation_request.source_set_required
                && fixture.generation_request.citation_required,
            evidence: "source set and citations are mandatory for generation".into(),
        },
        P0StubStep {
            name: "validate_citations".into(),
            ok: fixture
                .citations
                .iter()
                .all(|citation| citation.status == CitationStatus::Validated),
            evidence: "all cited source-derived claims have validated citations".into(),
        },
        P0StubStep {
            name: "collect_aggregate_responses".into(),
            ok: !fixture.responses.raw_responses_included
                && !fixture.responses.cross_session_learner_profile_created,
            evidence: "no raw responses or hidden learner profile in proof".into(),
        },
        P0StubStep {
            name: "export_participant_artifact".into(),
            ok: fixture.export_manifest.audience == ExportAudience::Participants
                && fixture.export_manifest.privacy_gate_passed
                && fixture.export_manifest.citation_gate_passed,
            evidence: "participant export passes privacy and citation gates".into(),
        },
        P0StubStep {
            name: "prove_delegation_bounds".into(),
            ok: fixture.delegations.iter().all(|delegation| {
                !delegation.expires_at.is_empty() && !delegation.revocation_ref.is_empty()
            }),
            evidence: "delegations are expiring and revocable".into(),
        },
    ];
    let execution = P0StubExecution {
        ui_executed: false,
        wrench_called: false,
        gear_called: false,
        bolt_called: false,
        biscuit_runtime_called: false,
        llm_provider_called: false,
        durable_storage_written: false,
    };
    let valid = report.valid && steps.iter().all(|step| step.ok);
    P0StubWorkflowProof {
        valid,
        fixture_valid: report.valid,
        steps,
        execution,
    }
}

/// Validate the contract-level P0 invariants. This is not product runtime authz.
pub fn validate_p0_fixture(fixture: &P0Fixture) -> P0ValidationReport {
    let mut findings = Vec::new();

    if fixture.schema != P0_FIXTURE_SCHEMA {
        push(
            &mut findings,
            "schema_invalid",
            "schema",
            "unsupported schema",
        );
    }
    if fixture.source_set.owner != "rumble-lm-selection-only" {
        push(
            &mut findings,
            "source_truth_owned_by_rumble",
            "sourceSet.owner",
            "Rumble LM must store source-set selection only",
        );
    }
    if !matches!(
        fixture.source_set.status,
        SourceSetStatus::Ready | SourceSetStatus::Locked
    ) {
        push(
            &mut findings,
            "source_set_not_ready",
            "sourceSet.status",
            "source-grounded generation requires a ready or locked source set",
        );
    }
    if fixture.source_set.source_refs.is_empty() {
        push(
            &mut findings,
            "source_set_required",
            "sourceSet.sourceRefs",
            "at least one source ref is required",
        );
    }

    let mut source_refs = BTreeSet::new();
    let mut chunk_refs = BTreeSet::new();
    for source in &fixture.source_set.source_refs {
        source_refs.insert(source.source_ref.as_str());
        chunk_refs.extend(source.source_chunk_refs.iter().map(String::as_str));
        if source.provenance.owner != "gear-memory" {
            push(
                &mut findings,
                "source_provenance_not_gear",
                &source.source_ref,
                "source provenance must be Gear-owned",
            );
        }
        if source.provenance.produced_by != "wrench-loader" {
            push(
                &mut findings,
                "source_not_extracted_by_wrench",
                &source.source_ref,
                "canonical extraction must come from Wrench Loader",
            );
        }
        if !is_sha256_tag(&source.provenance.hash) {
            push(
                &mut findings,
                "source_hash_invalid",
                &source.source_ref,
                "source provenance requires sha256 hash",
            );
        }
    }

    if !fixture.generation_request.source_set_required {
        push(
            &mut findings,
            "generation_source_not_required",
            "generationRequest.sourceSetRequired",
            "source-grounded generation must require a source set",
        );
    }
    if !fixture.generation_request.citation_required {
        push(
            &mut findings,
            "generation_citation_not_required",
            "generationRequest.citationRequired",
            "source-grounded generation must require citations",
        );
    }
    if fixture.generation_request.provider_policy_ref.is_empty() {
        push(
            &mut findings,
            "provider_policy_required",
            "generationRequest.providerPolicyRef",
            "provider policy must be explicit",
        );
    }

    for activity in &fixture.activities {
        for claim in &activity.claims {
            if activity.grounding_mode == GroundingMode::SourceGrounded
                && claim.claim_class == ClaimClass::SourceDerived
                && claim.citation_refs.is_empty()
            {
                push(
                    &mut findings,
                    "citation_required",
                    &claim.claim_id,
                    "source-derived claims require citations",
                );
            }
            for citation_ref in &claim.citation_refs {
                match fixture
                    .citations
                    .iter()
                    .find(|c| &c.citation_id == citation_ref)
                {
                    Some(citation) => {
                        validate_citation(&mut findings, citation, &source_refs, &chunk_refs)
                    }
                    None => push(
                        &mut findings,
                        "citation_missing",
                        citation_ref,
                        "claim references a missing citation",
                    ),
                }
            }
        }
    }

    if fixture.session.analytics_mode != AnalyticsMode::AggregateOnly {
        push(
            &mut findings,
            "analytics_must_be_aggregate",
            "session.analyticsMode",
            "P0 analytics must be aggregate-only by default",
        );
    }
    if fixture.responses.raw_responses_included {
        push(
            &mut findings,
            "raw_responses_forbidden",
            "responses.rawResponsesIncluded",
            "contract proof must not include raw participant responses",
        );
    }
    if fixture.responses.cross_session_learner_profile_created {
        push(
            &mut findings,
            "hidden_profiling_forbidden",
            "responses.crossSessionLearnerProfileCreated",
            "hidden cross-session learner profiles are forbidden",
        );
    }

    validate_export(&mut findings, &fixture.export_manifest);
    validate_delegations(&mut findings, fixture);
    validate_audit(&mut findings, &fixture.audit_log_sample);
    validate_sovereignty(&mut findings, &fixture.sovereignty);

    P0ValidationReport {
        valid: findings.is_empty(),
        findings,
    }
}

fn validate_citation(
    findings: &mut Vec<P0Finding>,
    citation: &P0Citation,
    source_refs: &BTreeSet<&str>,
    chunk_refs: &BTreeSet<&str>,
) {
    if citation.status != CitationStatus::Validated {
        push(
            findings,
            "citation_status_unsatisfied",
            &citation.citation_id,
            "citation must be validated",
        );
    }
    if matches!(
        citation.support_level,
        SupportLevel::Weak | SupportLevel::Contradicted | SupportLevel::NotReviewed
    ) {
        push(
            findings,
            "citation_support_weak",
            &citation.citation_id,
            "weak/contradicted/not-reviewed citation cannot satisfy grounding",
        );
    }
    if !source_refs.contains(citation.source_ref.as_str()) {
        push(
            findings,
            "citation_source_missing",
            &citation.citation_id,
            "citation source must belong to source set",
        );
    }
    if !chunk_refs.contains(citation.source_chunk_ref.as_str()) {
        push(
            findings,
            "citation_chunk_missing",
            &citation.citation_id,
            "citation chunk must belong to source set",
        );
    }
}

fn validate_export(findings: &mut Vec<P0Finding>, export: &P0ExportManifest) {
    let private = ["private_responses", "facilitator_only_notes"];
    if export.audience == ExportAudience::Participants
        && export
            .included_data_classes
            .iter()
            .any(|class| private.contains(&class.as_str()))
    {
        push(
            findings,
            "participant_export_private_data",
            "exportManifest.includedDataClasses",
            "participant exports exclude private data by default",
        );
    }
    if export.artifact_ref.is_empty() {
        push(
            findings,
            "export_artifact_ref_required",
            "exportManifest.artifactRef",
            "export requires an artifact ref",
        );
    }
    if !is_sha256_hex(&export.checksum_sha256) {
        push(
            findings,
            "export_checksum_required",
            "exportManifest.checksumSha256",
            "export requires sha256 checksum",
        );
    }
    if !export.privacy_gate_passed || !export.citation_gate_passed {
        push(
            findings,
            "export_gates_required",
            "exportManifest.gates",
            "export requires privacy and citation gates",
        );
    }
    if export.revocation_ref.is_empty() {
        push(
            findings,
            "export_revocation_ref_required",
            "exportManifest.revocationRef",
            "export requires revocation ref",
        );
    }
}

fn validate_delegations(findings: &mut Vec<P0Finding>, fixture: &P0Fixture) {
    let actions = fixture
        .delegations
        .iter()
        .map(|delegation| delegation.action)
        .collect::<BTreeSet<_>>();
    for action in [
        DelegatedAction::SourceAttach,
        DelegatedAction::RunRequest,
        DelegatedAction::ExportCreate,
    ] {
        if !actions.contains(&action) {
            push(
                findings,
                "delegation_action_missing",
                format!("delegations.{action:?}"),
                "required delegated action is missing",
            );
        }
    }
    for delegation in &fixture.delegations {
        if delegation.workspace != fixture.workspace_id {
            push(
                findings,
                "delegation_workspace_mismatch",
                &delegation.delegation_ref,
                "delegation workspace must match fixture workspace",
            );
        }
        if delegation.session_id != fixture.session.session_id {
            push(
                findings,
                "delegation_session_mismatch",
                &delegation.delegation_ref,
                "delegation session must match fixture session",
            );
        }
        if delegation.expires_at.is_empty() {
            push(
                findings,
                "delegation_expiry_required",
                &delegation.delegation_ref,
                "delegation must be time-bounded",
            );
        }
        if delegation.revocation_ref.is_empty() {
            push(
                findings,
                "delegation_revocation_required",
                &delegation.delegation_ref,
                "delegation must have revocation ref",
            );
        }
        if delegation.action == DelegatedAction::RunRequest
            && !delegation
                .forbidden_capabilities
                .iter()
                .any(|cap| cap == "activity:publish")
        {
            push(
                findings,
                "bolt_publish_forbidden_missing",
                &delegation.delegation_ref,
                "Bolt generation delegation must forbid publishing",
            );
        }
        if delegation.action == DelegatedAction::ExportCreate
            && !delegation
                .forbidden_capabilities
                .iter()
                .any(|cap| cap == "private_response:read")
        {
            push(
                findings,
                "export_private_read_forbidden_missing",
                &delegation.delegation_ref,
                "export delegation must forbid private response reads by default",
            );
        }
    }
}

fn validate_audit(findings: &mut Vec<P0Finding>, audit: &[P0AuditEvent]) {
    let forbidden = [
        "secret",
        "token",
        "bearer",
        "password",
        "credential",
        "raw_response",
    ];
    for event in audit {
        for key in &event.metadata_keys {
            let normalized = key.replace('_', "").to_ascii_lowercase();
            if forbidden
                .iter()
                .any(|marker| normalized.contains(&marker.replace('_', "")))
            {
                push(
                    findings,
                    "secret_or_pii_in_logs",
                    &event.event_name,
                    "audit metadata must not contain raw responses, tokens, or secrets",
                );
            }
        }
    }
}

fn validate_sovereignty(findings: &mut Vec<P0Finding>, sovereignty: &P0Sovereignty) {
    let checks = [
        (sovereignty.mandatory_us_saas, "mandatoryUsSaas"),
        (sovereignty.opaque_storage, "opaqueStorage"),
        (
            sovereignty.blocking_license_dependency,
            "blockingLicenseDependency",
        ),
        (
            sovereignty.silent_third_party_model_fallback,
            "silentThirdPartyModelFallback",
        ),
        (sovereignty.pii_in_logs, "piiInLogs"),
    ];
    for (failed, target) in checks {
        if failed {
            push(
                findings,
                "sovereignty_filter_failed",
                target,
                "sovereignty filters must be false for P0",
            );
        }
    }
}

fn is_sha256_tag(value: &str) -> bool {
    value.strip_prefix("sha256:").is_some_and(is_sha256_hex)
}

fn is_sha256_hex(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

/// Deterministic contract-valid P0 fixture for tests and stub development.
pub fn valid_p0_fixture() -> P0Fixture {
    P0Fixture {
        schema: P0_FIXTURE_SCHEMA.into(),
        workspace_id: "workspace-eu-001".into(),
        session: P0Session {
            session_id: "session-p0-001".into(),
            analytics_mode: AnalyticsMode::AggregateOnly,
            provider_policy_ref: "provider-policy-eu-self-hosted-or-approved".into(),
        },
        source_set: P0SourceSet {
            revision: 1,
            status: SourceSetStatus::Locked,
            owner: "rumble-lm-selection-only".into(),
            source_refs: vec![P0SourceRef {
                source_ref: "gear-source:rumble-lm-charter:001".into(),
                source_chunk_refs: vec!["gear-source-chunk:rumble-lm-charter:001".into()],
                provenance: P0Provenance {
                    owner: "gear-memory".into(),
                    produced_by: "wrench-loader".into(),
                    hash: format!("sha256:{}", "a".repeat(64)),
                },
            }],
        },
        generation_request: P0GenerationRequest {
            source_set_required: true,
            citation_required: true,
            provider_policy_ref: "provider-policy-eu-self-hosted-or-approved".into(),
            delegation_ref: "delegation:generation:001".into(),
        },
        activities: vec![P0Activity {
            activity_id: "activity-p0-001".into(),
            status: ActivityStatus::Published,
            generated: true,
            grounding_mode: GroundingMode::SourceGrounded,
            claims: vec![P0Claim {
                claim_id: "claim-p0-001".into(),
                claim_class: ClaimClass::SourceDerived,
                citation_refs: vec!["citation-p0-001".into()],
            }],
        }],
        citations: vec![P0Citation {
            citation_id: "citation-p0-001".into(),
            source_ref: "gear-source:rumble-lm-charter:001".into(),
            source_chunk_ref: "gear-source-chunk:rumble-lm-charter:001".into(),
            support_level: SupportLevel::Strong,
            status: CitationStatus::Validated,
        }],
        responses: P0Responses {
            raw_responses_included: false,
            cross_session_learner_profile_created: false,
        },
        export_manifest: P0ExportManifest {
            audience: ExportAudience::Participants,
            included_data_classes: vec![
                "session_metadata".into(),
                "activities".into(),
                "aggregate_results".into(),
                "validated_summary".into(),
                "citations".into(),
                "source_provenance".into(),
            ],
            artifact_ref: "gear-artifact:rumble-lm-session-export:001".into(),
            checksum_sha256: "b".repeat(64),
            privacy_gate_passed: true,
            citation_gate_passed: true,
            revocation_ref: "revocation:export-p0-001".into(),
        },
        delegations: vec![
            P0Delegation {
                delegation_ref: "delegation:source-import:001".into(),
                holder: DelegationHolder::WrenchLoader,
                action: DelegatedAction::SourceAttach,
                workspace: "workspace-eu-001".into(),
                session_id: "session-p0-001".into(),
                expires_at: "2026-06-30T01:00:00Z".into(),
                revocation_ref: "revocation:source-import:001".into(),
                forbidden_capabilities: vec!["response:read".into(), "export:create".into()],
            },
            P0Delegation {
                delegation_ref: "delegation:generation:001".into(),
                holder: DelegationHolder::Bolt,
                action: DelegatedAction::RunRequest,
                workspace: "workspace-eu-001".into(),
                session_id: "session-p0-001".into(),
                expires_at: "2026-06-30T01:00:00Z".into(),
                revocation_ref: "revocation:generation:001".into(),
                forbidden_capabilities: vec!["activity:publish".into(), "citation:validate".into()],
            },
            P0Delegation {
                delegation_ref: "delegation:export:001".into(),
                holder: DelegationHolder::GearDepot,
                action: DelegatedAction::ExportCreate,
                workspace: "workspace-eu-001".into(),
                session_id: "session-p0-001".into(),
                expires_at: "2026-06-30T01:00:00Z".into(),
                revocation_ref: "revocation:export:001".into(),
                forbidden_capabilities: vec![
                    "private_response:read".into(),
                    "source:read_all".into(),
                ],
            },
        ],
        audit_log_sample: vec![P0AuditEvent {
            event_name: "response.submitted".into(),
            metadata_keys: vec!["activityRunId".into(), "responseCountAfterSubmit".into()],
        }],
        sovereignty: P0Sovereignty {
            mandatory_us_saas: false,
            opaque_storage: false,
            blocking_license_dependency: false,
            silent_third_party_model_fallback: false,
            pii_in_logs: false,
        },
    }
}

/// Deterministic invalid fixture that exercises the critical refusal codes.
pub fn invalid_p0_fixture() -> P0Fixture {
    let mut fixture = valid_p0_fixture();
    fixture.source_set.status = SourceSetStatus::Open;
    fixture.source_set.owner = "rumble-lm-durable-memory".into();
    fixture.source_set.source_refs.clear();
    fixture.generation_request.source_set_required = false;
    fixture.citations[0].source_ref = "missing-source".into();
    fixture.citations[0].source_chunk_ref = "missing-chunk".into();
    fixture.citations[0].support_level = SupportLevel::Weak;
    fixture.session.analytics_mode = AnalyticsMode::IndividualProfiles;
    fixture.responses.raw_responses_included = true;
    fixture.responses.cross_session_learner_profile_created = true;
    fixture
        .export_manifest
        .included_data_classes
        .push("private_responses".into());
    fixture.export_manifest.checksum_sha256.clear();
    fixture.delegations[1].workspace = "workspace-other".into();
    fixture.delegations[1].expires_at.clear();
    fixture.audit_log_sample[0]
        .metadata_keys
        .push("bearerToken".into());
    fixture.sovereignty.mandatory_us_saas = true;
    fixture.sovereignty.pii_in_logs = true;
    fixture
}

#[cfg(test)]
mod tests {
    use super::*;

    fn codes(report: &P0ValidationReport) -> BTreeSet<&str> {
        report
            .findings
            .iter()
            .map(|finding| finding.code.as_str())
            .collect()
    }

    #[test]
    fn valid_contract_fixture_passes() {
        let report = validate_p0_fixture(&valid_p0_fixture());
        assert!(report.valid, "unexpected findings: {:#?}", report.findings);
    }

    #[test]
    fn invalid_contract_fixture_fails_expected_gates() {
        let report = validate_p0_fixture(&invalid_p0_fixture());
        assert!(!report.valid);
        let codes = codes(&report);
        for expected in [
            "source_truth_owned_by_rumble",
            "source_set_not_ready",
            "source_set_required",
            "generation_source_not_required",
            "citation_support_weak",
            "hidden_profiling_forbidden",
            "raw_responses_forbidden",
            "participant_export_private_data",
            "export_checksum_required",
            "delegation_workspace_mismatch",
            "delegation_expiry_required",
            "secret_or_pii_in_logs",
            "sovereignty_filter_failed",
        ] {
            assert!(
                codes.contains(expected),
                "missing {expected}; got {codes:?}"
            );
        }
    }

    #[test]
    fn bolt_generation_delegation_cannot_publish() {
        let fixture = valid_p0_fixture();
        let generation = fixture
            .delegations
            .iter()
            .find(|delegation| delegation.action == DelegatedAction::RunRequest)
            .unwrap();
        assert!(
            generation
                .forbidden_capabilities
                .contains(&"activity:publish".to_string())
        );
        assert!(
            generation
                .forbidden_capabilities
                .contains(&"citation:validate".to_string())
        );
    }

    #[test]
    fn participant_export_excludes_private_data_by_default() {
        let fixture = valid_p0_fixture();
        assert_eq!(
            fixture.export_manifest.audience,
            ExportAudience::Participants
        );
        assert!(
            !fixture
                .export_manifest
                .included_data_classes
                .iter()
                .any(|class| class == "private_responses" || class == "facilitator_only_notes")
        );
    }

    #[test]
    fn p0_contract_roundtrips_as_json() {
        let fixture = valid_p0_fixture();
        let json = serde_json::to_string(&fixture).unwrap();
        assert!(json.contains(P0_FIXTURE_SCHEMA));
        let decoded: P0Fixture = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, fixture);
    }

    #[test]
    fn p0_stub_workflow_proves_vertical_without_runtime_calls() {
        let proof = run_p0_stub_workflow();
        assert!(proof.valid);
        assert!(proof.fixture_valid);
        assert!(proof.steps.iter().all(|step| step.ok));
        assert!(!proof.execution.ui_executed);
        assert!(!proof.execution.wrench_called);
        assert!(!proof.execution.gear_called);
        assert!(!proof.execution.bolt_called);
        assert!(!proof.execution.biscuit_runtime_called);
        assert!(!proof.execution.llm_provider_called);
        assert!(!proof.execution.durable_storage_written);
    }
}
