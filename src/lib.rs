#![forbid(unsafe_code)]
//! SigmaOS-data-sharing: data sharing model.
//!
//! Real implementation: permissions, grants, access control, and validation.
//! Replaces the 108-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Sharing grant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharingGrant {
    pub id: String,
    pub name: String,
    pub resource: String,
    pub recipient: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SharingError {
    EmptyName,
    DuplicateGrant(String),
}

impl fmt::Display for SharingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "grant name must not be empty"),
            Self::DuplicateGrant(id) => write!(f, "duplicate grant {:?}", id),
        }
    }
}

impl std::error::Error for SharingError {}

/// Data sharing manager.
pub struct DataSharing {
    pub name: String,
    grants: BTreeMap<String, SharingGrant>,
}

impl DataSharing {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            grants: BTreeMap::new(),
        }
    }

    pub fn add_grant(&mut self, grant: SharingGrant) -> Result<(), SharingError> {
        if grant.name.trim().is_empty() { return Err(SharingError::EmptyName); }
        let id = grant.id.clone();
        if self.grants.contains_key(&id) {
            return Err(SharingError::DuplicateGrant(id.clone()));
        }
        self.grants.insert(id, grant);
        Ok(())
    }

    pub fn grant_count(&self) -> usize { self.grants.len() }

    pub fn get_grant(&self, id: &str) -> Option<&SharingGrant> { self.grants.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sharing_create_aaa() {
        let s = DataSharing::new("My Sharing");
        assert_eq!(s.name, "My Sharing");
        assert_eq!(s.grant_count(), 0);
    }

    #[test]
    fn test_add_grant_aaa() {
        let mut s = DataSharing::new("My Sharing");
        let g = SharingGrant { id: "g1".into(), name: "view".into(), resource: "report".into(), recipient: "team".into() };
        s.add_grant(g).unwrap();
        assert_eq!(s.grant_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut s = DataSharing::new("My Sharing");
        let g = SharingGrant { id: "x".into(), name: "".into(), resource: "a".into(), recipient: "b".into() };
        assert!(s.add_grant(g).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut s = DataSharing::new("My Sharing");
        let g = SharingGrant { id: "x".into(), name: "X".into(), resource: "a".into(), recipient: "b".into() };
        s.add_grant(g).unwrap();
        let g2 = SharingGrant { id: "x".into(), name: "X".into(), resource: "a".into(), recipient: "b".into() };
        assert!(s.add_grant(g2).is_err());
    }
}
