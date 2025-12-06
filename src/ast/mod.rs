//! # A-lang Abstract Syntax Tree
//!
//! AST definitions supporting all 5 WOW factors:
//! 1. Time-Travel Debugging
//! 2. Reactive Variables
//! 3. Runtime Syntax Extensions
//! 4. Auto-Parallelization
//! 5. Context-Aware Types

use serde::{Deserialize, Serialize};
use std::fmt;

/// Source location information for error reporting
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self {
            start,
            end,
            line,
            column,
        }
    }

    pub fn dummy() -> Self {
        Self {
            start: 0,
            end: 0,
            line: 0,
            column: 0,
        }
    }
}

/// Program is a collection of statements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub span: Span,
}

/// Statement types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    /// Variable declaration: let x = expr;
    Let {
        name: String,
        value: Expression,
        type_annotation: Option<TypeAnnotation>,
        span: Span,
    },

    /// Constant declaration: const x = expr;
    Const {
        name: String,
        value: Expression,
        type_annotation: Option<TypeAnnotation>,
        span: Span,
    },

    /// WOW #2: Reactive variable declaration: reactive x <- expr;
    Reactive {
        name: String,
        initial_value: Expression,
        type_annotation: Option<TypeAnnotation>,
        span: Span,
    },

    /// Function declaration
    Function {
        name: String,
        parameters: Vec<Parameter>,
        body: Vec<Statement>,
        return_type: Option<TypeAnnotation>,
        is_async: bool,
        span: Span,
    },

    /// Return statement
    Return {
        value: Option<Expression>,
        span: Span,
    },

    /// Expression statement
    Expression { expr: Expression, span: Span },

    /// If statement
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
        span: Span,
    },

    /// While loop
    While {
        condition: Expression,
        body: Vec<Statement>,
        span: Span,
    },

    /// For loop
    For {
        variable: String,
        iterable: Expression,
        body: Vec<Statement>,
        span: Span,
    },

    /// Break statement
    Break { span: Span },

    /// Continue statement
    Continue { span: Span },

    /// Match expression (pattern matching)
    Match {
        value: Expression,
        arms: Vec<MatchArm>,
        span: Span,
    },

    /// Struct definition
    Struct {
        name: String,
        fields: Vec<StructField>,
        span: Span,
    },

    /// Enum definition
    Enum {
        name: String,
        variants: Vec<EnumVariant>,
        span: Span,
    },

    /// Import statement
    Import {
        path: Vec<String>,
        items: Vec<String>,
        span: Span,
    },

    /// Export statement
    Export { item: String, span: Span },

    /// Class definition
    Class {
        name: String,
        superclass: Option<String>,
        constructor: Option<Function>,
        methods: Vec<ClassMethod>,
        span: Span,
    },

    /// Try-catch-finally statement
    Try {
        try_block: Vec<Statement>,
        catch_clause: Option<CatchClause>,
        finally_block: Option<Vec<Statement>>,
        span: Span,
    },

    /// Throw statement
    Throw { value: Expression, span: Span },

    /// WOW #1: Time-Travel Debugging - Snapshot
    Snapshot { label: Option<String>, span: Span },

    /// WOW #1: Time-Travel Debugging - Rewind
    Rewind {
        steps: Option<Expression>,
        to_label: Option<String>,
        span: Span,
    },

    /// WOW #1: Time-Travel Debugging - Checkpoint
    Checkpoint { label: String, span: Span },

    /// WOW #2: Watch expression for reactive updates
    Watch {
        expression: Expression,
        handler: Vec<Statement>,
        span: Span,
    },

    /// WOW #2: Effect (side effect when dependencies change)
    Effect {
        dependencies: Vec<String>,
        body: Vec<Statement>,
        span: Span,
    },

    /// WOW #2: Computed value (derived from reactive values)
    Computed {
        name: String,
        dependencies: Vec<String>,
        expression: Expression,
        span: Span,
    },

    /// WOW #3: Syntax extension definition
    SyntaxExtension {
        name: String,
        pattern: String,
        transformer: Expression,
        span: Span,
    },

    /// WOW #4: Parallel block
    Parallel { body: Vec<Statement>, span: Span },

    /// WOW #4: Concurrent execution
    Concurrent { tasks: Vec<Expression>, span: Span },

    /// WOW #4: Atomic block (synchronized)
    Atomic { body: Vec<Statement>, span: Span },

    /// WOW #5: Context definition
    Context {
        name: String,
        constraints: Vec<TypeConstraint>,
        span: Span,
    },
}

