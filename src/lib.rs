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
    content: Line<'a>,
}

#[derive(Debug, Default)]
pub struct StatusBar<'a> {
    sections: Vec<StatusBarSection<'a>>,
}

impl<'a> StatusBar<'a> {
    pub fn new(nsections: usize) -> StatusBar<'a> {
        StatusBar {
            sections: vec![StatusBarSection::default(); nsections],
        }
    }

    pub fn section(
        mut self,
        index: usize,
        content: impl Into<Line<'a>>,
    ) -> Result<Self, StatusBarError> {
        if let Some(section) = self.sections.get_mut(index) {
            section.content = content.into();
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

        let layout = Layout::horizontal(vec![
            Constraint::Ratio(1, self.sections.len() as u32);
            self.sections.len()
        ]);

        let areas = layout.split(area);

        for (i, segment) in areas.iter().enumerate() {
            buf.set_line(
                segment.left(),
                area.top(),
                &self.sections[i].content,
                self.sections[i].content.width() as u16,
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
        terminal
            .draw(|f| f.render_widget("hello world", f.size()))
            .unwrap();
        drop(terminal);
        let view = String::from_utf8(buf).unwrap();
        println!("{view}");
        Ok(())
    }
}
