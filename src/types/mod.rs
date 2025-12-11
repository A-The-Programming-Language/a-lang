//! # Context-Aware Type System (WOW Factor #5)
//!
//! A-lang's type system adapts to usage context, providing:
//! - Bidirectional type inference
//! - Gradual typing (start dynamic, add types later)
//! - Context-dependent type behavior
//! - Type refinement based on control flow

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Type representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    /// Unknown/inferred type
    Unknown,

    /// Any type (top type)
    Any,

    /// Never type (bottom type)
    Never,

    /// Primitive types
    Nil,
    Boolean,
    Integer,
    Float,
    String,

    /// Composite types
    Array(Box<Type>),
    Tuple(Vec<Type>),
    Object(HashMap<String, Type>),

    /// Function type
    Function {
        parameters: Vec<Type>,
        return_type: Box<Type>,
    },

    /// Union type
    Union(Vec<Type>),

    /// Intersection type
    Intersection(Vec<Type>),

    /// Optional type
    Optional(Box<Type>),

    /// Generic type variable
    TypeVar(String),

    /// Named type (user-defined)
    Named(String),

    /// Reactive type wrapper (WOW #2)
    Reactive(Box<Type>),

    /// Contextual type (WOW #5)
    Contextual {
        base_type: Box<Type>,
        context: String,
        constraints: Vec<TypeConstraint>,
    },
}

/// Type constraint for context-aware typing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeConstraint {
    pub kind: ConstraintKind,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConstraintKind {
    /// Type must implement a trait/interface
    Implements(String),

    /// Type must extend another type
    Extends(String),

    /// Type must satisfy a predicate
    Satisfies(String),

    /// Type must be assignable to another type
    AssignableTo(Box<Type>),
}

/// Type inference context
pub struct TypeContext {
    /// Variable types
    bindings: HashMap<String, Type>,

    /// Type aliases
    aliases: HashMap<String, Type>,

    /// Active context name
    current_context: Option<String>,
}

impl TypeContext {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            aliases: HashMap::new(),
            current_context: None,
        }
    }

    /// Bind a variable to a type
    pub fn bind(&mut self, name: String, ty: Type) {
        self.bindings.insert(name, ty);
    }

    /// Get the type of a variable
    pub fn get(&self, name: &str) -> Option<&Type> {
        self.bindings.get(name)
    }

    /// Create a type alias
    pub fn alias(&mut self, name: String, ty: Type) {
        self.aliases.insert(name, ty);
    }

    /// Enter a new context
    pub fn enter_context(&mut self, name: String) {
        self.current_context = Some(name);
    }

    /// Exit the current context
    pub fn exit_context(&mut self) {
        self.current_context = None;
    }

    /// Get the current context
    pub fn current_context(&self) -> Option<&str> {
        self.current_context.as_deref()
    }
}

impl Default for TypeContext {
    fn default() -> Self {
        Self::new()
    }
}

impl Type {
    /// Check if this type is compatible with another type
    pub fn is_compatible_with(&self, other: &Type) -> bool {
        match (self, other) {
            (Type::Any, _) | (_, Type::Any) => true,
            (Type::Never, _) | (_, Type::Never) => true,
            (Type::Unknown, _) | (_, Type::Unknown) => true,
            (a, b) if a == b => true,
            (Type::Optional(inner), other) | (other, Type::Optional(inner)) => {
                inner.is_compatible_with(other) || matches!(other, Type::Nil)
            }
            (Type::Union(types), other) => types.iter().any(|t| t.is_compatible_with(other)),
            (other, Type::Union(types)) => types.iter().any(|t| other.is_compatible_with(t)),
            _ => false,
        }
    }