/// Expression types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    /// Literal values
    Literal { value: Literal, span: Span },

    /// Variable reference
    Identifier { name: String, span: Span },

    /// Binary operation
    Binary {
        left: Box<Expression>,
        operator: BinaryOp,
        right: Box<Expression>,
        span: Span,
    },

    /// Unary operation
    Unary {
        operator: UnaryOp,
        operand: Box<Expression>,
        span: Span,
    },

    /// Function call
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
        span: Span,
    },

    /// Array literal
    Array {
        elements: Vec<Expression>,
        span: Span,
    },

    /// Object literal (hash map)
    Object {
        fields: Vec<(String, Expression)>,
        span: Span,
    },

    /// Property access: obj.property
    PropertyAccess {
        object: Box<Expression>,
        property: String,
        span: Span,
    },

    /// Index access: arr[index]
    IndexAccess {
        object: Box<Expression>,
        index: Box<Expression>,
        span: Span,
    },

    /// Lambda function
    Lambda {
        parameters: Vec<Parameter>,
        body: Vec<Statement>,
        return_type: Option<TypeAnnotation>,
        span: Span,
    },

    /// Async/await
    Await {
        expression: Box<Expression>,
        span: Span,
    },

    /// WOW #2: Signal creation
    Signal {
        initial_value: Box<Expression>,
        span: Span,
    },

    /// WOW #3: Quote (code as data)
    Quote {
        expression: Box<Expression>,
        span: Span,
    },

    /// WOW #3: Unquote (evaluate in quote context)
    Unquote {
        expression: Box<Expression>,
        span: Span,
    },

    /// WOW #4: Parallel map
    ParallelMap {
        collection: Box<Expression>,
        function: Box<Expression>,
        span: Span,
    },

    /// WOW #4: Parallel filter
    ParallelFilter {
        collection: Box<Expression>,
        predicate: Box<Expression>,
        span: Span,
    },

    /// Pipeline operator: expr |> function
    Pipeline {
        value: Box<Expression>,
        function: Box<Expression>,
        span: Span,
    },

    /// Stream pipe: expr ~> handler
    StreamPipe {
        source: Box<Expression>,
        handler: Box<Expression>,
        span: Span,
    },

    /// Range: start..end
    Range {
        start: Box<Expression>,
        end: Box<Expression>,
        inclusive: bool,
        span: Span,
    },

    /// Ternary: condition ? true_expr : false_expr
    Ternary {
        condition: Box<Expression>,
        then_expr: Box<Expression>,
        else_expr: Box<Expression>,
        span: Span,
    },

    /// New expression: new ClassName(args)
    New {
        class_name: String,
        arguments: Vec<Expression>,
        span: Span,
    },

    /// This keyword
    This { span: Span },
}

/// Literal values
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Nil,
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOp {
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,

    // Comparison
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Logical
    And,
    Or,

    // Bitwise
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Subtract => write!(f, "-"),
            BinaryOp::Multiply => write!(f, "*"),
            BinaryOp::Divide => write!(f, "/"),
            BinaryOp::Modulo => write!(f, "%"),
            BinaryOp::Power => write!(f, "**"),
            BinaryOp::Equal => write!(f, "=="),
            BinaryOp::NotEqual => write!(f, "!="),
            BinaryOp::Less => write!(f, "<"),
            BinaryOp::LessEqual => write!(f, "<="),
            BinaryOp::Greater => write!(f, ">"),
            BinaryOp::GreaterEqual => write!(f, ">="),
            BinaryOp::And => write!(f, "&&"),
            BinaryOp::Or => write!(f, "||"),
            BinaryOp::BitwiseAnd => write!(f, "&"),
            BinaryOp::BitwiseOr => write!(f, "|"),
            BinaryOp::BitwiseXor => write!(f, "^"),
            BinaryOp::LeftShift => write!(f, "<<"),
            BinaryOp::RightShift => write!(f, ">>"),
        }
    }
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOp {
    Negate,
    Not,
    BitwiseNot,
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnaryOp::Negate => write!(f, "-"),
            UnaryOp::Not => write!(f, "!"),
            UnaryOp::BitwiseNot => write!(f, "~"),
        }
    }
}

/// Function parameter
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: Option<TypeAnnotation>,
    pub default_value: Option<Expression>,
    pub span: Span,
}

/// Match arm
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expression>,
    pub body: Vec<Statement>,
    pub span: Span,
}

/// Pattern for pattern matching
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Pattern {
    Wildcard,
    Literal(Literal),
    Identifier(String),
    Tuple(Vec<Pattern>),
    Array(Vec<Pattern>),
    Object(Vec<(String, Pattern)>),
    Variant { name: String, fields: Vec<Pattern> },
}

/// Class method definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassMethod {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub body: Vec<Statement>,
    pub is_static: bool,
    pub span: Span,
}

/// Function definition (used for constructors and standalone functions)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Function {
    pub name: Option<String>,
    pub parameters: Vec<Parameter>,
    pub body: Vec<Statement>,
    pub return_type: Option<TypeAnnotation>,
    pub span: Span,
}

