//! Generated Markdown links for entries.
//!
//! Sirno owns only the guard-bounded generated-link region.
//! Prose outside the region remains user-owned.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

use crate::entry::Entry;

/// Settings for one generated-link structural field.
///
/// `to` includes links from the current entry to metadata targets.
/// `from` includes links from the current entry to entries that point at it.
// sirno:witness:generated-link-policy:begin
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GeneratedLinkFieldSettings {
    /// Include outgoing metadata targets.
    pub to: bool,
    /// Include incoming metadata sources.
    pub from: bool,
}
// sirno:witness:generated-link-policy:end

// sirno:witness:generated-link-policy:begin
impl GeneratedLinkFieldSettings {
    /// Construct structural-field link settings from explicit sides.
    pub fn new(to: bool, from: bool) -> Self {
        Self { to, from }
    }

    /// Construct structural-field link settings from one boolean applied to both sides.
    pub fn from_bool(enabled: bool) -> Self {
        Self::new(enabled, enabled)
    }

    /// Construct enabled structural-field link settings.
    pub fn enabled() -> Self {
        Self::from_bool(true)
    }

    /// Construct disabled structural-field link settings.
    pub fn disabled() -> Self {
        Self::from_bool(false)
    }
}
// sirno:witness:generated-link-policy:end

impl Default for GeneratedLinkFieldSettings {
    fn default() -> Self {
        Self::disabled()
    }
}

impl From<bool> for GeneratedLinkFieldSettings {
    fn from(value: bool) -> Self {
        Self::from_bool(value)
    }
}

impl fmt::Display for GeneratedLinkFieldSettings {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.to == self.from {
            write!(formatter, "{}", self.to)
        } else {
            write!(formatter, "to={} from={}", self.to, self.from)
        }
    }
}

// sirno:witness:generated-link-policy:begin
impl Serialize for GeneratedLinkFieldSettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.to == self.from {
            return serializer.serialize_bool(self.to);
        }

        let mut state = serializer.serialize_struct("GeneratedLinkFieldSettings", 2)?;
        state.serialize_field("to", &self.to)?;
        state.serialize_field("from", &self.from)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for GeneratedLinkFieldSettings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = GeneratedLinkFieldValue::deserialize(deserializer)?;
        Ok(match value {
            | GeneratedLinkFieldValue::Bool(enabled) => Self::from_bool(enabled),
            | GeneratedLinkFieldValue::Sides(sides) => Self::new(sides.to, sides.from),
        })
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum GeneratedLinkFieldValue {
    Bool(bool),
    Sides(GeneratedLinkSides),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedLinkSides {
    to: bool,
    from: bool,
}
// sirno:witness:generated-link-policy:end

/// Settings that choose which metadata fields become generated links.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(default, deny_unknown_fields)]
// sirno:witness:generated-link-policy:begin
pub struct GeneratedLinkSettings {
    /// Include `category` targets.
    pub category: GeneratedLinkFieldSettings,
    /// Include `belongs` targets.
    pub belongs: GeneratedLinkFieldSettings,
    /// Render clique sections derived from `belongs` targets.
    pub clique: bool,
    /// Include `refines` targets.
    pub refines: GeneratedLinkFieldSettings,
}
// sirno:witness:generated-link-policy:end

impl Default for GeneratedLinkSettings {
    fn default() -> Self {
        Self {
            category: GeneratedLinkFieldSettings::disabled(),
            belongs: GeneratedLinkFieldSettings::enabled(),
            clique: false,
            refines: GeneratedLinkFieldSettings::disabled(),
        }
    }
}

impl GeneratedLinkSettings {
    /// Render the generated-link footer for one entry using only that entry as context.
    ///
    /// Use `GeneratedLinkIndex::from_entries` when clique expansion needs the full lake.
    // sirno:witness:generated-footer:begin
    pub fn render_entry(&self, entry: &Entry) -> String {
        GeneratedLinkIndex::from_entries(std::slice::from_ref(entry)).render_entry(entry, self)
    }
    // sirno:witness:generated-footer:end
}

/// Borrowed Markdown body whose generated-link footer can be inspected or changed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GeneratedLinkBody<'a> {
    body: &'a str,
}

