use std::time::{Duration, Instant};
use ratatui::{style::Color, widgets::canvas::{Painter, Shape}};
use rand::Rng;

pub struct Fish {
    pub x: u16,
    pub y: u16,
    pub facing_right: bool,
    pub body_color: Color,
    pub mouth_color: Color,
    pub created: Instant,
    pub lifespan: Duration,
}

impl Fish {
    pub fn new(x: u16, y: u16, body_color: Color, mouth_color: Color) -> Self {
        Self {
            x,
            y,
            body_color,
            mouth_color,
            facing_right: if rand::thread_rng().gen_range(0.0..1.0) > 0.5 {true} else {false}, 
            created: Instant::now(),
            lifespan: Duration::from_secs(10)
        }
    }

    pub fn is_dead(&self) -> bool {
        if Instant::now() > self.created + self.lifespan {
            return true;
        }

        return false;
    }
}

impl Shape for Fish {
    fn draw(&self, painter: &mut Painter) {
        let size: u16 = 7;

        if self.facing_right && !(self.x as i16 - size as i16 > 0 && self.y as i16 - size as i16 > 0) {
            return;
        }

        if !self.facing_right && !(self.y as i16 - size as i16 > 0) {
            return;
        }

        let (x, y) = painter.get_point(self.x.into(), self.y.into()).unwrap();
        painter.paint(x, y, self.mouth_color);

        let x_range = if self.facing_right {self.x - size + 2..self.x - 1} else {self.x + 1..self.x + size};
        let y_range = self.y - size/2..self.y + size/2;

        for x in x_range {
            for y in y_range.clone() {
                match painter.get_point(x.into(), y.into()) {
                    Some((paint_x, paint_y)) => painter.paint(paint_x, paint_y, self.body_color),
                    None => (),
                }
            }
        }

        let tail_x_range = if self.facing_right {self.x - size..self.x - size + 1} else {self.x + size..self.x + size + 1};

        for x in tail_x_range {
            for y in y_range.clone() {
                if y >= y_range.start + 2 && y <= y_range.end - 2 { 
                    match painter.get_point(x.into(), y.into()) {
                        Some((paint_x, paint_y)) => painter.paint(paint_x, paint_y, self.body_color),
                        None => (),
                    }
                }
            }
        }
    }
}
