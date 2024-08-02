use ratatui::{layout::Rect, style::Color, widgets::canvas::{Painter, Shape}};
use rand::Rng;

pub struct Water {
    pub frame: Rect,
    pub color: Color,
}

impl Water {
    pub fn bottom(&self) -> u16 {
        self.frame.y
    }

    pub fn top(&self) -> u16 {
        self.frame.y + self.frame.height/3
    }

    pub fn left(&self) -> u16 {
        self.frame.x
    }

    pub fn right(&self) -> u16 {
        self.frame.x + self.frame.width
    }
}

impl Shape for Water {
    fn draw(&self, painter: &mut Painter) {
        let wave_interval = rand::thread_rng().gen_range(6..12);
        for x in self.left()..self.right() {
            if x%wave_interval == 0 {
                let (wave_x, wave_y) = painter.get_point(x.into(), (self.frame.height/3 + 2).into()).unwrap();
                painter.paint(wave_x, wave_y, self.color);

                let (wave_x_2_start, wave_y_2_start) = painter.get_point(if x <= 1 {0.into()} else {(x-2).into()}, (self.frame.height/3 + 1).into()).unwrap();
                let (wave_x_2_end, _) = painter.get_point(if x >= self.frame.width - 1 {self.frame.width.into()} else {(x+2).into()}, (self.frame.height/3 + 1).into()).unwrap();

                for wx in wave_x_2_start..wave_x_2_end {
                    painter.paint(wx, wave_y_2_start, self.color);
                }
            }

            for y in self.bottom()..self.top() {
                let (x, y) = painter.get_point(x.into(), y.into()).unwrap();
                painter.paint(x, y, self.color);
            }
        }
    }
}
