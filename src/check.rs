//! Structural checks for Sirno entries.
//!
//! Sirno checks the shape of entries and structural targets.
//! It does not decide whether prose is true or whether code satisfies a claim.

use std::collections::BTreeMap;

use crate::entry::Entry;
use crate::id::EntryId;

const CATEGORY_FIELD: &str = "category";
const META_CATEGORY_ID: &str = "meta";

/// Boundary at which Sirno checks structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckMode {
    /// Editing checks keep local movement fast.
    Edit,
    /// Review checks treat dangling structural references as errors.
    Review,
}

/// Severity of one structural diagnostic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckSeverity {
    /// A condition worth showing during editing.
    Warning,
    /// A structural violation at the selected boundary.
    Error,
}

/// Reason for one structural diagnostic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckDiagnosticKind {
    /// A structural target id does not name an entry.
    MissingTarget,
    /// A category target is not itself categorized by `meta`.
    CategoryTargetNotMeta,
}

/// One structural diagnostic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckDiagnostic {
    /// Diagnostic severity.
    pub severity: CheckSeverity,
    /// Structural problem detected by the check.
    pub kind: CheckDiagnosticKind,
    /// Entry whose metadata produced the diagnostic.
    pub entry: EntryId,
    /// Metadata field that produced the diagnostic.
    pub field: &'static str,
    /// Referenced id that produced the diagnostic.
    pub target: EntryId,
}

impl CheckDiagnostic {
    /// Human-readable diagnostic message.
    pub fn message(&self) -> String {
        match self.kind {
            | CheckDiagnosticKind::MissingTarget => format!(
                "`{}` references missing entry `{}` through `{}`",
                self.entry, self.target, self.field
            ),
            | CheckDiagnosticKind::CategoryTargetNotMeta => format!(
                "`{}` uses `{}` as a category, but `{}` is not categorized by `meta`",
                self.entry, self.target, self.target
            ),
        }
    }
}

/// Result of checking a set of entries.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CheckReport {
    diagnostics: Vec<CheckDiagnostic>,
}

impl CheckReport {
    /// Construct an empty report.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add one diagnostic to the report.
    pub fn push(&mut self, diagnostic: CheckDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    /// All diagnostics in deterministic check order.
    pub fn diagnostics(&self) -> &[CheckDiagnostic] {
        &self.diagnostics
    }

    /// Returns true when the report contains no diagnostics.
    pub fn is_clean(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Returns true when at least one diagnostic is an error.
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|diagnostic| diagnostic.severity == CheckSeverity::Error)
    }
}

