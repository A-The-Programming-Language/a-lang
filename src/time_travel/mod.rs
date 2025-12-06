//! # Time-Travel Debugging System (WOW Factor #1)
//!
//! This module implements A-lang's revolutionary time-travel debugging feature.
//! It allows developers to:
//! - Take snapshots of program state at any point
//! - Rewind execution to previous states
//! - Replay execution from checkpoints
//! - Inspect historical state changes
//!
//! Implementation uses persistent data structures for efficient state storage.

use crate::interpreter::value::Value;
use chrono::{DateTime, Utc};
use im::HashMap as PersistentHashMap;
use std::collections::VecDeque;
use std::fmt;

/// Maximum number of snapshots to keep in memory (configurable)
const MAX_SNAPSHOTS: usize = 1000;

/// A snapshot of the program state at a specific point in time
#[derive(Debug, Clone)]
pub struct Snapshot {
    /// Unique identifier for this snapshot
    pub id: usize,

    /// Optional label for named checkpoints
    pub label: Option<String>,

    /// Timestamp when the snapshot was taken
    pub timestamp: DateTime<Utc>,

    /// The program state (variables and their values)
    pub state: PersistentHashMap<String, Value>,

    /// Call stack at the time of snapshot
    pub call_stack: Vec<StackFrame>,

    /// Line number in source code
    pub line: usize,

    /// Source file name
    pub file: String,

    /// Additional metadata
    pub metadata: SnapshotMetadata,
}

/// Stack frame information
#[derive(Debug, Clone)]
pub struct StackFrame {
    pub function_name: String,
    pub line: usize,
    pub locals: PersistentHashMap<String, Value>,
}

/// Metadata associated with a snapshot
#[derive(Debug, Clone)]
pub struct SnapshotMetadata {
    /// Memory usage at snapshot time (in bytes)
    pub memory_usage: usize,

    /// Number of operations since last snapshot
    pub operations_count: usize,

    /// Custom tags for filtering
    pub tags: Vec<String>,
}

/// The Time-Travel Debugger
pub struct TimeTravelDebugger {
    /// All snapshots in chronological order
    snapshots: VecDeque<Snapshot>,

    /// Current position in the snapshot history
    current_index: usize,

    /// Named checkpoints for quick access
    checkpoints: std::collections::HashMap<String, usize>,

    /// Whether time-travel is enabled
    enabled: bool,

    /// Configuration
    config: TimeTravelConfig,

    /// Counter for generating snapshot IDs
    next_id: usize,
}

/// Configuration for time-travel debugging
#[derive(Debug, Clone)]
pub struct TimeTravelConfig {
    /// Maximum snapshots to keep
    pub max_snapshots: usize,

    /// Auto-snapshot interval (take snapshot every N operations)
    pub auto_snapshot_interval: Option<usize>,

    /// Enable compression for old snapshots
    pub compress_old_snapshots: bool,

    /// Minimum time between auto-snapshots (milliseconds)
    pub min_snapshot_interval_ms: u64,
}

impl Default for TimeTravelConfig {
    fn default() -> Self {
        Self {
            max_snapshots: MAX_SNAPSHOTS,
            auto_snapshot_interval: Some(100),
            compress_old_snapshots: true,
            min_snapshot_interval_ms: 10,
        }
    }
}

impl TimeTravelDebugger {
    /// Create a new time-travel debugger
    pub fn new(config: TimeTravelConfig) -> Self {
        Self {
            snapshots: VecDeque::new(),
            current_index: 0,
            checkpoints: std::collections::HashMap::new(),
            enabled: true,
            config,
            next_id: 0,
        }
    }

    /// Create with default configuration
    pub fn default() -> Self {
        Self::new(TimeTravelConfig::default())
    }

    /// Enable or disable time-travel debugging
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Take a snapshot of the current state
    pub fn snapshot(
        &mut self,
        state: PersistentHashMap<String, Value>,
        call_stack: Vec<StackFrame>,
        line: usize,
        file: String,
        label: Option<String>,
    ) -> Result<usize, TimeTravelError> {
        if !self.enabled {
            return Err(TimeTravelError::Disabled);
        }

        // Remove snapshots ahead of current position (branching timeline)
        while self.current_index < self.snapshots.len() {
            self.snapshots.pop_back();
        }

        // Enforce max snapshots limit
        if self.snapshots.len() >= self.config.max_snapshots {
            self.snapshots.pop_front();
            if self.current_index > 0 {
                self.current_index -= 1;
            }
        }

        let snapshot = Snapshot {
            id: self.next_id,
            label: label.clone(),
            timestamp: Utc::now(),
            state,
            call_stack,
            line,
            file,
            metadata: SnapshotMetadata {
                memory_usage: 0, // TODO: Calculate actual memory usage
                operations_count: 0,
                tags: Vec::new(),
            },
        };

        let id = snapshot.id;
        self.snapshots.push_back(snapshot);
        self.current_index = self.snapshots.len();
        self.next_id += 1;

        // Register checkpoint if labeled
        if let Some(ref label) = label {
            self.checkpoints
                .insert(label.clone(), self.current_index - 1);
        }

        Ok(id)
    }

