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
    style: Style,
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
        content: String,
        style: Style,
    ) -> Result<Self, StatusBarError> {
        if let Some(section) = self.sections.get_mut(index) {
            section.content = content.into();
            section.style = style;
            Ok(self)
        } else {
            Err(StatusBarError::IndexOutOfBounds(index))
        }
    }

    pub fn content(mut self, index: usize, content: String) -> Result<Self, StatusBarError> {
        if let Some(section) = self.sections.get_mut(index) {
            section.content = content.into();
            Ok(self)
        } else {
            Err(StatusBarError::IndexOutOfBounds(index))
        }
    }

    pub fn style(mut self, index: usize, style: Style) -> Result<Self, StatusBarError> {
        if let Some(section) = self.sections.get_mut(index) {
            section.style = style;
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
    use ratatui::assert_buffer_eq;
    use ratatui::buffer::Buffer;
    use ratatui::layout::Rect;

    #[test]
    fn render_default() -> color_eyre::Result<()> {
        let status_bar = StatusBar::new(2)
            .section(0, "Hello".into(), Style::default())?
            .section(1, "World".into(), Style::default())?;
        let expected = Buffer::with_lines(vec!["Hello          World          "]);
        let area = Rect::new(0, 0, 30, 1);
        let mut actual = Buffer::empty(area);
        status_bar.render_ref(area, &mut actual);
        assert_buffer_eq!(actual, expected);
        Ok(())
    }
}
