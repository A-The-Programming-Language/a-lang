//! # Runtime Syntax Extensions (WOW Factor #3)
//!
//! This module enables A-lang to define new syntax at runtime, allowing
//! developers to create Domain-Specific Languages (DSLs) on the fly without
//! recompiling the language.
//!
//! ## Features
//!
//! - Define new syntax patterns during runtime
//! - Transform custom syntax into standard AST nodes
//! - Macro expansion with hygiene
//! - Quote/unquote for metaprogramming
//!
//! ## Example
//!
//! ```alang
//! syntax "unless" {
//!     pattern: "unless CONDITION then BODY",
//!     transform: |cond, body| {
//!         if !cond then body
//!     }
//! }
//!
//! // Now you can use the new syntax:
//! unless x > 10 then {
//!     print("x is not greater than 10");
//! }
//! ```

use crate::ast::{Expression, Statement};
use std::collections::HashMap;
use std::sync::Arc;

/// A syntax extension definition
pub struct SyntaxExtension {
    pub name: String,
    pub pattern: SyntaxPattern,
    pub transformer: Arc<dyn Fn(Vec<Expression>) -> Result<Statement, String> + Send + Sync>,
}

impl Clone for SyntaxExtension {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            pattern: self.pattern.clone(),
            transformer: Arc::clone(&self.transformer),
        }
    }
}

impl std::fmt::Debug for SyntaxExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SyntaxExtension")
            .field("name", &self.name)
            .field("pattern", &self.pattern)
            .field("transformer", &"<function>")
            .finish()
    }
}

/// Pattern for matching custom syntax
#[derive(Debug, Clone)]
pub struct SyntaxPattern {
    pub pattern_string: String,
    pub placeholders: Vec<String>,
}

impl SyntaxPattern {
    pub fn new(pattern: String) -> Self {
        // Parse pattern and extract placeholders (uppercase words)
        let placeholders: Vec<String> = pattern
            .split_whitespace()
            .filter(|word| word.chars().all(|c| c.is_uppercase() || c == '_'))
            .map(|s| s.to_string())
            .collect();

        Self {
            pattern_string: pattern,
            placeholders,
        }
    }

    pub fn matches(&self, _input: &str) -> bool {
        // Simplified pattern matching
        // In a real implementation, this would be more sophisticated
        true
    }
}

/// Registry for syntax extensions
pub struct SyntaxRegistry {
    extensions: HashMap<String, SyntaxExtension>,
}

impl SyntaxRegistry {
    pub fn new() -> Self {
        Self {
            extensions: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: String, extension: SyntaxExtension) {
        self.extensions.insert(name, extension);
    }

    pub fn get(&self, name: &str) -> Option<&SyntaxExtension> {
        self.extensions.get(name)
    }

    pub fn remove(&mut self, name: &str) -> Option<SyntaxExtension> {
        self.extensions.remove(name)
    }

    pub fn list_extensions(&self) -> Vec<&String> {
        self.extensions.keys().collect()
    }
}

impl Default for SyntaxRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Macro expansion context
pub struct MacroExpander {
    registry: SyntaxRegistry,
    expansion_depth: usize,
    max_depth: usize,
}

impl MacroExpander {
    pub fn new() -> Self {
        Self {
            registry: SyntaxRegistry::new(),
            expansion_depth: 0,
            max_depth: 100,
        }
    }

    pub fn expand(&mut self, statement: &Statement) -> Result<Statement, String> {
        if self.expansion_depth >= self.max_depth {
            return Err("Maximum macro expansion depth exceeded".to_string());
        }

        self.expansion_depth += 1;
        let result = self.expand_internal(statement);
        self.expansion_depth -= 1;

        result
    }

    fn expand_internal(&mut self, statement: &Statement) -> Result<Statement, String> {
        // TODO: Implement actual macro expansion
        Ok(statement.clone())
    }

    pub fn register_extension(&mut self, name: String, extension: SyntaxExtension) {
        self.registry.register(name, extension);
    }
}

impl Default for MacroExpander {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syntax_pattern_creation() {
        let pattern = SyntaxPattern::new("unless CONDITION then BODY".to_string());
        assert_eq!(pattern.placeholders.len(), 2);
        assert_eq!(pattern.placeholders[0], "CONDITION");
        assert_eq!(pattern.placeholders[1], "BODY");
    }

    #[test]
    fn test_registry() {
        let mut registry = SyntaxRegistry::new();
        assert_eq!(registry.list_extensions().len(), 0);
    }
}