    /// Rewind to a previous state by N steps
    pub fn rewind(&mut self, steps: usize) -> Result<&Snapshot, TimeTravelError> {
        if !self.enabled {
            return Err(TimeTravelError::Disabled);
        }

        if self.snapshots.is_empty() {
            return Err(TimeTravelError::NoSnapshots);
        }

        if steps > self.current_index {
            return Err(TimeTravelError::InvalidStep {
                requested: steps,
                available: self.current_index,
            });
        }

        self.current_index = self.current_index.saturating_sub(steps);

        self.snapshots
            .get(self.current_index)
            .ok_or(TimeTravelError::SnapshotNotFound)
    }

    /// Fast-forward by N steps
    pub fn forward(&mut self, steps: usize) -> Result<&Snapshot, TimeTravelError> {
        if !self.enabled {
            return Err(TimeTravelError::Disabled);
        }

        if self.snapshots.is_empty() {
            return Err(TimeTravelError::NoSnapshots);
        }

        let new_index = self.current_index + steps;
        if new_index >= self.snapshots.len() {
            return Err(TimeTravelError::InvalidStep {
                requested: steps,
                available: self.snapshots.len() - self.current_index - 1,
            });
        }

        self.current_index = new_index;

        self.snapshots
            .get(self.current_index)
            .ok_or(TimeTravelError::SnapshotNotFound)
    }

    /// Jump to a specific checkpoint by label
    pub fn jump_to_checkpoint(&mut self, label: &str) -> Result<&Snapshot, TimeTravelError> {
        if !self.enabled {
            return Err(TimeTravelError::Disabled);
        }

        let index = self
            .checkpoints
            .get(label)
            .ok_or_else(|| TimeTravelError::CheckpointNotFound(label.to_string()))?;

        self.current_index = *index;

        self.snapshots
            .get(self.current_index)
            .ok_or(TimeTravelError::SnapshotNotFound)
    }

    /// Get the current snapshot
    pub fn current_snapshot(&self) -> Result<&Snapshot, TimeTravelError> {
        if self.snapshots.is_empty() {
            return Err(TimeTravelError::NoSnapshots);
        }

        self.snapshots
            .get(self.current_index)
            .ok_or(TimeTravelError::SnapshotNotFound)
    }

    /// Get all snapshots
    pub fn all_snapshots(&self) -> Vec<&Snapshot> {
        self.snapshots.iter().collect()
    }

    /// Get snapshots within a time range
    pub fn snapshots_in_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<&Snapshot> {
        self.snapshots
            .iter()
            .filter(|s| s.timestamp >= start && s.timestamp <= end)
            .collect()
    }

    /// Get the difference between two snapshots
    pub fn diff(&self, from_id: usize, to_id: usize) -> Result<StateDiff, TimeTravelError> {
        let from_snapshot = self
            .snapshots
            .iter()
            .find(|s| s.id == from_id)
            .ok_or(TimeTravelError::SnapshotNotFound)?;

        let to_snapshot = self
            .snapshots
            .iter()
            .find(|s| s.id == to_id)
            .ok_or(TimeTravelError::SnapshotNotFound)?;

        Ok(StateDiff::compute(&from_snapshot.state, &to_snapshot.state))
    }

    /// Clear all snapshots
    pub fn clear(&mut self) {
        self.snapshots.clear();
        self.checkpoints.clear();
        self.current_index = 0;
    }

    /// Get statistics about the debugger
    pub fn stats(&self) -> TimeTravelStats {
        TimeTravelStats {
            total_snapshots: self.snapshots.len(),
            current_position: self.current_index,
            checkpoints_count: self.checkpoints.len(),
            memory_usage: self.estimate_memory_usage(),
        }
    }

    /// Estimate memory usage of all snapshots
    fn estimate_memory_usage(&self) -> usize {
        // Simplified estimation
        self.snapshots.len() * std::mem::size_of::<Snapshot>()
    }

    /// Export snapshots to JSON for analysis (placeholder)
    pub fn export_to_json(&self) -> Result<String, TimeTravelError> {
        // TODO: Implement proper serialization
        Ok(format!("{{\"snapshots\": {}}}", self.snapshots.len()))
    }

    /// Import snapshots from JSON (placeholder)
    pub fn import_from_json(&mut self, _json: &str) -> Result<(), TimeTravelError> {
        // TODO: Implement proper deserialization
        Err(TimeTravelError::SerializationError(
            "Not yet implemented".to_string(),
        ))
    }

    /// Replay from a snapshot, executing operations forward
    pub fn replay_from(&mut self, snapshot_id: usize) -> Result<ReplaySession, TimeTravelError> {
        let index = self
            .snapshots
            .iter()
            .position(|s| s.id == snapshot_id)
            .ok_or(TimeTravelError::SnapshotNotFound)?;

        Ok(ReplaySession {
            start_index: index,
            current_index: index,
            debugger: self,
        })
    }
}

