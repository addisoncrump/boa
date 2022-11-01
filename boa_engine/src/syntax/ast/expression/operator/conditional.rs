use crate::syntax::ast::visitor::{VisitWith, Visitor, VisitorMut};
use crate::syntax::ast::{expression::Expression, ContainsSymbol};
use crate::try_break;
use boa_interner::{Interner, ToInternedString};
use std::ops::ControlFlow;

/// The `conditional` (ternary) operation is the only JavaScript operation that takes three
/// operands.
///
/// This operation takes three operands: a condition followed by a question mark (`?`),
/// then an expression to execute `if` the condition is truthy followed by a colon (`:`),
/// and finally the expression to execute if the condition is `false`.
/// This operator is frequently used as a shortcut for the `if` statement.
///
/// More information:
///  - [ECMAScript reference][spec]
///  - [MDN documentation][mdn]
///
/// [spec]: https://tc39.es/ecma262/#prod-ConditionalExpression
/// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Grammar_and_types#Literals
#[cfg_attr(feature = "deser", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Conditional {
    condition: Box<Expression>,
    if_true: Box<Expression>,
    if_false: Box<Expression>,
}

impl Conditional {
    /// Gets the condition of the `Conditional` expression.
    #[inline]
    pub fn condition(&self) -> &Expression {
        &self.condition
    }

    /// Gets the expression returned if `condition` is truthy.
    #[inline]
    pub fn if_true(&self) -> &Expression {
        &self.if_true
    }

    /// Gets the expression returned if `condition` is falsy.
    #[inline]
    pub fn if_false(&self) -> &Expression {
        &self.if_false
    }

    /// Creates a `Conditional` AST Expression.
    #[inline]
    pub fn new(condition: Expression, if_true: Expression, if_false: Expression) -> Self {
        Self {
            condition: Box::new(condition),
            if_true: Box::new(if_true),
            if_false: Box::new(if_false),
        }
    }

    #[inline]
    pub(crate) fn contains_arguments(&self) -> bool {
        self.condition.contains_arguments()
            || self.if_true.contains_arguments()
            || self.if_false.contains_arguments()
    }

    #[inline]
    pub(crate) fn contains(&self, symbol: ContainsSymbol) -> bool {
        self.condition.contains(symbol)
            || self.if_true.contains(symbol)
            || self.if_false.contains(symbol)
    }
}

impl ToInternedString for Conditional {
    #[inline]
    fn to_interned_string(&self, interner: &Interner) -> String {
        format!(
            "{} ? {} : {}",
            self.condition().to_interned_string(interner),
            self.if_true().to_interned_string(interner),
            self.if_false().to_interned_string(interner)
        )
    }
}

impl From<Conditional> for Expression {
    #[inline]
    fn from(cond_op: Conditional) -> Self {
        Self::Conditional(cond_op)
    }
}

impl VisitWith for Conditional {
    fn visit_with<'a, V>(&'a self, visitor: &mut V) -> ControlFlow<V::BreakTy>
    where
        V: Visitor<'a>,
    {
        try_break!(visitor.visit_expression(&*self.condition));
        try_break!(visitor.visit_expression(&*self.if_true));
        visitor.visit_expression(&*self.if_false)
    }

    fn visit_with_mut<'a, V>(&'a mut self, visitor: &mut V) -> ControlFlow<V::BreakTy>
    where
        V: VisitorMut<'a>,
    {
        try_break!(visitor.visit_expression_mut(&mut *self.condition));
        try_break!(visitor.visit_expression_mut(&mut *self.if_true));
        visitor.visit_expression_mut(&mut *self.if_false)
    }
}
