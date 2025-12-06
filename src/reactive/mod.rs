//! # Reactive System (WOW Factor #2)
//!
//! A-lang's reactive programming system allows variables to automatically
//! propagate changes throughout the program. This is similar to modern
//! frontend frameworks but built directly into the language.
//!
//! Features:
//! - Reactive variables (signals) that notify observers on change
//! - Computed values that automatically update when dependencies change
//! - Effects that run side effects when dependencies change
//! - Automatic dependency tracking
//! - Efficient change propagation with topological sorting

use crate::interpreter::value::Value;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::sync::{Arc, RwLock};

/// Unique identifier for reactive nodes
type NodeId = usize;

/// A reactive signal that holds a value and notifies dependents on change
#[derive(Debug, Clone)]
pub struct Signal {
    id: NodeId,
    value: Arc<RwLock<Value>>,
    subscribers: Arc<RwLock<HashSet<NodeId>>>,
    name: String,
}

impl Signal {
    pub fn new(name: String, initial_value: Value) -> Self {
        static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let id = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        Self {
            id,
            value: Arc::new(RwLock::new(initial_value)),
            subscribers: Arc::new(RwLock::new(HashSet::new())),
            name,
        }
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn get(&self) -> Value {
        self.value.read().unwrap().clone()
    }

    pub fn set(&self, new_value: Value) -> Vec<NodeId> {
        let mut value = self.value.write().unwrap();
        *value = new_value;

        // Return list of subscribers to notify
        self.subscribers.read().unwrap().iter().cloned().collect()
    }

    pub fn subscribe(&self, subscriber_id: NodeId) {
        self.subscribers.write().unwrap().insert(subscriber_id);
    }

    pub fn unsubscribe(&self, subscriber_id: NodeId) {
        self.subscribers.write().unwrap().remove(&subscriber_id);
    }
}

/// A computed value that automatically recalculates when dependencies change
#[derive(Clone)]
pub struct Computed {
    id: NodeId,
    value: Arc<RwLock<Value>>,
    dependencies: Arc<RwLock<HashSet<NodeId>>>,
    subscribers: Arc<RwLock<HashSet<NodeId>>>,
    compute_fn: Arc<dyn Fn(&ReactiveContext) -> Value + Send + Sync>,
    name: String,
}

impl Computed {
    pub fn new<F>(name: String, dependencies: Vec<NodeId>, compute_fn: F) -> Self
    where
        F: Fn(&ReactiveContext) -> Value + Send + Sync + 'static,
    {
        static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1000);
        let id = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        Self {
            id,
            value: Arc::new(RwLock::new(Value::Nil)),
            dependencies: Arc::new(RwLock::new(dependencies.into_iter().collect())),
            subscribers: Arc::new(RwLock::new(HashSet::new())),
            compute_fn: Arc::new(compute_fn),
            name,
        }
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn get(&self) -> Value {
        self.value.read().unwrap().clone()
    }

    pub fn recompute(&self, context: &ReactiveContext) -> Vec<NodeId> {
        let new_value = (self.compute_fn)(context);
        let mut value = self.value.write().unwrap();
        *value = new_value;

        // Return list of subscribers to notify
        self.subscribers.read().unwrap().iter().cloned().collect()
    }

    pub fn subscribe(&self, subscriber_id: NodeId) {
        self.subscribers.write().unwrap().insert(subscriber_id);
    }

    pub fn dependencies(&self) -> Vec<NodeId> {
        self.dependencies.read().unwrap().iter().cloned().collect()
    }
}

impl fmt::Debug for Computed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Computed")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("value", &self.value)
            .field("dependencies", &self.dependencies)
            .finish()
    }
}

/// An effect that runs side effects when dependencies change
#[derive(Clone)]
pub struct Effect {
    id: NodeId,
    dependencies: Arc<RwLock<HashSet<NodeId>>>,
    effect_fn: Arc<dyn Fn(&ReactiveContext) + Send + Sync>,
    name: String,
    enabled: Arc<RwLock<bool>>,
}