/// Difference between two states
#[derive(Debug, Clone)]
pub struct StateDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub modified: Vec<(String, Value, Value)>, // (name, old_value, new_value)
}

impl StateDiff {
    pub fn compute(
        old_state: &PersistentHashMap<String, Value>,
        new_state: &PersistentHashMap<String, Value>,
    ) -> Self {
        let mut added = Vec::new();
        let mut removed = Vec::new();
        let mut modified = Vec::new();

        // Find added and modified
        for (key, new_value) in new_state.iter() {
            match old_state.get(key) {
                None => added.push(key.clone()),
                Some(old_value) if old_value != new_value => {
                    modified.push((key.clone(), old_value.clone(), new_value.clone()));
                }
                _ => {}
            }
        }

        // Find removed
        for key in old_state.keys() {
            if !new_state.contains_key(key) {
                removed.push(key.clone());
            }
        }

        Self {
            added,
            removed,
            modified,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.modified.is_empty()
    }
}

/// Replay session for stepping through historical execution
pub struct ReplaySession<'a> {
    start_index: usize,
    current_index: usize,
    debugger: &'a mut TimeTravelDebugger,
}

impl<'a> ReplaySession<'a> {
    pub fn step_forward(&mut self) -> Result<&Snapshot, TimeTravelError> {
        if self.current_index >= self.debugger.snapshots.len() - 1 {
            return Err(TimeTravelError::ReplayComplete);
        }

        self.current_index += 1;
        self.debugger
            .snapshots
            .get(self.current_index)
            .ok_or(TimeTravelError::SnapshotNotFound)
    }

    pub fn current_snapshot(&self) -> Result<&Snapshot, TimeTravelError> {
        self.debugger
            .snapshots
            .get(self.current_index)
            .ok_or(TimeTravelError::SnapshotNotFound)
    }

    pub fn is_complete(&self) -> bool {
        self.current_index >= self.debugger.snapshots.len() - 1
    }
}

/// Statistics about time-travel debugging
#[derive(Debug, Clone)]
pub struct TimeTravelStats {
    pub total_snapshots: usize,
    pub current_position: usize,
    pub checkpoints_count: usize,
    pub memory_usage: usize,
}

/// Errors that can occur during time-travel debugging
#[derive(Debug, Clone)]
pub enum TimeTravelError {
    Disabled,
    NoSnapshots,
    SnapshotNotFound,
    InvalidStep { requested: usize, available: usize },
    CheckpointNotFound(String),
    SerializationError(String),
    ReplayComplete,
}

impl fmt::Display for TimeTravelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TimeTravelError::Disabled => write!(f, "Time-travel debugging is disabled"),
            TimeTravelError::NoSnapshots => write!(f, "No snapshots available"),
            TimeTravelError::SnapshotNotFound => write!(f, "Snapshot not found"),
            TimeTravelError::InvalidStep {
                requested,
                available,
            } => {
                write!(
                    f,
                    "Cannot step {} positions, only {} available",
                    requested, available
                )
            }
            TimeTravelError::CheckpointNotFound(label) => {
                write!(f, "Checkpoint '{}' not found", label)
            }
            TimeTravelError::SerializationError(msg) => {
                write!(f, "Serialization error: {}", msg)
            }
            TimeTravelError::ReplayComplete => write!(f, "Replay session is complete"),
        }
    }
}

impl std::error::Error for TimeTravelError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_creation() {
        let mut debugger = TimeTravelDebugger::default();
        let state = PersistentHashMap::new();
        let call_stack = vec![];

        let result = debugger.snapshot(
            state,
            call_stack,
            1,
            "test.al".to_string(),
            Some("test_checkpoint".to_string()),
        );

        assert!(result.is_ok());
        assert_eq!(debugger.snapshots.len(), 1);
    }

    #[test]
    fn test_rewind() {
        let mut debugger = TimeTravelDebugger::default();

        // Create multiple snapshots
        for i in 0..5 {
            let state = PersistentHashMap::new();
            debugger
                .snapshot(state, vec![], i, "test.al".to_string(), None)
                .unwrap();
        }

        // Rewind 2 steps
        let result = debugger.rewind(2);
        assert!(result.is_ok());
        assert_eq!(debugger.current_index, 3);
    }

    #[test]
    fn test_checkpoint() {
        let mut debugger = TimeTravelDebugger::default();

        let state = PersistentHashMap::new();
        debugger
            .snapshot(
                state,
                vec![],
                1,
                "test.al".to_string(),
                Some("important".to_string()),
            )
            .unwrap();

        let result = debugger.jump_to_checkpoint("important");
        assert!(result.is_ok());
    }

    #[test]
    fn test_state_diff() {
        let mut old_state = PersistentHashMap::new();
        old_state.insert("x".to_string(), Value::Integer(10));

        let mut new_state = PersistentHashMap::new();
        new_state.insert("x".to_string(), Value::Integer(20));
        new_state.insert("y".to_string(), Value::Integer(30));

        let diff = StateDiff::compute(&old_state, &new_state);

        assert_eq!(diff.added.len(), 1);
        assert_eq!(diff.modified.len(), 1);
    }
}
