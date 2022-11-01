//! The [`Declaration`] Parse Node, as defined by the [spec].
//!
//! Javascript declarations include:
//! - [Lexical][lex] declarations (`let`, `const`).
//! - [Function][fun] declarations (`function`, `async function`).
//! - [Class][class] declarations.
//!
//! See [*Difference between statements and declarations*][diff] for an explanation on why `Declaration`s
//! and `Statement`s are distinct nodes.
//!
//! [spec]: https://tc39.es/ecma262/#prod-Declaration
//! [lex]: https://tc39.es/ecma262/#prod-LexicalDeclaration
//! [fun]: https://tc39.es/ecma262/#prod-HoistableDeclaration
//! [class]: https://tc39.es/ecma262/#prod-ClassDeclaration
//! [diff]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements#difference_between_statements_and_declarations

use super::{
    expression::Identifier,
    function::{AsyncFunction, AsyncGenerator, Class, Function, Generator},
    ContainsSymbol,
};
use boa_interner::{Interner, ToIndentedString, ToInternedString};
use core::ops::ControlFlow;
use tap::Tap;

mod variable;

use crate::syntax::ast::visitor::{VisitWith, Visitor, VisitorMut};
pub use variable::*;

/// The `Declaration` Parse Node.
///
/// See the [module level documentation][self] for more information.
#[cfg_attr(feature = "deser", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum Declaration {
    /// See [`Function`]
    Function(Function),

    /// See [`Generator`]
    Generator(Generator),

    /// See [`AsyncFunction`]
    AsyncFunction(AsyncFunction),

    /// See [`AsyncGenerator`]
    AsyncGenerator(AsyncGenerator),

    /// See [`Class`]
    Class(Class),

    /// See [`LexicalDeclaration`]
    Lexical(LexicalDeclaration),
}

impl Declaration {
    /// Return the lexically declared names of a `Declaration`.
    ///
    /// The returned list may contain duplicates.
    ///
    /// If a declared name originates from a function declaration it is flagged as `true` in the returned list.
    ///
    /// More information:
    ///  - [ECMAScript specification][spec]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-static-semantics-lexicallydeclarednames
    pub(crate) fn lexically_declared_names(&self) -> Vec<(Identifier, bool)> {
        match self {
            Declaration::Function(f) => {
                if let Some(name) = f.name() {
                    vec![(name, true)]
                } else {
                    Vec::new()
                }
            }
            Declaration::Generator(g) => {
                if let Some(name) = g.name() {
                    vec![(name, false)]
                } else {
                    Vec::new()
                }
            }
            Declaration::AsyncFunction(af) => {
                if let Some(name) = af.name() {
                    vec![(name, false)]
                } else {
                    Vec::new()
                }
            }
            Declaration::AsyncGenerator(ag) => {
                if let Some(name) = ag.name() {
                    vec![(name, false)]
                } else {
                    Vec::new()
                }
            }
            Declaration::Class(cl) => {
                if let Some(name) = cl.name() {
                    vec![(name, false)]
                } else {
                    Vec::new()
                }
            }
            Declaration::Lexical(lexical) => {
                let mut names = Vec::new();
                for decl in lexical.variable_list().as_ref() {
                    match decl.binding() {
                        Binding::Identifier(ident) => {
                            names.push((*ident, false));
                        }
                        Binding::Pattern(pattern) => {
                            names.extend(pattern.idents().into_iter().map(|name| (name, false)));
                        }
                    }
                }
                names
            }
        }
    }

    /// Returns true if the node contains a identifier reference named 'arguments'.
    ///
    /// More information:
    ///  - [ECMAScript specification][spec]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-static-semantics-containsarguments
    // TODO: replace with a visitor
    pub(crate) fn contains_arguments(&self) -> bool {
        match self {
            Self::Function(_)
            | Self::Generator(_)
            | Self::AsyncGenerator(_)
            | Self::AsyncFunction(_) => false,
            Self::Lexical(decl) => decl.contains_arguments(),
            Self::Class(class) => class.contains_arguments(),
        }
    }

    /// Returns `true` if the node contains the given token.
    ///
    /// More information:
    ///  - [ECMAScript specification][spec]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-static-semantics-contains
    // TODO: replace with a visitor
    pub(crate) fn contains(&self, symbol: ContainsSymbol) -> bool {
        match self {
            Self::Function(_)
            | Self::Generator(_)
            | Self::AsyncGenerator(_)
            | Self::AsyncFunction(_) => false,
            Self::Class(class) => class.contains(symbol),
            Self::Lexical(decl) => decl.contains(symbol),
        }
    }
}

impl ToIndentedString for Declaration {
    fn to_indented_string(&self, interner: &Interner, indentation: usize) -> String {
        match self {
            Declaration::Function(f) => f.to_indented_string(interner, indentation),
            Declaration::Generator(g) => g.to_indented_string(interner, indentation),
            Declaration::AsyncFunction(af) => af.to_indented_string(interner, indentation),
            Declaration::AsyncGenerator(ag) => ag.to_indented_string(interner, indentation),
            Declaration::Class(c) => c.to_indented_string(interner, indentation),
            Declaration::Lexical(l) => l.to_interned_string(interner).tap_mut(|s| s.push(';')),
        }
    }
}

impl VisitWith for Declaration {
    fn visit_with<'a, V>(&'a self, visitor: &mut V) -> ControlFlow<V::BreakTy>
    where
        V: Visitor<'a>,
    {
        match self {
            Declaration::Function(f) => visitor.visit_function(f),
            Declaration::Generator(g) => visitor.visit_generator(g),
            Declaration::AsyncFunction(af) => visitor.visit_async_function(af),
            Declaration::AsyncGenerator(ag) => visitor.visit_async_generator(ag),
            Declaration::Class(c) => visitor.visit_class(c),
            Declaration::Lexical(ld) => visitor.visit_lexical_declaration(ld),
        }
    }

    fn visit_with_mut<'a, V>(&'a mut self, visitor: &mut V) -> ControlFlow<V::BreakTy>
    where
        V: VisitorMut<'a>,
    {
        match self {
            Declaration::Function(f) => visitor.visit_function_mut(f),
            Declaration::Generator(g) => visitor.visit_generator_mut(g),
            Declaration::AsyncFunction(af) => visitor.visit_async_function_mut(af),
            Declaration::AsyncGenerator(ag) => visitor.visit_async_generator_mut(ag),
            Declaration::Class(c) => visitor.visit_class_mut(c),
            Declaration::Lexical(ld) => visitor.visit_lexical_declaration_mut(ld),
        }
    }
}