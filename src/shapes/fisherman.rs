use ratatui::{layout::Rect, style::Color, widgets::canvas::{Painter, Shape}};

pub struct Fisherman {
    pub frame: Rect,
    pub shoe_color: Color,
    pub pants_color: Color,
    pub shirt_color: Color,
    pub skin_color: Color,
    pub rod_color: Color,
    rod_geometry: Vec<(f64, f64)>,
    current_rotation: f64,
    hook_location: (f64, f64),
    rotation_velocity: f64,
    hook_velocity: (f64, f64),
    surface_level: f64,
}

impl Fisherman {
    pub fn get_base_geometry(frame: Rect) -> Vec<(f64, f64)> {
        let shoe_x = (frame.x as f64) + (frame.width as f64)/4.0 - 7.0;
        let shoe_y = (frame.height as f64)/2.0 + 6.0;
        vec![
            (shoe_x + 4.0, shoe_y + 9.0),
            (shoe_x + 5.0, shoe_y + 10.0),
            (shoe_x + 6.0, shoe_y + 11.0),
            (shoe_x + 7.0, shoe_y + 12.0),
            (shoe_x + 8.0, shoe_y + 13.0),
            (shoe_x + 9.0, shoe_y + 14.0),
            (shoe_x + 10.0, shoe_y + 15.0),
        ]
    }

    pub fn get_base_hook_location(frame: Rect) -> (f64, f64) {
        let (hook_x, hook_y) = Fisherman::get_base_geometry(frame)[Fisherman::get_base_geometry(frame).len() - 1];
        (hook_x, hook_y - 3.0)
    }

    fn get_rod_end(&self) -> (f64, f64) {
        self.rod_geometry[self.rod_geometry.len() - 1]
    }

    fn get_line_geometry(&self) -> Vec<(f64, f64)> {
        let rod_end = self.get_rod_end();
        if self.hook_location.0 == rod_end.0 {
            let mut locations: Vec<(f64, f64)> = vec![];
            for y in self.hook_location.1 as u16..rod_end.1 as u16 {
                locations.push((self.hook_location.0, y as f64));
            };
            return locations;
        }

        else {
            let slope = (self.hook_location.1 - rod_end.1)/(self.hook_location.0 - rod_end.0);
            let intercept = rod_end.1 - slope*rod_end.0;

            let mut locations: Vec<(f64, f64)> = vec![];
            let mut current_x = rod_end.0;
            while current_x + 0.1 < self.hook_location.0 {
                current_x += 0.1;
                locations.push((current_x, slope*current_x + intercept));
            }
            return locations;
        }
    }

    pub fn new(frame: Rect, surface_level: f64, pants_color: Color, rod_color: Color, shirt_color: Color, skin_color: Color, shoe_color: Color) -> Self {
        Self {
            frame, 
            pants_color,
            rod_color,
            shirt_color,
            skin_color,
            shoe_color,
            rod_geometry: Fisherman::get_base_geometry(frame),
            current_rotation: 0.0,
            hook_location: Fisherman::get_base_hook_location(frame),
            rotation_velocity: 0.0,
            hook_velocity: (0.0, 0.0),
            surface_level,
        }
    }

    pub fn cast(&mut self, ready: bool) {
        if ready {
            self.rotation_velocity = -0.7;
            self.hook_velocity = (8.0, -8.0);
        }

        else {
            self.rotation_velocity = 0.3;
        }
    }

    pub fn rotate_rod(&mut self) {
        if self.current_rotation + self.rotation_velocity < std::f64::consts::PI/4.0 && self.current_rotation >= 0.0 {
            let rod_base = self.rod_geometry[0];
            self.current_rotation += self.rotation_velocity;
            self.rod_geometry = self.rod_geometry.iter_mut().map(|point| {
                let translated_x = point.0 - rod_base.0;
                let translated_y = point.1 - rod_base.1;
                (translated_x*f64::cos(self.rotation_velocity) - translated_y*f64::sin(self.rotation_velocity) + rod_base.0, translated_x*f64::sin(self.rotation_velocity) + translated_y*f64::cos(self.rotation_velocity) + rod_base.1)
            }).collect();
            let translated_hook_x = self.hook_location.0 - rod_base.0;
            let translated_hook_y = self.hook_location.1 - rod_base.1;
            self.hook_location = (translated_hook_x*f64::cos(self.rotation_velocity) - translated_hook_y*f64::sin(self.rotation_velocity) + rod_base.0, translated_hook_x*f64::sin(self.rotation_velocity) + translated_hook_y*f64::cos(self.rotation_velocity) + rod_base.1);
        }

        else if self.current_rotation > 0.0 {
            self.cast(true);
        }

        else {
            self.rotation_velocity = 0.0;
            self.current_rotation = 0.0;
            self.rod_geometry = Self::get_base_geometry(self.frame);
            self.hook_location = Self::get_base_hook_location(self.frame);
        }
    }

    pub fn move_hook(&mut self) {
        self.hook_location = (self.hook_location.0 + self.hook_velocity.0, self.hook_location.1 + self.hook_velocity.1);
        if self.hook_location.1 < self.surface_level {
            self.hook_location.1 = self.surface_level;
            self.hook_velocity = (-4.0, 0.0);
        }

        if self.hook_location.0 < self.get_rod_end().0 {
            self.hook_location.0 = self.get_rod_end().0;
            self.hook_velocity = (0.0, 8.0);
        }

        if self.hook_location.0 <= Self::get_base_hook_location(self.frame).0 && self.hook_location.1 >= Self::get_base_hook_location(self.frame).1 {
            self.hook_location = Self::get_base_hook_location(self.frame);
            self.hook_velocity = (0.0, 0.0);
        }
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

        for geometry in self.get_line_geometry() {
            let get_point_result = painter.get_point(geometry.0, geometry.1);
            if get_point_result.is_some() {
                let (line_x, line_y) = get_point_result.unwrap();
                painter.paint(line_x, line_y, Color::White);
            }
        }

        for geometry in self.rod_geometry.clone() {
            let (rod_x, rod_y) = painter.get_point(geometry.0, geometry.1).unwrap();
            painter.paint(rod_x, rod_y, self.rod_color);
        }

        let (hook_x, hook_y) = painter.get_point(self.hook_location.0, self.hook_location.1).unwrap();
        painter.paint(hook_x, hook_y, Color::Red);
    }
}