impl<'a> GeneratedLinkBody<'a> {
    /// Borrow an entry body for generated-link operations.
    pub fn new(body: &'a str) -> Self {
        Self { body }
    }

    /// Validate generated-link guard boundaries.
    // sirno:witness:generated-footer-ownership:begin
    pub fn validate(&self) -> Result<(), GeneratedLinkError> {
        self.bounds().map(|_| ())
    }
    // sirno:witness:generated-footer-ownership:end

    /// Returns true when an existing generated-link region differs from `expected`.
    ///
    /// Bodies without a generated-link region are not stale.
    // sirno:witness:generated-footer:begin
    pub fn is_stale(&self, expected: &str) -> Result<bool, GeneratedLinkError> {
        let Some(bounds) = self.bounds()? else {
            return Ok(false);
        };
        Ok(&self.body[bounds.region_start..bounds.region_end] != expected)
    }
    // sirno:witness:generated-footer:end

    /// Apply generated links to an entry body.
    ///
    /// If no generated-link region exists, one is appended.
    /// If one valid generated-link region exists, only that region is replaced.
    // sirno:witness:generated-footer:begin
    pub fn apply(&self, footer: &str) -> Result<String, GeneratedLinkError> {
        let Some(bounds) = self.bounds()? else {
            return Ok(self.append_footer(footer));
        };
        let region_end = bounds.next_line_start(self.body);
        let before = self.body[..bounds.region_start].trim_end_matches('\n');
        let after = self.body[region_end..].trim_start_matches('\n');

        let mut out = String::new();
        if !before.is_empty() {
            out.push_str(before);
            out.push_str("\n\n");
        }
        out.push_str(footer);
        out.push('\n');
        if !after.is_empty() {
            out.push('\n');
            out.push_str(after);
        }
        Ok(out)
    }
    // sirno:witness:generated-footer:end

    /// Delete generated links from an entry body.
    ///
    /// If no generated-link region exists, the body is returned unchanged.
    // sirno:witness:generated-footer:begin
    pub fn delete(&self) -> Result<String, GeneratedLinkError> {
        let Some(bounds) = self.bounds()? else {
            return Ok(self.body.to_owned());
        };
        let region_end = bounds.next_line_start(self.body);
        let before = self.body[..bounds.region_start].trim_end_matches('\n');
        let after = self.body[region_end..].trim_start_matches('\n');

        let mut out = String::new();
        if !before.is_empty() {
            out.push_str(before);
        }
        if !before.is_empty() && !after.is_empty() {
            out.push_str("\n\n");
        }
        if !after.is_empty() {
            out.push_str(after);
        }
        if after.is_empty() && self.body.ends_with('\n') && !out.is_empty() {
            out.push('\n');
        }
        Ok(out)
    }
    // sirno:witness:generated-footer:end

    fn bounds(&self) -> Result<Option<GeneratedLinkBounds>, GeneratedLinkError> {
        GeneratedLinkBounds::find(self.body)
    }

    // sirno:witness:generated-footer:begin
    fn append_footer(&self, footer: &str) -> String {
        let before = self.body.trim_end_matches('\n');
        let mut out = String::new();
        if !before.is_empty() {
            out.push_str(before);
            out.push_str("\n\n");
            if !Self::ends_with_divider(before) {
                out.push_str(GENERATED_LINK_DIVIDER);
                out.push_str("\n\n");
            }
        }
        out.push_str(footer);
        out.push('\n');
        out
    }
    // sirno:witness:generated-footer:end

