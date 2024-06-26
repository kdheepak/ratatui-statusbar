//! # `ratatui-statusbar` Crate
//!
//! This crate provides components for creating status bars within Ratatui applications.
//!
//! ## Features
//! - Define status bar layouts with any number of sections
//! - Customizable flex layout and spacing between sections

use itertools::Itertools;
use ratatui::layout::Flex;
use ratatui::prelude::*;
use ratatui::widgets::WidgetRef;
use thiserror::Error;

/// An enumeration of potential errors that can impact the [`StatusBar`] operations.
#[derive(Error, Debug)]
pub enum StatusBarError {
    /// The requested index does not exist.
    #[error("Index out of bounds: {0}")]
    IndexOutOfBounds(usize),
}

/// A representation of a single section in a [`StatusBar`]
/// including optional decorators (pre/post separators) around the content.
///
/// # Examples
/// ```
/// let section = StatusBarSection::default()
///     .pre_separator(" | ")
///     .content("Section Content")
///     .post_separator(" | ");
/// ```
#[derive(Debug, Default, Clone)]
pub struct StatusBarSection<'a> {
    pre_separator: Option<Span<'a>>,
    content: Line<'a>,
    post_separator: Option<Span<'a>>,
}

impl<'a> StatusBarSection<'a> {
    /// Associates a pre-separator with the section.
    #[must_use]
    pub fn pre_separator(mut self, separator: impl Into<Span<'a>>) -> Self {
        self.pre_separator = Some(separator.into());
        self
    }

    /// Sets the main content of the section.
    #[must_use]
    pub fn content(mut self, content: impl Into<Line<'a>>) -> Self {
        self.content = content.into();
        self
    }

    /// Associates a post-separator with the section.
    #[must_use]
    pub fn post_separator(mut self, separator: impl Into<Span<'a>>) -> Self {
        self.post_separator = Some(separator.into());
        self
    }
}

impl<'a> From<Line<'a>> for StatusBarSection<'a> {
    fn from(line: Line<'a>) -> Self {
        StatusBarSection {
            pre_separator: None,
            content: line,
            post_separator: None,
        }
    }
}

impl<'a> From<Span<'a>> for StatusBarSection<'a> {
    fn from(span: Span<'a>) -> Self {
        StatusBarSection {
            pre_separator: None,
            content: span.into(),
            post_separator: None,
        }
    }
}

impl<'a> From<&'a str> for StatusBarSection<'a> {
    fn from(s: &'a str) -> Self {
        StatusBarSection {
            pre_separator: None,
            content: s.into(),
            post_separator: None,
        }
    }
}

/// A customizable [`StatusBar`] that can contain multiple sections.
///
/// # Examples
/// ```
/// let status_bar = StatusBar::new(3)
///     .flex(Flex::Center)
///     .spacing(2)
///     .section(0, "Left Section")?
///     .section(1, "Center Section")?
///     .section(2, "Right Section")?;
/// ```
#[derive(Debug, Default)]
pub struct StatusBar<'a> {
    sections: Vec<StatusBarSection<'a>>,
    flex: Flex,
    spacing: u16,
}

impl<'a> StatusBar<'a> {
    /// Initializes a new [`StatusBar`] with a specified number of sections, all set to default.
    #[must_use]
    pub fn new(nsections: usize) -> Self {
        Self {
            sections: vec![StatusBarSection::default(); nsections],
            flex: Flex::default(),
            spacing: 1,
        }
    }

    /// Configures the flex layout mode of the sections in the [`StatusBar`].
    #[must_use]
    pub fn flex(mut self, flex: Flex) -> Self {
        self.flex = flex;
        self
    }

    /// Sets the spacing between [`StatusBar`] sections.
    #[must_use]
    pub fn spacing(mut self, spacing: impl Into<u16>) -> Self {
        self.spacing = spacing.into();
        self
    }

    /// Modifies a specific section within the [`StatusBar`] based on its index.
    ///
    /// # Errors
    ///
    /// This function will return an error if the index is out of bounds, using the [`StatusBarError`] enum.
    pub fn section(
        mut self,
        index: usize,
        section: impl Into<StatusBarSection<'a>>,
    ) -> Result<Self, StatusBarError> {
        if let Some(s) = self.sections.get_mut(index) {
            *s = section.into();
            Ok(self)
        } else {
            Err(StatusBarError::IndexOutOfBounds(index))
        }
    }
}

impl Widget for StatusBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_ref(area, buf);
    }
}

impl WidgetRef for StatusBar<'_> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        if area.is_empty() {
            return;
        }

        let layout = Layout::horizontal(
            self.sections
                .iter()
                .map(|s| Constraint::Length(u16::try_from(s.content.width()).unwrap())),
        )
        .flex(self.flex)
        .spacing(self.spacing);

        let areas = layout.split(area);
        let areas = areas.iter().collect_vec();

        for (section, rect) in self.sections.iter().zip(areas) {
            buf.set_line(
                rect.left(),
                rect.top(),
                &section.content,
                u16::try_from(section.content.width()).unwrap(),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use ratatui::backend::TestBackend;

    use super::*;

    #[test]
    fn test_print_statusbar() -> color_eyre::Result<()> {
        let mut buf = Vec::new();
        let backend = CrosstermBackend::new(&mut buf);
        let mut terminal = Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Inline(1),
            },
        )?;
        let status_bar = StatusBar::new(2).section(0, "hello")?.section(1, "world")?;
        terminal
            .draw(|f| f.render_widget(status_bar, f.size()))
            .unwrap();
        drop(terminal);
        let view = String::from_utf8(buf).unwrap();
        println!("{view}");
        Ok(())
    }

    #[test]
    fn render_default() -> color_eyre::Result<()> {
        let area = Rect::new(0, 0, 15, 1);
        let backend = TestBackend::new(area.width, area.height);
        let status_bar = StatusBar::new(2).section(0, "hello")?.section(1, "world")?;
        let mut terminal = Terminal::new(backend)?;
        terminal.draw(|f| f.render_widget(status_bar, f.size()))?;
        let expected = Buffer::with_lines(vec!["hello world    "]);
        terminal.backend().assert_buffer(&expected);
        Ok(())
    }
}
