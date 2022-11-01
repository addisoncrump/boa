use crate::syntax::ast::visitor::{VisitWith, Visitor, VisitorMut};
use crate::syntax::ast::{expression::Expression, statement::Statement, ContainsSymbol};
use crate::try_break;
use boa_interner::{Interner, ToIndentedString, ToInternedString};
use std::ops::ControlFlow;

/// The `while` statement creates a loop that executes a specified statement as long as the
/// test condition evaluates to `true`.
///
/// The condition is evaluated before executing the statement.
///
/// More information:
///  - [ECMAScript reference][spec]
///  - [MDN documentation][mdn]
///
/// [spec]: https://tc39.es/ecma262/#prod-grammar-notation-WhileStatement
/// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/while
#[cfg_attr(feature = "deser", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct WhileLoop {
    condition: Expression,
    body: Box<Statement>,
}

impl WhileLoop {
    /// Creates a `WhileLoop` AST node.
    #[inline]
    pub fn new(condition: Expression, body: Statement) -> Self {
        Self {
            condition,
            body: body.into(),
        }
    }

    /// Gets the condition of the while loop.
    #[inline]
    pub fn condition(&self) -> &Expression {
        &self.condition
    }

    /// Gets the body of the while loop.
    #[inline]
    pub fn body(&self) -> &Statement {
        &self.body
    }

    #[inline]
    pub(crate) fn contains_arguments(&self) -> bool {
        self.condition.contains_arguments() || self.body.contains_arguments()
    }

    #[inline]
    pub(crate) fn contains(&self, symbol: ContainsSymbol) -> bool {
        self.condition.contains(symbol) || self.body.contains(symbol)
    }
}

impl ToIndentedString for WhileLoop {
    fn to_indented_string(&self, interner: &Interner, indentation: usize) -> String {
        format!(
            "while ({}) {}",
            self.condition().to_interned_string(interner),
            self.body().to_indented_string(interner, indentation)
        )
    }
}

impl From<WhileLoop> for Statement {
    #[inline]
    fn from(while_loop: WhileLoop) -> Self {
        Self::WhileLoop(while_loop)
    }
}

impl VisitWith for WhileLoop {
    fn visit_with<'a, V>(&'a self, visitor: &mut V) -> ControlFlow<V::BreakTy>
    where
        V: Visitor<'a>,
    {
        try_break!(visitor.visit_expression(&self.condition));
        visitor.visit_statement(&*self.body)
    }

    fn visit_with_mut<'a, V>(&'a mut self, visitor: &mut V) -> ControlFlow<V::BreakTy>
    where
        V: VisitorMut<'a>,
    {
        try_break!(visitor.visit_expression_mut(&mut self.condition));
        visitor.visit_statement_mut(&mut *self.body)
    }
}