//! Await expression Expression.

use crate::syntax::ast::ContainsSymbol;
use std::ops::ControlFlow;

use super::Expression;
use crate::syntax::ast::visitor::{VisitWith, Visitor, VisitorMut};
use boa_interner::{Interner, ToIndentedString, ToInternedString};

/// An await expression is used within an async function to pause execution and wait for a
/// promise to resolve.
///
/// More information:
///  - [ECMAScript reference][spec]
///  - [MDN documentation][mdn]
///
/// [spec]: https://tc39.es/ecma262/#prod-AwaitExpression
/// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/await
#[cfg_attr(feature = "deser", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Await {
    target: Box<Expression>,
}

impl Await {
    /// Return the target expression that should be awaited.
    #[inline]
    pub(crate) fn target(&self) -> &Expression {
        &self.target
    }

    #[inline]
    pub(crate) fn contains_arguments(&self) -> bool {
        self.target.contains_arguments()
    }

    #[inline]
    pub(crate) fn contains(&self, symbol: ContainsSymbol) -> bool {
        self.target.contains(symbol)
    }
}

impl<T> From<T> for Await
where
    T: Into<Box<Expression>>,
{
    #[inline]
    fn from(e: T) -> Self {
        Self { target: e.into() }
    }
}

impl ToInternedString for Await {
    #[inline]
    fn to_interned_string(&self, interner: &Interner) -> String {
        format!("await {}", self.target.to_indented_string(interner, 0))
    }
}

impl From<Await> for Expression {
    #[inline]
    fn from(awaitexpr: Await) -> Self {
        Self::Await(awaitexpr)
    }
}

impl VisitWith for Await {
    fn visit_with<'a, V>(&'a self, visitor: &mut V) -> ControlFlow<V::BreakTy>
    where
        V: Visitor<'a>,
    {
        visitor.visit_expression(&*self.target)
    }

    fn visit_with_mut<'a, V>(&'a mut self, visitor: &mut V) -> ControlFlow<V::BreakTy>
    where
        V: VisitorMut<'a>,
    {
        visitor.visit_expression_mut(&mut *self.target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fmt() {
        // TODO: `let a = await fn()` is invalid syntax as of writing. It should be tested here once implemented.
        crate::syntax::ast::test_formatting(
            r#"
            async function f() {
                await function_call();
            }
            "#,
        );
    }
}