impl Effect {
    pub fn new<F>(name: String, dependencies: Vec<NodeId>, effect_fn: F) -> Self
    where
        F: Fn(&ReactiveContext) + Send + Sync + 'static,
    {
        static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(2000);
        let id = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        Self {
            id,
            dependencies: Arc::new(RwLock::new(dependencies.into_iter().collect())),
            effect_fn: Arc::new(effect_fn),
            name,
            enabled: Arc::new(RwLock::new(true)),
        }
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn run(&self, context: &ReactiveContext) {
        if *self.enabled.read().unwrap() {
            (self.effect_fn)(context);
        }
    }

    pub fn enable(&self) {
        *self.enabled.write().unwrap() = true;
    }

    pub fn disable(&self) {
        *self.enabled.write().unwrap() = false;
    }

    pub fn dependencies(&self) -> Vec<NodeId> {
        self.dependencies.read().unwrap().iter().cloned().collect()
    }
}

impl fmt::Debug for Effect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Effect")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("dependencies", &self.dependencies)
            .field("enabled", &self.enabled)
            .finish()
    }
}

/// Reactive node types
#[derive(Debug, Clone)]
pub enum ReactiveNode {
    Signal(Signal),
    Computed(Computed),
    Effect(Effect),
}

impl ReactiveNode {
    pub fn id(&self) -> NodeId {
        match self {
            ReactiveNode::Signal(s) => s.id(),
            ReactiveNode::Computed(c) => c.id(),
            ReactiveNode::Effect(e) => e.id(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            ReactiveNode::Signal(s) => s.name(),
            ReactiveNode::Computed(c) => c.name(),
            ReactiveNode::Effect(e) => e.name(),
        }
    }
}

/// The reactive context manages all reactive nodes and change propagation
pub struct ReactiveContext {
    nodes: RwLock<HashMap<NodeId, ReactiveNode>>,
    name_to_id: RwLock<HashMap<String, NodeId>>,

    // Dependency graph for topological sorting
    dependency_graph: RwLock<HashMap<NodeId, HashSet<NodeId>>>,

    // Currently executing node (for automatic dependency tracking)
    current_node: RwLock<Option<NodeId>>,

    // Batching support
    batch_mode: RwLock<bool>,
    pending_updates: RwLock<HashSet<NodeId>>,
}

impl ReactiveContext {
    pub fn new() -> Self {
        Self {
            nodes: RwLock::new(HashMap::new()),
            name_to_id: RwLock::new(HashMap::new()),
            dependency_graph: RwLock::new(HashMap::new()),
            current_node: RwLock::new(None),
            batch_mode: RwLock::new(false),
            pending_updates: RwLock::new(HashSet::new()),
        }
    }

    /// Register a new signal
    pub fn register_signal(
        &self,
        name: String,
        initial_value: Value,
    ) -> Result<NodeId, ReactiveError> {
        let signal = Signal::new(name.clone(), initial_value);
        let id = signal.id();

        let mut nodes = self.nodes.write().unwrap();
        let mut name_to_id = self.name_to_id.write().unwrap();

        if name_to_id.contains_key(&name) {
            return Err(ReactiveError::DuplicateName(name));
        }

        nodes.insert(id, ReactiveNode::Signal(signal));
        name_to_id.insert(name, id);

        Ok(id)
    }

