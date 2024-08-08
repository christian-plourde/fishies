use ratatui::{layout::Rect, style::Color, widgets::canvas::{Painter, Shape}};

pub struct Fisherman {
    pub frame: Rect,
    pub shoe_color: Color,
    pub pants_color: Color,
    pub shirt_color: Color,
    pub skin_color: Color,
    pub rod_color: Color,
    rod_geometry: Vec<(f64, f64)>,
}

impl Fisherman {
    pub fn new(frame: Rect, pants_color: Color, rod_color: Color, shirt_color: Color, skin_color: Color, shoe_color: Color) -> Self {
        let shoe_x = (frame.x as f64) + (frame.width as f64)/4.0 - 7.0;
        let shoe_y = (frame.height as f64)/2.0 + 6.0;
        let geometry = vec![
            (shoe_x + 4.0, shoe_y + 9.0),
            (shoe_x + 5.0, shoe_y + 10.0),
            (shoe_x + 6.0, shoe_y + 11.0),
            (shoe_x + 7.0, shoe_y + 12.0),
            (shoe_x + 8.0, shoe_y + 13.0),
            (shoe_x + 9.0, shoe_y + 14.0),
            (shoe_x + 10.0, shoe_y + 15.0),
        ];
        Self {
            frame, 
            pants_color,
            rod_color,
            shirt_color,
            skin_color,
            shoe_color,
            rod_geometry: geometry,
        }
    }

    pub fn rotate_rod(&mut self, angle: f64) {
        let rod_base = self.rod_geometry[0];
        self.rod_geometry = self.rod_geometry.iter_mut().map(|point| {
            let translated_x = point.0 - rod_base.0;
            let translated_y = point.1 - rod_base.1;
            (translated_x*f64::cos(angle) - translated_y*f64::sin(angle) + rod_base.0, translated_x*f64::sin(angle) + translated_y*f64::cos(angle) + rod_base.1)
        }).collect();
    }
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

        for geometry in self.rod_geometry.clone() {
            let (rod_x, rod_y) = painter.get_point(geometry.0, geometry.1).unwrap();
            painter.paint(rod_x, rod_y, self.rod_color);
        }
    }
}
