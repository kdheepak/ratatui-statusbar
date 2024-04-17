use ratatui::prelude::*;

#[derive(Debug, Default)]
pub struct StatusBarSection {
    content: String,
}

#[derive(Debug, Default)]
pub struct StatusBar {
    sections: [StatusBarSection; 6],
}

impl StatusBar {
    pub fn new() -> StatusBar {
        StatusBar {
            sections: Default::default(),
        }
    }

    pub fn section(mut self, index: usize, content: String) -> Self {
        if let Some(section) = self.sections.get_mut(index) {
            section.content = content;
        }
        self
    }
}

impl Widget for StatusBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Clear the entire StatusBar area
        buf.set_span(
            area.left(),
            area.top(),
            &Span::from(" ".repeat(area.width as usize)),
            area.width,
        );

        let mut start_x = area.left();

        for (index, section) in self.sections.iter().enumerate() {
            let section_width = section.content.len();

            if start_x + (section_width as u16) > area.right() {
                break;
            }

            buf.set_stringn(
                start_x,
                area.top(),
                &section.content,
                section_width,
                Style::default(),
            );

            start_x += section_width as u16;
            if index < self.sections.len() - 1 {
                start_x += 1;
            }
        }
    }
}