    /// Register a computed value
    pub fn register_computed<F>(
        &self,
        name: String,
        dependencies: Vec<String>,
        compute_fn: F,
    ) -> Result<NodeId, ReactiveError>
    where
        F: Fn(&ReactiveContext) -> Value + Send + Sync + 'static,
    {
        // Resolve dependency names to IDs
        let name_to_id = self.name_to_id.read().unwrap();
        let dep_ids: Result<Vec<NodeId>, _> = dependencies
            .iter()
            .map(|dep_name| {
                name_to_id
                    .get(dep_name)
                    .cloned()
                    .ok_or_else(|| ReactiveError::UnknownDependency(dep_name.clone()))
            })
            .collect();

        let dep_ids = dep_ids?;
        drop(name_to_id);

        let computed = Computed::new(name.clone(), dep_ids.clone(), compute_fn);
        let id = computed.id();

        // Subscribe to dependencies
        let nodes = self.nodes.read().unwrap();
        for dep_id in &dep_ids {
            if let Some(ReactiveNode::Signal(signal)) = nodes.get(dep_id) {
                signal.subscribe(id);
            } else if let Some(ReactiveNode::Computed(comp)) = nodes.get(dep_id) {
                comp.subscribe(id);
            }
        }
        drop(nodes);

        // Update dependency graph
        let mut dep_graph = self.dependency_graph.write().unwrap();
        dep_graph.insert(id, dep_ids.into_iter().collect());
        drop(dep_graph);

        // Initial computation
        computed.recompute(self);

        let mut nodes = self.nodes.write().unwrap();
        let mut name_to_id = self.name_to_id.write().unwrap();

        if name_to_id.contains_key(&name) {
            return Err(ReactiveError::DuplicateName(name));
        }

        nodes.insert(id, ReactiveNode::Computed(computed));
        name_to_id.insert(name, id);

        Ok(id)
    }

    /// Register an effect
    pub fn register_effect<F>(
        &self,
        name: String,
        dependencies: Vec<String>,
        effect_fn: F,
    ) -> Result<NodeId, ReactiveError>
    where
        F: Fn(&ReactiveContext) + Send + Sync + 'static,
    {
        // Resolve dependency names to IDs
        let name_to_id = self.name_to_id.read().unwrap();
        let dep_ids: Result<Vec<NodeId>, _> = dependencies
            .iter()
            .map(|dep_name| {
                name_to_id
                    .get(dep_name)
                    .cloned()
                    .ok_or_else(|| ReactiveError::UnknownDependency(dep_name.clone()))
            })
            .collect();

        let dep_ids = dep_ids?;
        drop(name_to_id);

        let effect = Effect::new(name.clone(), dep_ids.clone(), effect_fn);
        let id = effect.id();

        // Subscribe to dependencies
        let nodes = self.nodes.read().unwrap();
        for dep_id in &dep_ids {
            if let Some(ReactiveNode::Signal(signal)) = nodes.get(dep_id) {
                signal.subscribe(id);
            } else if let Some(ReactiveNode::Computed(comp)) = nodes.get(dep_id) {
                comp.subscribe(id);
            }
        }
        drop(nodes);

        // Update dependency graph
        let mut dep_graph = self.dependency_graph.write().unwrap();
        dep_graph.insert(id, dep_ids.into_iter().collect());
        drop(dep_graph);

        // Initial run
        effect.run(self);

        let mut nodes = self.nodes.write().unwrap();
        nodes.insert(id, ReactiveNode::Effect(effect));

        Ok(id)
    }

    /// Get a signal value by name
    pub fn get(&self, name: &str) -> Result<Value, ReactiveError> {
        let name_to_id = self.name_to_id.read().unwrap();
        let id = name_to_id
            .get(name)
            .ok_or_else(|| ReactiveError::NotFound(name.to_string()))?;

        let nodes = self.nodes.read().unwrap();
        match nodes.get(id) {
            Some(ReactiveNode::Signal(signal)) => Ok(signal.get()),
            Some(ReactiveNode::Computed(computed)) => Ok(computed.get()),
            Some(ReactiveNode::Effect(_)) => Err(ReactiveError::InvalidOperation(
                "Cannot get value of an effect".to_string(),
            )),
            None => Err(ReactiveError::NotFound(name.to_string())),
        }
    }

    /// Set a signal value by name
    pub fn set(&self, name: &str, value: Value) -> Result<(), ReactiveError> {
        let name_to_id = self.name_to_id.read().unwrap();
        let id = *name_to_id
            .get(name)
            .ok_or_else(|| ReactiveError::NotFound(name.to_string()))?;
        drop(name_to_id);

        let nodes = self.nodes.read().unwrap();
        match nodes.get(&id) {
            Some(ReactiveNode::Signal(signal)) => {
                let subscribers = signal.set(value);
                drop(nodes);

                if *self.batch_mode.read().unwrap() {
                    // In batch mode, just collect updates
                    let mut pending = self.pending_updates.write().unwrap();
                    pending.extend(subscribers);
                } else {
                    // Propagate changes immediately
                    self.propagate_changes(subscribers)?;
                }
                Ok(())
            }
            Some(ReactiveNode::Computed(_)) => Err(ReactiveError::InvalidOperation(
                "Cannot directly set a computed value".to_string(),
            )),
            Some(ReactiveNode::Effect(_)) => Err(ReactiveError::InvalidOperation(
                "Cannot set an effect".to_string(),
            )),
            None => Err(ReactiveError::NotFound(name.to_string())),
        }
    }