// sirno:witness:structural-check:begin
/// Check structural metadata targets for a set of entries.
///
/// Parsing already enforces required fields, accepted field shapes, and valid id syntax.
/// This pass checks entry ids named by structural fields.
/// It also checks that `category` targets are categorized by `meta`.
pub fn check_entries<'a>(
    entries: impl IntoIterator<Item = &'a Entry>, mode: CheckMode,
) -> CheckReport {
    let entries = entries.into_iter().collect::<Vec<_>>();
    let entries_by_id =
        entries.iter().map(|entry| (entry.id.clone(), *entry)).collect::<BTreeMap<_, _>>();
    let meta_id = EntryId::new(META_CATEGORY_ID)
        .unwrap_or_else(|error| panic!("invalid built-in meta category id: {error}"));
    let severity = match mode {
        | CheckMode::Edit => CheckSeverity::Warning,
        | CheckMode::Review => CheckSeverity::Error,
    };

    let mut report = CheckReport::new();
    for entry in entries {
        for (field, target) in entry.metadata.structural_targets() {
            if !entries_by_id.contains_key(target) {
                report.push(CheckDiagnostic {
                    severity,
                    kind: CheckDiagnosticKind::MissingTarget,
                    entry: entry.id.clone(),
                    field,
                    target: target.clone(),
                });
            }
        }
        for target in &entry.metadata.category {
            let Some(category_entry) = entries_by_id.get(target) else {
                continue;
            };
            if !category_entry.metadata.category.contains(&meta_id) {
                report.push(CheckDiagnostic {
                    severity,
                    kind: CheckDiagnosticKind::CategoryTargetNotMeta,
                    entry: entry.id.clone(),
                    field: CATEGORY_FIELD,
                    target: target.clone(),
                });
            }
        }
    }
    report
}
// sirno:witness:structural-check:end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entry::EntryMetadata;

    fn entry(id: &str) -> Entry {
        Entry::new(EntryId::new(id).unwrap(), EntryMetadata::new(id, "description").unwrap(), "")
    }

    #[test]
    fn clean_entries_produce_clean_report() {
        let mut concept = entry("concept");
        concept.metadata.category.push(EntryId::new("meta").unwrap());
        let mut meta = entry("meta");
        meta.metadata.category.push(EntryId::new("meta").unwrap());

        let report = check_entries([&concept, &meta], CheckMode::Review);
        assert!(report.is_clean());
    }

    #[test]
    fn edit_mode_reports_dangling_reference_as_warning() {
        let mut concept = entry("concept");
        concept.metadata.category.push(EntryId::new("meta").unwrap());

        let report = check_entries([&concept], CheckMode::Edit);
        assert_eq!(report.diagnostics()[0].kind, CheckDiagnosticKind::MissingTarget);
        assert_eq!(report.diagnostics()[0].severity, CheckSeverity::Warning);
        assert!(!report.has_errors());
    }

    #[test]
    fn review_mode_reports_dangling_reference_as_error() {
        let mut concept = entry("concept");
        concept.metadata.category.push(EntryId::new("meta").unwrap());

        let report = check_entries([&concept], CheckMode::Review);
        assert_eq!(report.diagnostics()[0].kind, CheckDiagnosticKind::MissingTarget);
        assert_eq!(report.diagnostics()[0].severity, CheckSeverity::Error);
        assert!(report.has_errors());
    }

    #[test]
    fn edit_mode_reports_category_target_not_categorized_by_meta_as_warning() {
        let mut feature = entry("feature");
        feature.metadata.category.push(EntryId::new("topic").unwrap());
        let mut topic = entry("topic");
        topic.metadata.category.push(EntryId::new("concept").unwrap());
        let mut concept = entry("concept");
        concept.metadata.category.push(EntryId::new("meta").unwrap());
        let mut meta = entry("meta");
        meta.metadata.category.push(EntryId::new("meta").unwrap());

        let report = check_entries([&feature, &topic, &concept, &meta], CheckMode::Edit);

        assert_eq!(report.diagnostics().len(), 1);
        assert_eq!(report.diagnostics()[0].kind, CheckDiagnosticKind::CategoryTargetNotMeta);
        assert_eq!(report.diagnostics()[0].severity, CheckSeverity::Warning);
        assert!(!report.has_errors());
    }

    #[test]
    fn review_mode_reports_category_target_not_categorized_by_meta_as_error() {
        let mut feature = entry("feature");
        feature.metadata.category.push(EntryId::new("topic").unwrap());
        let mut topic = entry("topic");
        topic.metadata.category.push(EntryId::new("concept").unwrap());
        let mut concept = entry("concept");
        concept.metadata.category.push(EntryId::new("meta").unwrap());
        let mut meta = entry("meta");
        meta.metadata.category.push(EntryId::new("meta").unwrap());

        let report = check_entries([&feature, &topic, &concept, &meta], CheckMode::Review);

        assert_eq!(report.diagnostics().len(), 1);
        assert_eq!(report.diagnostics()[0].kind, CheckDiagnosticKind::CategoryTargetNotMeta);
        assert_eq!(report.diagnostics()[0].severity, CheckSeverity::Error);
        assert!(report.has_errors());
    }
}