/// Catch clause for try-catch
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatchClause {
    pub parameter: Option<String>,
    pub body: Vec<Statement>,
    pub span: Span,
}

/// Struct field definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructField {
    pub name: String,
    pub type_annotation: TypeAnnotation,
    pub default_value: Option<Expression>,
    pub span: Span,
}

/// Enum variant definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: String,
    pub fields: Vec<TypeAnnotation>,
    pub span: Span,
}

/// Type annotations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeAnnotation {
    Named(String),
    Array(Box<TypeAnnotation>),
    Tuple(Vec<TypeAnnotation>),
    Function {
        parameters: Vec<TypeAnnotation>,
        return_type: Box<TypeAnnotation>,
    },
    Generic {
        name: String,
        type_parameters: Vec<TypeAnnotation>,
    },
    Union(Vec<TypeAnnotation>),
    Optional(Box<TypeAnnotation>),

    /// WOW #2: Reactive type wrapper
    Reactive(Box<TypeAnnotation>),

    /// WOW #5: Context-aware type
    Contextual {
        base_type: Box<TypeAnnotation>,
        context_name: String,
    },

    /// WOW #5: Inferred type
    Inferred,
}

/// WOW #5: Type constraints for context-aware typing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeConstraint {
    pub name: String,
    pub constraint_type: ConstraintType,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConstraintType {
    Implements(String),
    Extends(String),
    Where(Expression),
}

impl Program {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self {
            statements,
            span: Span::dummy(),
        }
    }
}

impl Expression {
    pub fn span(&self) -> &Span {
        match self {
            Expression::Literal { span, .. } => span,
            Expression::Identifier { span, .. } => span,
            Expression::Binary { span, .. } => span,
            Expression::Unary { span, .. } => span,
            Expression::Call { span, .. } => span,
            Expression::Array { span, .. } => span,
            Expression::Object { span, .. } => span,
            Expression::PropertyAccess { span, .. } => span,
            Expression::IndexAccess { span, .. } => span,
            Expression::Lambda { span, .. } => span,
            Expression::Await { span, .. } => span,
            Expression::Signal { span, .. } => span,
            Expression::Quote { span, .. } => span,
            Expression::Unquote { span, .. } => span,
            Expression::ParallelMap { span, .. } => span,
            Expression::ParallelFilter { span, .. } => span,
            Expression::Pipeline { span, .. } => span,
            Expression::StreamPipe { span, .. } => span,
            Expression::Range { span, .. } => span,
            Expression::Ternary { span, .. } => span,
            Expression::New { span, .. } => span,
            Expression::This { span } => span,
        }
    }
}

impl Statement {
    pub fn span(&self) -> &Span {
        match self {
            Statement::Let { span, .. } => span,
            Statement::Const { span, .. } => span,
            Statement::Reactive { span, .. } => span,
            Statement::Function { span, .. } => span,
            Statement::Return { span, .. } => span,
            Statement::Expression { span, .. } => span,
            Statement::If { span, .. } => span,
            Statement::While { span, .. } => span,
            Statement::For { span, .. } => span,
            Statement::Break { span } => span,
            Statement::Continue { span } => span,
            Statement::Match { span, .. } => span,
            Statement::Struct { span, .. } => span,
            Statement::Enum { span, .. } => span,
            Statement::Import { span, .. } => span,
            Statement::Export { span, .. } => span,
            Statement::Class { span, .. } => span,
            Statement::Try { span, .. } => span,
            Statement::Throw { span, .. } => span,
            Statement::Snapshot { span, .. } => span,
            Statement::Rewind { span, .. } => span,
            Statement::Checkpoint { span, .. } => span,
            Statement::Watch { span, .. } => span,
            Statement::Effect { span, .. } => span,
            Statement::Computed { span, .. } => span,
            Statement::SyntaxExtension { span, .. } => span,
            Statement::Parallel { span, .. } => span,
            Statement::Concurrent { span, .. } => span,
            Statement::Atomic { span, .. } => span,
            Statement::Context { span, .. } => span,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_creation() {
        let lit = Literal::Integer(42);
        assert_eq!(lit, Literal::Integer(42));
    }

    #[test]
    fn test_expression_span() {
        let expr = Expression::Literal {
            value: Literal::Integer(42),
            span: Span::new(0, 2, 1, 1),
        };
        assert_eq!(expr.span().start, 0);
        assert_eq!(expr.span().end, 2);
    }

    #[test]
    fn test_binary_op_display() {
        assert_eq!(format!("{}", BinaryOp::Add), "+");
        assert_eq!(format!("{}", BinaryOp::Equal), "==");
        assert_eq!(format!("{}", BinaryOp::Power), "**");
    }
}
