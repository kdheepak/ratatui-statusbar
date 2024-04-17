use ratatui::prelude::*;
use ratatui::widgets::WidgetRef;

#[derive(Debug, Default)]
pub struct StatusBarSection<'a> {
    content: Line<'a>,
    style: Style,
}

#[derive(Debug, Default)]
pub struct StatusBar<'a> {
    sections: [StatusBarSection<'a>; 6],
}

impl<'a> StatusBar<'a> {
    pub fn new() -> StatusBar<'a> {
        StatusBar {
            sections: Default::default(),
        }
    }

    pub fn section(mut self, index: usize, content: String, style: Style) -> Self {
        if let Some(section) = self.sections.get_mut(index) {
            section.content = content.into();
            section.style = style;
        }
        self
    }

    pub fn content(mut self, index: usize, content: String) -> Self {
        if let Some(section) = self.sections.get_mut(index) {
            section.content = content.into();
        }
        self
    }

    pub fn style(mut self, index: usize, style: Style) -> Self {
        if let Some(section) = self.sections.get_mut(index) {
            section.style = style;
        }
        self
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

        buf.set_span(
            area.left(),
            area.top(),
            &Span::from(" ".repeat(area.width as usize)),
            area.width,
        );

        let mut start_x = area.left();

        for (i, section) in self.sections.iter().enumerate() {
            let section_width = section.content.width();

            if start_x + (section_width as u16) > area.right() {
                break;
            }

            buf.set_line(start_x, area.top(), &section.content, section_width as u16);

            start_x += section_width as u16;
            if i < self.sections.len().saturating_sub(1) {
                start_x += 1;
            }
        }
    }
}

