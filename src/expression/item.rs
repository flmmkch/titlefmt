use super::{Expression, FunctionCall};
use crate::metadata;

/// An item of a formatting expression.
pub enum Item<'a, T: metadata::Provider>
where
    T: 'a,
{
    /// Simple text.
    Text(String),
    /// Metadata tag.
    ///
    /// Defined in the expression string between percentage signs: `%tag_name%`.
    Tag(String),
    /// Optional sub-expression.
    ///
    /// Returns an empty string if none of the tags in the sub-expression was found.
    /// Defined in the expression string between square brackets: `[this is an optional sub-expression with a %tag%]`.
    OptionalExpr(Box<Expression<'a, T>>),
    /// A function call.
    ///
    /// Defined in the expression string with a dollar sign and a comma-separated argument list between parentheses: `$function(arg1, arg2, ...)`.
    Function(FunctionCall<'a, T>),
}
