use std::path::PathBuf;

use iced::Command;

use crate::StyleSheet;

/// A command that loads a stylesheet from disk.
pub fn load_stylesheet<P: Into<PathBuf>, M>(
    path: P,
    on_load: impl FnOnce(StyleSheet) -> M + Send + 'static,
) -> Command<M> {
    let path = path.into();

    Command::perform(
        async move { StyleSheet::load(path).expect("Failed to load stylesheet!") },
        on_load,
    )
}

/// A command that parses a stylesheet from memory.
pub fn parse_stylesheet<M>(
    style: &'static str,
    on_load: impl FnOnce(StyleSheet) -> M + Send + 'static,
) -> Command<M> {
    Command::perform(
        async move { StyleSheet::parse(style).expect("Failed to parse stylesheet!") },
        on_load,
    )
}
