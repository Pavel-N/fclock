use tui::{widgets::{Block, BorderType, Borders}, style::{Style, Color}, layout::Rect, Frame, backend::Backend};


pub enum FBlockColor {
    None,
    Hours,
    Minutes,
    Both
}


pub struct FBlock {
    pub color: FBlockColor,
    borders: bool
}

impl Default for FBlock {
    fn default() -> Self {
        Self { 
            borders: false,
            color: FBlockColor::None
        }
    }
    // TODO fns: with colors
}

impl FBlock {
    pub fn with_borders(mut self) -> Self {
        self.borders = true;
        self
    }

    pub fn draw<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let mut block = Block::default()
            .style(Style::default().bg(match self.color {
                FBlockColor::None => Color::White,
                FBlockColor::Hours => Color::Red,
                FBlockColor::Minutes => Color::Green,
                FBlockColor::Both => Color::Blue,
            }));

        if self.borders {
            block = block
                .border_type(BorderType::Rounded)
                .border_style(Style::default().bg(Color::Reset))
                .borders(Borders::all())
        }
        
        f.render_widget(block, area);
    }
}
