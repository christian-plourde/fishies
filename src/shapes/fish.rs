use std::time::{Duration, Instant};
use ratatui::{style::Color, widgets::canvas::{Painter, Shape}};
use rand::Rng;

pub struct Fish {
    size: u16,
    pub x: u16,
    pub y: u16,
    pub rotation: f64,
    pub surface_level: u16,
    pub vx: i16,
    pub vy: i16,
    pub facing_right: bool,
    pub body_color: Color,
    pub mouth_color: Color,
    pub created: Instant,
    pub lifespan: Duration,
    pub hooked: bool,
}

impl Fish {
    pub fn new(x: u16, y: u16, surface_level: u16, body_color: Color, mouth_color: Color) -> Self {
        Self {
            size: 7,
            x,
            y,
            rotation: 0.0,
            surface_level,
            vx: 1,
            vy: 1,
            body_color,
            mouth_color,
            facing_right: if rand::thread_rng().gen_range(0.0..1.0) > 0.5 {true} else {false}, 
            created: Instant::now(),
            lifespan: Duration::from_secs(20),
            hooked: false,
        }
    }

    pub fn is_dead(&self) -> bool {
        if Instant::now() > self.created + self.lifespan {
            return true;
        }

        return false;
    }

    pub fn get_geometry(&self) -> Vec<(f64, f64, Color)> {
        if self.facing_right && !(self.x as i16 - self.size as i16 > 0 && self.y as i16 - self.size as i16 > 0) {
            return vec![];
        }

        if !self.facing_right && !(self.y as i16 - self.size as i16 > 0) {
            return vec![];
        }

        let mut points: Vec<(f64, f64, Color)> = vec![(self.x as f64, self.y as f64, self.mouth_color)];
        let x_range = if self.facing_right {self.x - self.size + 2..self.x - 1} else {self.x + 1..self.x + self.size};
        let y_range = self.y - self.size/2..self.y + self.size/2;

        for x in x_range {
            for y in y_range.clone() {
                points.push((x.into(), y.into(), self.body_color));
            }
        }

        let tail_x_range = if self.facing_right {self.x - self.size..self.x - self.size + 1} else {self.x + self.size..self.x + self.size + 1};

        for x in tail_x_range {
            for y in y_range.clone() {
                if y >= y_range.start + 2 && y <= y_range.end - 2 { 
                    points.push((x.into(), y.into(), self.body_color));
                }
            }
        };

        let rotation = if !self.facing_right {self.rotation*-1.0} else {self.rotation};
        points = points.iter_mut().map(|point| {
            let translated_x = point.0 - (self.x as f64);
            let translated_y = point.1 - (self.y as f64);
            (translated_x*f64::cos(rotation) - translated_y*f64::sin(rotation) + (self.x as f64), translated_x*f64::sin(rotation) + translated_y*f64::cos(rotation) + (self.y as f64), point.2)
        }).collect();

        return points;
    }

    pub fn r#move(&mut self) {
        if self.facing_right {
            self.x += self.vx as u16;
        }

        else {
            if self.x as i16 - self.vx > 0 {
                self.x -= self.vx as u16;
            }
        }
        
        if rand::thread_rng().gen_bool(0.5) {
            if self.y != self.surface_level {
                self.y += self.vy as u16;
            }
        }

        else {
            if self.y != 0 {
                self.y -= self.vy as u16;
            }
        }
    }
}

impl Shape for Fish {
    fn draw(&self, painter: &mut Painter) {
        for point in self.get_geometry() {
            match painter.get_point(point.0, point.1) {
                Some((paint_x, paint_y)) => painter.paint(paint_x, paint_y, point.2),
                None => ()
            }
        }
    }
}