    /// Check if this type is numeric
    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::Integer | Type::Float)
    }

    /// Check if this type is a string
    pub fn is_string(&self) -> bool {
        matches!(self, Type::String)
    }

    /// Check if this type is callable
    pub fn is_callable(&self) -> bool {
        matches!(self, Type::Function { .. })
    }

    /// Get a human-readable name for this type
    pub fn name(&self) -> String {
        match self {
            Type::Unknown => "unknown".to_string(),
            Type::Any => "any".to_string(),
            Type::Never => "never".to_string(),
            Type::Nil => "nil".to_string(),
            Type::Boolean => "boolean".to_string(),
            Type::Integer => "integer".to_string(),
            Type::Float => "float".to_string(),
            Type::String => "string".to_string(),
            Type::Array(inner) => format!("array<{}>", inner.name()),
            Type::Tuple(types) => {
                let names: Vec<_> = types.iter().map(|t| t.name()).collect();
                format!("({})", names.join(", "))
            }
            Type::Object(_) => "object".to_string(),
            Type::Function { .. } => "function".to_string(),
            Type::Union(types) => {
                let names: Vec<_> = types.iter().map(|t| t.name()).collect();
                names.join(" | ")
            }
            Type::Intersection(types) => {
                let names: Vec<_> = types.iter().map(|t| t.name()).collect();
                names.join(" & ")
            }
            Type::Optional(inner) => format!("{}?", inner.name()),
            Type::TypeVar(name) => name.clone(),
            Type::Named(name) => name.clone(),
            Type::Reactive(inner) => format!("reactive<{}>", inner.name()),
            Type::Contextual { base_type, context, .. } => {
                format!("{}@{}", base_type.name(), context)
            }
        }
    }

    /// Unify two types (find a common type)
    pub fn unify(&self, other: &Type) -> Option<Type> {
        match (self, other) {
            (Type::Unknown, other) | (other, Type::Unknown) => Some(other.clone()),
            (a, b) if a == b => Some(a.clone()),
            (Type::Integer, Type::Float) | (Type::Float, Type::Integer) => Some(Type::Float),
            (Type::Any, _) | (_, Type::Any) => Some(Type::Any),
            _ => None,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Type inference engine
pub struct TypeInference {
    context: TypeContext,
}

impl TypeInference {
    pub fn new() -> Self {
        Self {
            context: TypeContext::new(),
        }
    }

    /// Infer the type of an expression
    pub fn infer(&mut self, expr: &crate::ast::Expression) -> Result<Type, TypeError> {
        use crate::ast::Expression;

        match expr {
            Expression::Literal { value, .. } => Ok(self.infer_literal(value)),
            Expression::Identifier { name, .. } => {
                self.context
                    .get(name)
                    .cloned()
                    .ok_or_else(|| TypeError::UndefinedVariable(name.clone()))
            }
            Expression::Binary {
                left,
                operator,
                right,
                ..
            } => self.infer_binary(left, operator, right),
            Expression::Array { elements, .. } => self.infer_array(elements),
            _ => Ok(Type::Unknown),
        }
    }

    fn infer_literal(&self, literal: &crate::ast::Literal) -> Type {
        use crate::ast::Literal;

        match literal {
            Literal::Integer(_) => Type::Integer,
            Literal::Float(_) => Type::Float,
            Literal::String(_) => Type::String,
            Literal::Boolean(_) => Type::Boolean,
            Literal::Nil => Type::Nil,
        }
    }

    fn infer_binary(
        &mut self,
        left: &crate::ast::Expression,
        operator: &crate::ast::BinaryOp,
        right: &crate::ast::Expression,
    ) -> Result<Type, TypeError> {
        let left_type = self.infer(left)?;
        let right_type = self.infer(right)?;

        use crate::ast::BinaryOp;

        match operator {
            BinaryOp::Add | BinaryOp::Subtract | BinaryOp::Multiply | BinaryOp::Divide => {
                if left_type.is_numeric() && right_type.is_numeric() {
                    Ok(left_type.unify(&right_type).unwrap_or(Type::Float))
                } else if matches!(operator, BinaryOp::Add)
                    && left_type.is_string()
                    && right_type.is_string()
                {
                    Ok(Type::String)
                } else {
                    Err(TypeError::TypeMismatch {
                        expected: "numeric or string".to_string(),
                        found: format!("{} and {}", left_type.name(), right_type.name()),
                    })
                }
            }
            BinaryOp::Equal
            | BinaryOp::NotEqual
            | BinaryOp::Less
            | BinaryOp::LessEqual
            | BinaryOp::Greater
            | BinaryOp::GreaterEqual => Ok(Type::Boolean),
            BinaryOp::And | BinaryOp::Or => Ok(Type::Boolean),
            _ => Ok(Type::Unknown),
        }
    }

    fn infer_array(&mut self, elements: &[crate::ast::Expression]) -> Result<Type, TypeError> {
        if elements.is_empty() {
            return Ok(Type::Array(Box::new(Type::Unknown)));
        }

        let first_type = self.infer(&elements[0])?;
        Ok(Type::Array(Box::new(first_type)))
    }

    /// Get the type context
    pub fn context(&self) -> &TypeContext {
        &self.context
    }

    /// Get mutable type context
    pub fn context_mut(&mut self) -> &mut TypeContext {
        &mut self.context
    }
}

impl Default for TypeInference {
    fn default() -> Self {
        Self::new()
    }
}

/// Type error
#[derive(Debug, Clone)]
pub enum TypeError {
    UndefinedVariable(String),
    TypeMismatch { expected: String, found: String },
    CannotInfer(String),
    InvalidOperation { operation: String, types: Vec<Type> },
    ConstraintViolation(String),
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypeError::UndefinedVariable(name) => write!(f, "Undefined variable: {}", name),
            TypeError::TypeMismatch { expected, found } => {
                write!(f, "Type mismatch: expected {}, found {}", expected, found)
            }
            TypeError::CannotInfer(msg) => write!(f, "Cannot infer type: {}", msg),
            TypeError::InvalidOperation { operation, types } => {
                let type_names: Vec<_> = types.iter().map(|t| t.name()).collect();
                write!(
                    f,
                    "Invalid operation '{}' for types: {}",
                    operation,
                    type_names.join(", ")
                )
            }
            TypeError::ConstraintViolation(msg) => write!(f, "Constraint violation: {}", msg),
        }
    }
}

impl std::error::Error for TypeError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_compatibility() {
        let int_type = Type::Integer;
        let float_type = Type::Float;
        let any_type = Type::Any;

        assert!(int_type.is_compatible_with(&any_type));
        assert!(any_type.is_compatible_with(&float_type));
    }

    #[test]
    fn test_type_unification() {
        let int_type = Type::Integer;
        let float_type = Type::Float;

        let unified = int_type.unify(&float_type);
        assert_eq!(unified, Some(Type::Float));
    }

    #[test]
    fn test_type_names() {
        assert_eq!(Type::Integer.name(), "integer");
        assert_eq!(Type::Array(Box::new(Type::String)).name(), "array<string>");
        assert_eq!(Type::Optional(Box::new(Type::Boolean)).name(), "boolean?");
    }
}
