use ratatui::{layout::Rect, style::Color, widgets::canvas::{Painter, Shape}};

pub struct Fisherman {
    pub frame: Rect,
    pub shoe_color: Color,
    pub pants_color: Color,
    pub shirt_color: Color,
    pub skin_color: Color,
    pub rod_color: Color,
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
            painter.paint(x, (shoe_1_y - 7).into(), self.shoe_color);
            painter.paint(x, (shoe_1_y - 8).into(), self.shirt_color);
            painter.paint(x, (shoe_1_y - 9).into(), self.shirt_color);
            painter.paint(x, (shoe_1_y - 10).into(), self.shirt_color);
            painter.paint(x, (shoe_1_y - 11).into(), self.shirt_color);
            painter.paint(x, (shoe_1_y - 12).into(), self.shirt_color);
            painter.paint(x, (shoe_1_y - 13).into(), self.shirt_color);
        }

        painter.paint(shoe_2_x, (shoe_1_y - 14).into(), self.skin_color);
        painter.paint(shoe_2_x + 1, (shoe_1_y - 14).into(), self.skin_color);
        painter.paint(shoe_2_x + 2, (shoe_1_y - 14).into(), self.skin_color);
        painter.paint(shoe_2_x, (shoe_1_y - 15).into(), self.skin_color);
        painter.paint(shoe_2_x + 1, (shoe_1_y - 15).into(), self.skin_color);
        painter.paint(shoe_2_x + 2, (shoe_1_y - 15).into(), self.skin_color);
        painter.paint(shoe_2_x, (shoe_1_y - 16).into(), self.skin_color);
        painter.paint(shoe_2_x + 1, (shoe_1_y - 16).into(), self.skin_color);
        painter.paint(shoe_2_x + 2, (shoe_1_y - 16).into(), self.skin_color);
        painter.paint(shoe_2_x, (shoe_1_y - 17).into(), self.shoe_color);
        painter.paint(shoe_2_x + 1, (shoe_1_y - 17).into(), self.shoe_color);
        painter.paint(shoe_2_x + 2, (shoe_1_y - 17).into(), self.shoe_color);
        painter.paint(shoe_2_x + 3, (shoe_1_y - 17).into(), self.shoe_color);
        painter.paint(shoe_2_x, (shoe_1_y - 18).into(), self.shoe_color);
        painter.paint(shoe_2_x + 1, (shoe_1_y - 18).into(), self.shoe_color);
        painter.paint(shoe_2_x + 2, (shoe_1_y - 18).into(), self.shoe_color);

        painter.paint(shoe_1_x + 1, (shoe_1_y - 12).into(), self.shirt_color);
        painter.paint(shoe_1_x + 1, (shoe_1_y - 11).into(), self.shirt_color);
        painter.paint(shoe_1_x + 2, (shoe_1_y - 10).into(), self.shirt_color);
        painter.paint(shoe_1_x + 2, (shoe_1_y - 9).into(), self.shirt_color);
        painter.paint(shoe_1_x + 3, (shoe_1_y - 8).into(), self.shirt_color);
        painter.paint(shoe_1_x + 3, (shoe_1_y - 8).into(), self.shirt_color);

        painter.paint(shoe_1_x + 4, (shoe_1_y - 9).into(), self.rod_color);
        painter.paint(shoe_1_x + 5, (shoe_1_y - 10).into(), self.rod_color);
        painter.paint(shoe_1_x + 6, (shoe_1_y - 11).into(), self.rod_color);
        painter.paint(shoe_1_x + 7, (shoe_1_y - 12).into(), self.rod_color);
        painter.paint(shoe_1_x + 8, (shoe_1_y - 13).into(), self.rod_color);
        painter.paint(shoe_1_x + 9, (shoe_1_y - 14).into(), self.rod_color);
        painter.paint(shoe_1_x + 10, (shoe_1_y - 15).into(), self.rod_color);
    }
}
