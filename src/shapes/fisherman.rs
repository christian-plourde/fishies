use ratatui::{layout::Rect, style::Color, widgets::canvas::{Painter, Shape}};

pub struct Fisherman {
    pub frame: Rect,
    pub shoe_color: Color,
    pub pants_color: Color,
}

impl Shape for Fisherman {
    fn draw(&self, painter: &mut Painter) {
        let (shoe_1_x, shoe_1_y) = painter.get_point((self.frame.x as f64) + (self.frame.width as f64)/4.0 - 7.0, (self.frame.height as f64)/2.0 + 6.0).unwrap();
        let (shoe_2_x, shoe_2_y) = painter.get_point((self.frame.x as f64) + (self.frame.width as f64)/4.0 - 10.0, (self.frame.height as f64)/2.0 + 6.0).unwrap();
        painter.paint(shoe_1_x.into(), shoe_1_y.into(), self.shoe_color);
        painter.paint(shoe_1_x.into(), (shoe_1_y - 1).into(), self.shoe_color);
        painter.paint((shoe_1_x + 1).into(), shoe_1_y.into(), self.shoe_color);

        painter.paint(shoe_2_x.into(), shoe_2_y.into(), self.shoe_color);
        painter.paint(shoe_2_x.into(), (shoe_2_y - 1).into(), self.shoe_color);
        painter.paint((shoe_2_x + 1).into(), shoe_2_y.into(), self.shoe_color);

        painter.paint(shoe_1_x, (shoe_1_y - 2).into(), self.pants_color);
        painter.paint(shoe_1_x, (shoe_1_y - 3).into(), self.pants_color);
        painter.paint(shoe_1_x, (shoe_1_y - 4).into(), self.pants_color);

        painter.paint(shoe_2_x, (shoe_2_y - 2).into(), self.pants_color);
        painter.paint(shoe_2_x, (shoe_2_y - 3).into(), self.pants_color);
        painter.paint(shoe_2_x, (shoe_2_y - 4).into(), self.pants_color);

        for x in shoe_2_x..=shoe_1_x {
            painter.paint(x, (shoe_1_y - 5).into(), self.pants_color);
            painter.paint(x, (shoe_1_y - 6).into(), self.pants_color);
        }
    }
}