    fn ends_with_divider(body: &str) -> bool {
        body.lines()
            .rev()
            .find(|line| !line.trim().is_empty())
            .is_some_and(|line| line.trim() == GENERATED_LINK_DIVIDER)
    }
}

/// Lake-wide context for generated-link rendering.
///
/// Invariant: each `belongs` target maps to itself and every parsed entry that names it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
// sirno:witness:generated-footer:begin
pub struct GeneratedLinkIndex {
    category_sources_by_target: BTreeMap<crate::EntryId, BTreeSet<crate::EntryId>>,
    cliques_by_belongs_target: BTreeMap<crate::EntryId, BTreeSet<crate::EntryId>>,
    belongs_sources_by_target: BTreeMap<crate::EntryId, BTreeSet<crate::EntryId>>,
    refines_sources_by_target: BTreeMap<crate::EntryId, BTreeSet<crate::EntryId>>,
}
// sirno:witness:generated-footer:end

impl GeneratedLinkIndex {
    /// Construct a generated-link index from parsed entries.
    // sirno:witness:generated-footer:begin
    pub fn from_entries(entries: &[Entry]) -> Self {
        let mut category_sources_by_target =
            BTreeMap::<crate::EntryId, BTreeSet<crate::EntryId>>::new();
        let mut cliques_by_belongs_target =
            BTreeMap::<crate::EntryId, BTreeSet<crate::EntryId>>::new();
        let mut belongs_sources_by_target =
            BTreeMap::<crate::EntryId, BTreeSet<crate::EntryId>>::new();
        let mut refines_sources_by_target =
            BTreeMap::<crate::EntryId, BTreeSet<crate::EntryId>>::new();
        for entry in entries {
            Self::insert_sources(
                &mut category_sources_by_target,
                &entry.id,
                &entry.metadata.category,
            );
            Self::insert_sources(
                &mut belongs_sources_by_target,
                &entry.id,
                &entry.metadata.belongs,
            );
            Self::insert_sources(
                &mut refines_sources_by_target,
                &entry.id,
                &entry.metadata.refines,
            );
            // sirno:witness:generated-footer:end
            // sirno:witness:belongs:begin
            for target in &entry.metadata.belongs {
                let clique = cliques_by_belongs_target.entry(target.clone()).or_default();
                clique.insert(target.clone());
                clique.insert(entry.id.clone());
            }
            // sirno:witness:belongs:end
            // sirno:witness:generated-footer:begin
        }
        Self {
            category_sources_by_target,
            cliques_by_belongs_target,
            belongs_sources_by_target,
            refines_sources_by_target,
        }
    }
    // sirno:witness:generated-footer:end

    /// Render the generated-link footer for one entry using this lake-wide index.
    pub fn render_entry(&self, entry: &Entry, settings: &GeneratedLinkSettings) -> String {
        // sirno:witness:generated-footer:begin
        let mut out = String::new();
        out.push_str(BEGIN_LINKS_GUARD);
        out.push_str("\n\n");
        // sirno:witness:generated-footer:end

        // sirno:witness:generated-footer:begin
        let mut sections = Vec::new();
        if settings.category.from {
            sections.push(GeneratedLinkSection::new(
                "Category (from)",
                self.incoming_targets(&self.category_sources_by_target, entry),
            ));
        }
        if settings.category.to {
            sections.push(GeneratedLinkSection::new(
                "Category (to)",
                entry.metadata.category.iter().cloned().collect(),
            ));
        }
        if settings.belongs.from {
            sections.push(GeneratedLinkSection::new(
                "Belongs (from)",
                self.incoming_targets(&self.belongs_sources_by_target, entry),
            ));
        }
        if settings.belongs.to {
            sections.push(GeneratedLinkSection::new(
                "Belongs (to)",
                entry.metadata.belongs.iter().cloned().collect(),
            ));
        }
        if settings.clique {
            sections.push(GeneratedLinkSection::new("Clique", self.clique_targets(entry)));
        }
        if settings.refines.from {
            sections.push(GeneratedLinkSection::new(
                "Refines (from)",
                self.incoming_targets(&self.refines_sources_by_target, entry),
            ));
        }
        if settings.refines.to {
            sections.push(GeneratedLinkSection::new(
                "Refines (to)",
                entry.metadata.refines.iter().cloned().collect(),
            ));
        }
        // sirno:witness:generated-footer:end

        // sirno:witness:generated-footer:begin
        if sections.is_empty() {
            out.push_str("(none)\n\n");
        } else {
            for section in &sections {
                section.render(&mut out);
            }
        }

        out.push_str(END_LINKS_GUARD);
        out
        // sirno:witness:generated-footer:end
    }