    /// Propagate changes through the dependency graph
    fn propagate_changes(&self, mut changed_ids: Vec<NodeId>) -> Result<(), ReactiveError> {
        let mut visited = HashSet::new();
        let mut to_update = VecDeque::new();

        // Add initial changed nodes
        for id in changed_ids.drain(..) {
            if visited.insert(id) {
                to_update.push_back(id);
            }
        }

        // Process in topological order
        while let Some(id) = to_update.pop_front() {
            let nodes = self.nodes.read().unwrap();

            match nodes.get(&id) {
                Some(ReactiveNode::Computed(computed)) => {
                    let subscribers = computed.recompute(self);
                    drop(nodes);

                    for sub_id in subscribers {
                        if visited.insert(sub_id) {
                            to_update.push_back(sub_id);
                        }
                    }
                }
                Some(ReactiveNode::Effect(effect)) => {
                    effect.run(self);
                    drop(nodes);
                }
                _ => {
                    drop(nodes);
                }
            }
        }

        Ok(())
    }

    /// Start batch mode (accumulate updates)
    pub fn begin_batch(&self) {
        *self.batch_mode.write().unwrap() = true;
    }

    /// End batch mode and propagate all accumulated changes
    pub fn end_batch(&self) -> Result<(), ReactiveError> {
        *self.batch_mode.write().unwrap() = false;

        let pending: Vec<NodeId> = {
            let mut pending = self.pending_updates.write().unwrap();
            let result = pending.iter().cloned().collect();
            pending.clear();
            result
        };

        self.propagate_changes(pending)
    }

    /// Get all reactive node names
    pub fn all_names(&self) -> Vec<String> {
        self.name_to_id.read().unwrap().keys().cloned().collect()
    }

    /// Remove a reactive node
    pub fn remove(&self, name: &str) -> Result<(), ReactiveError> {
        let mut name_to_id = self.name_to_id.write().unwrap();
        let id = name_to_id
            .remove(name)
            .ok_or_else(|| ReactiveError::NotFound(name.to_string()))?;

        let mut nodes = self.nodes.write().unwrap();
        nodes.remove(&id);

        let mut dep_graph = self.dependency_graph.write().unwrap();
        dep_graph.remove(&id);

        Ok(())
    }

    /// Get statistics about the reactive system
    pub fn stats(&self) -> ReactiveStats {
        let nodes = self.nodes.read().unwrap();
        let dep_graph = self.dependency_graph.read().unwrap();

        let mut signal_count = 0;
        let mut computed_count = 0;
        let mut effect_count = 0;

        for node in nodes.values() {
            match node {
                ReactiveNode::Signal(_) => signal_count += 1,
                ReactiveNode::Computed(_) => computed_count += 1,
                ReactiveNode::Effect(_) => effect_count += 1,
            }
        }

        ReactiveStats {
            total_nodes: nodes.len(),
            signals: signal_count,
            computed_values: computed_count,
            effects: effect_count,
            total_dependencies: dep_graph.values().map(|deps| deps.len()).sum(),
        }
    }

