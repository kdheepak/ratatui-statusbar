use itertools::Itertools;
use ratatui::layout::Flex;
use ratatui::prelude::*;
use ratatui::widgets::WidgetRef;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StatusBarError {
    #[error("Index out of bounds: {0}")]
    IndexOutOfBounds(usize),
}

#[derive(Debug, Default, Clone)]
pub struct StatusBarSection<'a> {
    pre_separator: Option<Span<'a>>,
    content: Line<'a>,
    post_separator: Option<Span<'a>>,
}

impl<'a> StatusBarSection<'a> {
    pub fn pre_separator(mut self, separator: impl Into<Span<'a>>) -> Self {
        self.pre_separator = Some(separator.into());
        self
    }

    pub fn content(mut self, content: impl Into<Line<'a>>) -> Self {
        self.content = content.into();
        self
    }

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

#[derive(Debug, Default)]
pub struct StatusBar<'a> {
    sections: Vec<StatusBarSection<'a>>,
    flex: Flex,
    spacing: u16,
}

impl<'a> StatusBar<'a> {
    pub fn new(nsections: usize) -> Self {
        let sections = vec![StatusBarSection::default(); nsections];
        Self {
            sections,
            flex: Flex::Start,
            spacing: 1,
        }
    }

    pub fn flex(mut self, flex: Flex) -> Self {
        self.flex = flex;
        self
    }

    pub fn spacing(mut self, spacing: impl Into<u16>) -> Self {
        self.spacing = spacing.into();
        self
    }

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
                .map(|s| Constraint::Length(s.content.width() as u16)),
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
                section.content.width() as u16,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_default() -> color_eyre::Result<()> {
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
}