    fn insert_sources(
        sources_by_target: &mut BTreeMap<crate::EntryId, BTreeSet<crate::EntryId>>,
        source: &crate::EntryId, targets: &[crate::EntryId],
    ) {
        for target in targets {
            sources_by_target.entry(target.clone()).or_default().insert(source.clone());
        }
    }

    fn incoming_targets(
        &self, sources_by_target: &BTreeMap<crate::EntryId, BTreeSet<crate::EntryId>>,
        entry: &Entry,
    ) -> BTreeSet<crate::EntryId> {
        sources_by_target.get(&entry.id).cloned().unwrap_or_default()
    }

    // sirno:witness:belongs:begin
    fn clique_targets(&self, entry: &Entry) -> BTreeSet<crate::EntryId> {
        let mut targets = BTreeSet::new();
        for target in &entry.metadata.belongs {
            if let Some(clique) = self.cliques_by_belongs_target.get(target) {
                targets.extend(clique.iter().filter(|id| *id != &entry.id).cloned());
            }
        }
        if let Some(clique) = self.cliques_by_belongs_target.get(&entry.id) {
            targets.extend(clique.iter().filter(|id| *id != &entry.id).cloned());
        }
        targets
    }
    // sirno:witness:belongs:end
}

#[derive(Debug)]
struct GeneratedLinkSection {
    title: &'static str,
    targets: BTreeSet<crate::EntryId>,
}

impl GeneratedLinkSection {
    fn new(title: &'static str, targets: BTreeSet<crate::EntryId>) -> Self {
        Self { title, targets }
    }

    // sirno:witness:generated-footer:begin
    fn render(&self, out: &mut String) {
        if self.targets.is_empty() {
            out.push_str(self.title);
            out.push_str(": (none)\n\n");
            return;
        }

        out.push_str(self.title);
        out.push(':');
        out.push('\n');
        for id in &self.targets {
            out.push_str("- ");
            out.push_str(&format!("[{}]({}.md)", id.as_str(), id.as_str()));
            out.push('\n');
        }
        out.push('\n');
    }
    // sirno:witness:generated-footer:end
}

/// Opening guard for Sirno-owned generated links.
// sirno:witness:generated-footer-ownership:begin
pub const BEGIN_LINKS_GUARD: &str = "> **Sirno generated links begin. Do not edit this section.**";
/// Closing guard for Sirno-owned generated links.
pub const END_LINKS_GUARD: &str = "> **Sirno generated links end.**";
// sirno:witness:generated-footer-ownership:end

const GENERATED_LINK_DIVIDER: &str = "---";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct GeneratedLinkBounds {
    region_start: usize,
    region_end: usize,
}