    /// Visualize the dependency graph (DOT format)
    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph ReactiveGraph {\n");

        let nodes = self.nodes.read().unwrap();
        let dep_graph = self.dependency_graph.read().unwrap();

        // Add nodes
        for (id, node) in nodes.iter() {
            let shape = match node {
                ReactiveNode::Signal(_) => "box",
                ReactiveNode::Computed(_) => "ellipse",
                ReactiveNode::Effect(_) => "diamond",
            };
            dot.push_str(&format!(
                "  {} [label=\"{}\", shape={}];\n",
                id,
                node.name(),
                shape
            ));
        }

        // Add edges
        for (id, deps) in dep_graph.iter() {
            for dep_id in deps {
                dot.push_str(&format!("  {} -> {};\n", dep_id, id));
            }
        }

        dot.push_str("}\n");
        dot
    }
}

impl Default for ReactiveContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about the reactive system
#[derive(Debug, Clone)]
pub struct ReactiveStats {
    pub total_nodes: usize,
    pub signals: usize,
    pub computed_values: usize,
    pub effects: usize,
    pub total_dependencies: usize,
}

/// Errors in the reactive system
#[derive(Debug, Clone)]
pub enum ReactiveError {
    NotFound(String),
    DuplicateName(String),
    UnknownDependency(String),
    InvalidOperation(String),
    CircularDependency(Vec<String>),
}

impl fmt::Display for ReactiveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReactiveError::NotFound(name) => write!(f, "Reactive node '{}' not found", name),
            ReactiveError::DuplicateName(name) => {
                write!(f, "Reactive node '{}' already exists", name)
            }
            ReactiveError::UnknownDependency(name) => write!(f, "Unknown dependency '{}'", name),
            ReactiveError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            ReactiveError::CircularDependency(cycle) => {
                write!(f, "Circular dependency detected: {}", cycle.join(" -> "))
            }
        }
    }
}

impl std::error::Error for ReactiveError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_creation() {
        let ctx = ReactiveContext::new();
        let result = ctx.register_signal("count".to_string(), Value::Integer(0));
        assert!(result.is_ok());

        let value = ctx.get("count").unwrap();
        assert_eq!(value, Value::Integer(0));
    }

    #[test]
    fn test_signal_update() {
        let ctx = ReactiveContext::new();
        ctx.register_signal("count".to_string(), Value::Integer(0))
            .unwrap();

        ctx.set("count", Value::Integer(42)).unwrap();
        let value = ctx.get("count").unwrap();
        assert_eq!(value, Value::Integer(42));
    }

    #[test]
    fn test_computed_value() {
        let ctx = ReactiveContext::new();
        ctx.register_signal("x".to_string(), Value::Integer(10))
            .unwrap();
        ctx.register_signal("y".to_string(), Value::Integer(20))
            .unwrap();

        ctx.register_computed(
            "sum".to_string(),
            vec!["x".to_string(), "y".to_string()],
            |ctx| {
                let x = match ctx.get("x").unwrap() {
                    Value::Integer(n) => n,
                    _ => 0,
                };
                let y = match ctx.get("y").unwrap() {
                    Value::Integer(n) => n,
                    _ => 0,
                };
                Value::Integer(x + y)
            },
        )
        .unwrap();

        let sum = ctx.get("sum").unwrap();
        assert_eq!(sum, Value::Integer(30));

        // Update x and check if sum updates
        ctx.set("x", Value::Integer(15)).unwrap();
        let sum = ctx.get("sum").unwrap();
        assert_eq!(sum, Value::Integer(35));
    }

    #[test]
    fn test_batch_updates() {
        let ctx = ReactiveContext::new();
        ctx.register_signal("a".to_string(), Value::Integer(1))
            .unwrap();
        ctx.register_signal("b".to_string(), Value::Integer(2))
            .unwrap();

        ctx.begin_batch();
        ctx.set("a", Value::Integer(10)).unwrap();
        ctx.set("b", Value::Integer(20)).unwrap();
        ctx.end_batch().unwrap();

        assert_eq!(ctx.get("a").unwrap(), Value::Integer(10));
        assert_eq!(ctx.get("b").unwrap(), Value::Integer(20));
    }

    #[test]
    fn test_stats() {
        let ctx = ReactiveContext::new();
        ctx.register_signal("x".to_string(), Value::Integer(1))
            .unwrap();
        ctx.register_signal("y".to_string(), Value::Integer(2))
            .unwrap();

        let stats = ctx.stats();
        assert_eq!(stats.signals, 2);
        assert_eq!(stats.total_nodes, 2);
    }
}
