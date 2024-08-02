use ratatui::{layout::Rect, style::Color, widgets::canvas::{Painter, Shape}};

pub struct Dock {
    pub frame: Rect,
    pub color: Color,
}

impl Shape for Dock {
    fn draw(&self, painter: &mut Painter) {
        for x in self.frame.x..self.frame.x + self.frame.width/4 {
            for y in ((self.frame.height as f64)/2.0) as u16..(((self.frame.height as f64)/2.0) as u16) + 5 {
                let (dock_x, dock_y) = painter.get_point(x.into(), y.into()).unwrap();
                painter.paint(dock_x, dock_y, self.color);
            }
        }

        for x in self.frame.x + self.frame.width/4 - 5..self.frame.x + self.frame.width/4 {
            for y in self.frame.height/3..self.frame.height/2 {
                let (dock_x, dock_y) = painter.get_point(x.into(), y.into()).unwrap();
                painter.paint(dock_x, dock_y, self.color);
            }
        }
    }
}
