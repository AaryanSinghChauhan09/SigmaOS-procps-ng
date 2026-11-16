// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-procps-ng: procps ng component model.
//!
//! Real implementation: procps_ng_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcpsNgItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcpsNgError {
    EmptyProcpsNgItemName,
    DuplicateProcpsNgItem(String),
    EmptyEventSubject,
}

impl fmt::Display for ProcpsNgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyProcpsNgItemName => write!(f, "procps_ng_item name must not be empty"),
            Self::DuplicateProcpsNgItem(id) => write!(f, "duplicate procps_ng_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ProcpsNgError {}

pub struct ProcpsNg {
    pub name: String,
    procps_ng_items: BTreeMap<String, ProcpsNgItem>,
    events: BTreeMap<String, Event>,
}

impl ProcpsNg {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            procps_ng_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_procps_ng_item(&mut self, item: ProcpsNgItem) -> Result<(), ProcpsNgError> {
        if item.name.trim().is_empty() {
            return Err(ProcpsNgError::EmptyProcpsNgItemName);
        }
        let id = item.id.clone();
        if self.procps_ng_items.contains_key(&id) {
            return Err(ProcpsNgError::DuplicateProcpsNgItem(id.clone()));
        }
        self.procps_ng_items.insert(id, item);
        Ok(())
    }

    pub fn procps_ng_item_count(&self) -> usize {
        self.procps_ng_items.len()
    }

    pub fn get_procps_ng_item(&self, id: &str) -> Option<&ProcpsNgItem> {
        self.procps_ng_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ProcpsNgError> {
        if event.subject.trim().is_empty() {
            return Err(ProcpsNgError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_procps_ng_item(id: &str, name: &str) -> ProcpsNgItem {
        ProcpsNgItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = ProcpsNg::new("ProcpsNg");
        assert_eq!(m.name, "ProcpsNg");
        assert_eq!(m.procps_ng_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_procps_ng_item() {
        let mut m = ProcpsNg::new("ProcpsNg");
        m.add_procps_ng_item(sample_procps_ng_item("a1", "Alpha")).unwrap();
        assert_eq!(m.procps_ng_item_count(), 1);
        assert!(m.get_procps_ng_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = ProcpsNg::new("ProcpsNg");
        assert!(m.add_procps_ng_item(sample_procps_ng_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = ProcpsNg::new("ProcpsNg");
        m.add_procps_ng_item(sample_procps_ng_item("x", "One")).unwrap();
        assert!(m.add_procps_ng_item(sample_procps_ng_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = ProcpsNg::new("ProcpsNg");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
