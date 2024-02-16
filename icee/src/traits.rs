use crate::StyleSheet;

/// A widget extension that allows styling using icee stylesheets.
pub trait WithStyleSheet<'a> {
    /// The output type.
    type Output;

    /// Style this widget without a unique id.
    fn with_stylesheet(self, stylesheet: &'a StyleSheet) -> Self::Output;
    /// Style this widget with the given unique id.
    fn with_stylesheet_id(self, stylesheet: &'a StyleSheet, id: &'static str) -> Self::Output;
}
