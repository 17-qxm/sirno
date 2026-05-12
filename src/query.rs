//! Query predicates for Sirno entries.
//!
//! A query is a typed predicate over parsed Markdown entries.
//! It selects entries and leaves presentation to the caller.

use tracing::trace;

use crate::entry::Entry;
use crate::id::EntryId;

/// Case-insensitive text term for an entry query.
///
/// Empty terms are ignored by `EntryQuery::with_text_terms`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntryTextTerm {
    normalized: String,
}

impl EntryTextTerm {
    /// Construct a text term using Unicode lowercase conversion.
    pub fn new(raw: impl Into<String>) -> Self {
        Self { normalized: raw.into().to_lowercase() }
    }

    /// Normalized text used for matching.
    pub fn normalized(&self) -> &str {
        &self.normalized
    }

    fn is_empty(&self) -> bool {
        self.normalized.is_empty()
    }

    fn matches(&self, haystack: &str) -> bool {
        haystack.contains(&self.normalized)
    }
}

/// Predicate over Sirno entries.
///
/// Text terms are conjunctive.
/// Distinct metadata fields are conjunctive.
/// Repeated values inside one metadata field are disjunctive.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EntryQuery {
    text_terms: Vec<EntryTextTerm>,
    category: Vec<EntryId>,
    clustee: Vec<EntryId>,
    refiner: Vec<EntryId>,
    witness: bool,
}

impl EntryQuery {
    /// Construct an empty query that matches every entry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set text terms matched against id, name, description, and body.
    pub fn with_text_terms(mut self, terms: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.text_terms =
            terms.into_iter().map(EntryTextTerm::new).filter(|term| !term.is_empty()).collect();
        self
    }

    /// Set category targets.
    pub fn with_category(mut self, targets: impl IntoIterator<Item = EntryId>) -> Self {
        self.category = targets.into_iter().collect();
        self
    }

    /// Set clustee targets.
    pub fn with_clustee(mut self, targets: impl IntoIterator<Item = EntryId>) -> Self {
        self.clustee = targets.into_iter().collect();
        self
    }

    /// Set refiner targets.
    pub fn with_refiner(mut self, targets: impl IntoIterator<Item = EntryId>) -> Self {
        self.refiner = targets.into_iter().collect();
        self
    }

    /// Require the canonical witness marker.
    pub fn with_witness(mut self, witness: bool) -> Self {
        self.witness = witness;
        self
    }

    /// Returns true when this query selects the entry.
    pub fn matches(&self, entry: &Entry) -> bool {
        self.matches_text(entry)
            && matches_relation(&entry.metadata.category, &self.category)
            && matches_relation(&entry.metadata.clustee, &self.clustee)
            && matches_relation(&entry.metadata.refiner, &self.refiner)
            && (!self.witness || entry.metadata.witness.is_some())
    }

    fn matches_text(&self, entry: &Entry) -> bool {
        if self.text_terms.is_empty() {
            return true;
        }

        let haystack = entry_text(entry);
        self.text_terms.iter().all(|term| term.matches(&haystack))
    }
}

/// Return entries selected by a query in input order.
pub fn query_entries<'a>(
    entries: impl IntoIterator<Item = &'a Entry>, query: &EntryQuery,
) -> Vec<&'a Entry> {
    let entries = entries.into_iter().collect::<Vec<_>>();
    trace!("query_entries begin: entries={}", entries.len());
    let matches = entries.into_iter().filter(|entry| query.matches(entry)).collect::<Vec<_>>();
    trace!("query_entries end: matches={}", matches.len());
    matches
}

fn matches_relation(entry_targets: &[EntryId], query_targets: &[EntryId]) -> bool {
    query_targets.is_empty() || query_targets.iter().any(|target| entry_targets.contains(target))
}

fn entry_text(entry: &Entry) -> String {
    format!("{}\n{}\n{}\n{}", entry.id, entry.metadata.name, entry.metadata.description, entry.body)
        .to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entry::{EntryMetadata, WitnessMarker};

    fn id(raw: &str) -> EntryId {
        EntryId::new(raw).unwrap()
    }

    fn entry(raw_id: &str, name: &str, description: &str, body: &str) -> Entry {
        Entry::new(id(raw_id), EntryMetadata::new(name, description).unwrap(), body)
    }

    #[test]
    fn empty_query_matches_every_entry() {
        let concept = entry("concept", "Concept", "A named idea.", "");

        assert!(EntryQuery::new().matches(&concept));
    }

    #[test]
    fn text_terms_match_entry_text_case_insensitively() {
        let concept = entry(
            "concept",
            "Concept",
            "A named idea.",
            "A cognitive route through project knowledge.",
        );

        let query = EntryQuery::new().with_text_terms(["ROUTE", "project"]);

        assert!(query.matches(&concept));
        assert!(!EntryQuery::new().with_text_terms(["missing"]).matches(&concept));
    }

    #[test]
    fn relation_values_are_disjunctive_inside_one_field() {
        let mut concept = entry("concept", "Concept", "A named idea.", "");
        concept.metadata.category.push(id("meta"));

        let query = EntryQuery::new().with_category([id("narrative"), id("meta")]);

        assert!(query.matches(&concept));
    }

    #[test]
    fn relation_fields_are_conjunctive_across_fields() {
        let mut concept = entry("concept", "Concept", "A named idea.", "");
        concept.metadata.category.push(id("meta"));
        concept.metadata.clustee.push(id("knowledge"));

        let matching =
            EntryQuery::new().with_category([id("meta")]).with_clustee([id("knowledge")]);
        let missing = EntryQuery::new().with_category([id("meta")]).with_clustee([id("reader")]);

        assert!(matching.matches(&concept));
        assert!(!missing.matches(&concept));
    }

    #[test]
    fn witness_filter_requires_marker() {
        let plain = entry("concept", "Concept", "A named idea.", "");
        let mut witnessed = entry("witnessed", "Witnessed", "A witnessed idea.", "");
        witnessed.metadata.witness = Some(WitnessMarker::Present);

        let query = EntryQuery::new().with_witness(true);

        assert!(!query.matches(&plain));
        assert!(query.matches(&witnessed));
    }

    #[test]
    fn query_entries_preserves_input_order() {
        let first = entry("first", "First", "A first idea.", "");
        let second = entry("second", "Second", "A second idea.", "");
        let entries = [&first, &second];

        let matches = query_entries(entries, &EntryQuery::new().with_text_terms(["idea"]));

        assert_eq!(matches, vec![&first, &second]);
    }
}