impl GeneratedLinkBounds {
    // sirno:witness:generated-footer-ownership:begin
    fn find(body: &str) -> Result<Option<Self>, GeneratedLinkError> {
        let begin = Self::guard_positions(body, BEGIN_LINKS_GUARD);
        let end = Self::guard_positions(body, END_LINKS_GUARD);
        let bounds = match (begin.as_slice(), end.as_slice()) {
            | ([], []) => Ok(()),
            | ([begin], [end]) if begin < end => Ok(()),
            | ([begin], [end]) if begin > end => Err(GeneratedLinkError::EndBeforeBegin),
            | ([], [_]) => Err(GeneratedLinkError::MissingBegin),
            | ([_], []) => Err(GeneratedLinkError::MissingEnd),
            | (_, _) if begin.len() > 1 => Err(GeneratedLinkError::DuplicateBegin),
            | (_, _) if end.len() > 1 => Err(GeneratedLinkError::DuplicateEnd),
            | _ => Err(GeneratedLinkError::Malformed),
        };
        bounds?;

        if begin.is_empty() {
            return Ok(None);
        }

        let begin = begin[0];
        let end = end[0] + END_LINKS_GUARD.len();
        Ok(Some(Self { region_start: Self::line_start(body, begin), region_end: end }))
    }
    // sirno:witness:generated-footer-ownership:end

    fn next_line_start(self, body: &str) -> usize {
        body[self.region_end..]
            .find('\n')
            .map_or(body.len(), |position| self.region_end + position + 1)
    }

    fn guard_positions(body: &str, guard: &str) -> Vec<usize> {
        body.match_indices(guard).map(|(index, _)| index).collect()
    }

    fn line_start(body: &str, index: usize) -> usize {
        body[..index].rfind('\n').map_or(0, |position| position + 1)
    }
}

