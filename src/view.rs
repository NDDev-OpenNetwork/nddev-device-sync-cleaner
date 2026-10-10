//! Portable native-provider response projections. No I/O or UI runtime.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum CheckState {
    Ok,
    Warning,
    Error,
    Info,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Check {
    pub id: String,
    pub state: CheckState,
    pub detail: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Diagnostics {
    pub tool: String,
    pub version: String,
    pub checked_unix: u64,
    pub errors: usize,
    pub warnings: usize,
    pub checks: Vec<Check>,
}

/// Summary of a provider-owned run. Inventory paths and raw native output are
/// intentionally not part of this view; no reclaimed bytes are inferred.
#[derive(Clone, Deserialize, Serialize)]
pub struct CleanerRunSummary {
    pub tool: String,
    pub version: String,
    pub started_unix: u64,
    pub duration_ms: u64,
    pub dry_run: bool,
    #[serde(deserialize_with = "Option::deserialize")]
    pub freed_bytes: Option<u64>,
    pub deleted: usize,
    pub skipped: usize,
    pub failed: usize,
}