/// Error raised by generated-link footer handling.
#[derive(Debug, Error, PartialEq, Eq)]
// sirno:witness:generated-footer:begin
pub enum GeneratedLinkError {
    /// A closing guard appears without an opening guard.
    #[error("generated-link footer is missing its opening guard")]
    MissingBegin,
    /// An opening guard appears without a closing guard.
    #[error("generated-link footer is missing its closing guard")]
    MissingEnd,
    /// More than one opening guard appears.
    #[error("generated-link footer has duplicate opening guards")]
    DuplicateBegin,
    /// More than one closing guard appears.
    #[error("generated-link footer has duplicate closing guards")]
    DuplicateEnd,
    /// The closing guard appears before the opening guard.
    #[error("generated-link footer closing guard appears before opening guard")]
    EndBeforeBegin,
    /// The generated-link guard state is malformed.
    #[error("generated-link footer boundaries are malformed")]
    Malformed,
}
// sirno:witness:generated-footer:end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Entry, EntryId, EntryMetadata};

    fn id(raw: &str) -> EntryId {
        EntryId::new(raw).unwrap()
    }

    fn entry() -> Entry {
        let mut metadata = EntryMetadata::new("Concept", "A named idea.").unwrap();
        metadata.category.push(id("meta"));
        metadata.belongs.push(id("core"));
        metadata.refines.push(id("metadata"));
        Entry::new(id("concept"), metadata, "Body.\n")
    }

    #[test]
    fn default_settings_render_only_belongs_links() {
        let footer = GeneratedLinkSettings::default().render_entry(&entry());

        assert!(!footer.contains("[meta](meta.md)"));
        assert!(footer.contains("- [core](core.md)"));
        assert!(!footer.contains("[metadata](metadata.md)"));
        assert!(!footer.contains("## Sirno Links"));
        assert!(footer.contains("Belongs (from): (none)"));
        assert!(footer.contains("Belongs (to):\n- [core](core.md)"));
        assert!(footer.contains(BEGIN_LINKS_GUARD));
        assert!(footer.contains(END_LINKS_GUARD));
        assert!(footer.contains("> **Sirno generated links begin."));
    }

    #[test]
    fn quoted_guards_are_separated_from_link_list() {
        let footer = GeneratedLinkSettings::default().render_entry(&entry());

        assert!(footer.contains(&format!(
            "{BEGIN_LINKS_GUARD}\n\nBelongs (from): (none)\n\nBelongs (to):\n"
        )));
        assert!(footer.contains(&format!("- [core](core.md)\n\n{END_LINKS_GUARD}")));
    }

    #[test]
    fn settings_can_enable_each_structural_field() {
        let settings = GeneratedLinkSettings {
            category: true.into(),
            belongs: true.into(),
            clique: false,
            refines: true.into(),
        };
        let footer = settings.render_entry(&entry());

        assert!(footer.contains("- [meta](meta.md)"));
        assert!(footer.contains("- [core](core.md)"));
        assert!(footer.contains("- [metadata](metadata.md)"));
        assert!(footer.contains("Category (from): (none)"));
        assert!(footer.contains("Category (to):"));
        assert!(footer.contains("Belongs (from): (none)"));
        assert!(footer.contains("Belongs (to):"));
        assert!(footer.contains("Refines (from): (none)"));
        assert!(footer.contains("Refines (to):"));
    }

    #[test]
    fn repeated_targets_render_once() {
        let mut entry = entry();
        entry.metadata.category.push(id("meta"));
        let settings = GeneratedLinkSettings {
            category: true.into(),
            belongs: false.into(),
            clique: false,
            refines: false.into(),
        };

        let footer = settings.render_entry(&entry);

        assert_eq!(footer.matches("[meta](meta.md)").count(), 1);
    }

    #[test]
    fn boolean_field_settings_render_to_and_from_edges() {
        let settings = GeneratedLinkSettings {
            category: true.into(),
            belongs: false.into(),
            clique: false,
            refines: false.into(),
        };
        let category =
            Entry::new(id("meta"), EntryMetadata::new("Meta", "A category.").unwrap(), "Body.\n");
        let mut member_metadata = EntryMetadata::new("Member", "A category member.").unwrap();
        member_metadata.category.push(id("meta"));
        let member = Entry::new(id("member"), member_metadata, "Body.\n");
        let entries = vec![category.clone(), member.clone()];
        let index = GeneratedLinkIndex::from_entries(&entries);

        let category_footer = index.render_entry(&category, &settings);
        let member_footer = index.render_entry(&member, &settings);

        assert!(category_footer.contains("Category (from):"));
        assert!(category_footer.contains("- [member](member.md)"));
        assert!(category_footer.contains("Category (to): (none)"));
        assert!(member_footer.contains("Category (from): (none)"));
        assert!(member_footer.contains("Category (to):"));
        assert!(member_footer.contains("- [meta](meta.md)"));
    }

    #[test]
    fn table_field_settings_can_choose_one_side() {
        let settings = GeneratedLinkSettings {
            category: GeneratedLinkFieldSettings::new(false, true),
            belongs: false.into(),
            clique: false,
            refines: false.into(),
        };
        let category =
            Entry::new(id("meta"), EntryMetadata::new("Meta", "A category.").unwrap(), "Body.\n");
        let mut member_metadata = EntryMetadata::new("Member", "A category member.").unwrap();
        member_metadata.category.push(id("meta"));
        let member = Entry::new(id("member"), member_metadata, "Body.\n");
        let entries = vec![category.clone(), member.clone()];
        let index = GeneratedLinkIndex::from_entries(&entries);

        let category_footer = index.render_entry(&category, &settings);
        let member_footer = index.render_entry(&member, &settings);

        assert!(category_footer.contains("Category (from):"));
        assert!(category_footer.contains("- [member](member.md)"));
        assert!(member_footer.contains("Category (from): (none)"));
        assert!(!member_footer.contains("[meta](meta.md)"));
    }

    #[test]
    fn clique_setting_expands_belongs_targets_to_edges() {
        let settings = GeneratedLinkSettings {
            category: false.into(),
            belongs: false.into(),
            clique: true,
            refines: false.into(),
        };

        let closure = Entry::new(
            id("core"),
            EntryMetadata::new("Core", "A review neighborhood.").unwrap(),
            "Body.\n",
        );
        let mut left_metadata = EntryMetadata::new("Left", "A neighborhood member.").unwrap();
        left_metadata.belongs.push(id("core"));
        let left = Entry::new(id("left"), left_metadata, "Body.\n");
        let mut right_metadata = EntryMetadata::new("Right", "A neighborhood member.").unwrap();
        right_metadata.belongs.push(id("core"));
        let right = Entry::new(id("right"), right_metadata, "Body.\n");
        let mut outside_metadata = EntryMetadata::new("Outside", "Another member.").unwrap();
        outside_metadata.belongs.push(id("other"));
        let outside = Entry::new(id("outside"), outside_metadata, "Body.\n");
        let entries = vec![closure.clone(), left.clone(), right.clone(), outside];
        let index = GeneratedLinkIndex::from_entries(&entries);

        let closure_footer = index.render_entry(&closure, &settings);
        let left_footer = index.render_entry(&left, &settings);

        assert!(closure_footer.contains("Clique:"));
        assert!(!closure_footer.contains("Belongs (from)"));
        assert!(closure_footer.contains("- [left](left.md)"));
        assert!(closure_footer.contains("- [right](right.md)"));
        assert!(!closure_footer.contains("[core](core.md)"));
        assert!(!closure_footer.contains("[outside](outside.md)"));
        assert!(left_footer.contains("Clique:"));
        assert!(!left_footer.contains("Belongs (to)"));
        assert!(left_footer.contains("- [core](core.md)"));
        assert!(left_footer.contains("- [right](right.md)"));
        assert!(!left_footer.contains("[left](left.md)"));
        assert!(!left_footer.contains("[outside](outside.md)"));
    }

    #[test]
    fn belongs_sections_remain_direct_when_clique_is_enabled() {
        let settings = GeneratedLinkSettings {
            category: false.into(),
            belongs: true.into(),
            clique: true,
            refines: false.into(),
        };

        let closure = Entry::new(
            id("core"),
            EntryMetadata::new("Core", "A review neighborhood.").unwrap(),
            "Body.\n",
        );
        let mut left_metadata = EntryMetadata::new("Left", "A neighborhood member.").unwrap();
        left_metadata.belongs.push(id("core"));
        let left = Entry::new(id("left"), left_metadata, "Body.\n");
        let mut right_metadata = EntryMetadata::new("Right", "A neighborhood member.").unwrap();
        right_metadata.belongs.push(id("core"));
        let right = Entry::new(id("right"), right_metadata, "Body.\n");
        let entries = vec![closure, left.clone(), right];
        let index = GeneratedLinkIndex::from_entries(&entries);

        let left_footer = index.render_entry(&left, &settings);

        assert!(left_footer.contains("Belongs (to):\n- [core](core.md)"));
        assert!(left_footer.contains("Clique:"));
        assert!(left_footer.contains("- [right](right.md)"));
    }

    #[test]
    fn renders_empty_enabled_sections_when_entry_has_no_structural_targets() {
        let metadata = EntryMetadata::new("Meta", "A category.").unwrap();
        let entry = Entry::new(EntryId::new("meta").unwrap(), metadata, "Body.\n");

        let footer = GeneratedLinkSettings::default().render_entry(&entry);

        assert!(footer.contains("Belongs (from): (none)"));
        assert!(footer.contains("Belongs (to): (none)"));
        assert!(!footer.contains(&format!("{BEGIN_LINKS_GUARD}\n\n(none)\n\n{END_LINKS_GUARD}")));
        assert!(!footer.contains("- none"));
    }

    #[test]
    fn renders_region_none_when_no_sections_are_enabled() {
        let metadata = EntryMetadata::new("Meta", "A category.").unwrap();
        let entry = Entry::new(EntryId::new("meta").unwrap(), metadata, "Body.\n");
        let settings = GeneratedLinkSettings {
            category: false.into(),
            belongs: false.into(),
            clique: false,
            refines: false.into(),
        };

        let footer = settings.render_entry(&entry);

        assert_eq!(footer, format!("{BEGIN_LINKS_GUARD}\n\n(none)\n\n{END_LINKS_GUARD}"));
        assert!(!footer.contains("- none"));
    }

    #[test]
    fn appends_footer_when_missing() {
        let footer = GeneratedLinkSettings::default().render_entry(&entry());

        let body = GeneratedLinkBody::new("Body.\n").apply(&footer).unwrap();

        assert_eq!(body, format!("Body.\n\n---\n\n{footer}\n"));
        assert_eq!(body.matches(BEGIN_LINKS_GUARD).count(), 1);
    }

    #[test]
    fn appends_footer_without_duplicate_divider() {
        let footer = GeneratedLinkSettings::default().render_entry(&entry());

        let body = GeneratedLinkBody::new("Body.\n\n---\n").apply(&footer).unwrap();

        assert_eq!(body, format!("Body.\n\n---\n\n{footer}\n"));
    }

    #[test]
    fn replaces_only_existing_footer_region() {
        let old = format!("{BEGIN_LINKS_GUARD}\nold\n{END_LINKS_GUARD}\n");
        let body = format!("Before.\n\n{old}\nAfter.\n");
        let footer = GeneratedLinkSettings::default().render_entry(&entry());

        let body = GeneratedLinkBody::new(&body).apply(&footer).unwrap();

        assert!(body.starts_with("Before.\n\n"));
        assert!(body.ends_with("After.\n"));
        assert!(!body.contains("old"));
        assert_eq!(body.matches(BEGIN_LINKS_GUARD).count(), 1);
    }

    #[test]
    fn deletes_existing_footer_region() {
        let footer = GeneratedLinkSettings::default().render_entry(&entry());
        let body = GeneratedLinkBody::new("Body.\n").apply(&footer).unwrap();

        let body = GeneratedLinkBody::new(&body).delete().unwrap();

        assert_eq!(body, "Body.\n\n---\n");
        assert!(!body.contains(BEGIN_LINKS_GUARD));
    }

    #[test]
    fn deletes_footer_without_touching_following_body() {
        let footer = GeneratedLinkSettings::default().render_entry(&entry());
        let body = format!("Before.\n\n{footer}\nAfter.\n");

        let body = GeneratedLinkBody::new(&body).delete().unwrap();

        assert_eq!(body, "Before.\n\nAfter.\n");
    }

    #[test]
    fn delete_is_noop_when_footer_is_missing() {
        let body = GeneratedLinkBody::new("Body.\n").delete().unwrap();

        assert_eq!(body, "Body.\n");
    }

    #[test]
    fn reports_generated_links_staleness() {
        let expected = GeneratedLinkSettings::default().render_entry(&entry());
        let current = GeneratedLinkBody::new("Body.\n").apply(&expected).unwrap();
        let stale = GeneratedLinkBody::new("Body.\n")
            .apply(
                &GeneratedLinkSettings {
                    category: true.into(),
                    belongs: true.into(),
                    clique: false,
                    refines: true.into(),
                }
                .render_entry(&entry()),
            )
            .unwrap();

        assert!(!GeneratedLinkBody::new("Body.\n").is_stale(&expected).unwrap());
        assert!(!GeneratedLinkBody::new(&current).is_stale(&expected).unwrap());
        assert!(GeneratedLinkBody::new(&stale).is_stale(&expected).unwrap());
    }

    #[test]
    fn rejects_missing_end_guard() {
        let error = GeneratedLinkBody::new(BEGIN_LINKS_GUARD).validate().unwrap_err();

        assert_eq!(error, GeneratedLinkError::MissingEnd);
    }

    #[test]
    fn rejects_duplicate_begin_guard() {
        let body = format!("{BEGIN_LINKS_GUARD}\n{BEGIN_LINKS_GUARD}\n{END_LINKS_GUARD}\n");
        let error = GeneratedLinkBody::new(&body).validate().unwrap_err();

        assert_eq!(error, GeneratedLinkError::DuplicateBegin);
    }
}
