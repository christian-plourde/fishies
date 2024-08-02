use ratatui::layout::Rect;
use std::time::Duration;
use std::time::Instant;
use std::io;
use std::io::stdout;
use std::io::Stdout;
use ratatui::crossterm::event;
use ratatui::crossterm::ExecutableCommand;
use ratatui::crossterm::event::Event;
use ratatui::crossterm::event::KeyCode;
use ratatui::crossterm::event::KeyModifiers;
use ratatui::crossterm::terminal::enable_raw_mode;
use ratatui::crossterm::terminal::disable_raw_mode;
use ratatui::crossterm::terminal::EnterAlternateScreen;
use ratatui::crossterm::terminal::LeaveAlternateScreen;
use ratatui::backend::CrosstermBackend;
use ratatui::terminal::Terminal;
use ratatui::terminal::Frame;
use ratatui::layout::Layout;
use ratatui::layout::Constraint;
use ratatui::style::Color;
use ratatui::widgets::canvas::Canvas;
use ratatui::symbols::Marker;
use ratatui::widgets::Widget;
use crate::shapes::water::Water;
use crate::shapes::dock::Dock;
use crate::shapes::fish::Fish;
use crate::shapes::fisherman::Fisherman;
use rand::Rng;

pub struct Fishies {
    frame: Rect,
    fish: Vec<Fish>,
    maximum_fish_population: usize,
    dock: Dock,
    water: Water,
    fisherman: Fisherman,
    fish_colors: [Color; 7],
}

impl Fishies {
    fn new() -> Self {
        let frame = Rect::new(0, 0, 256, 128); 
        Self {
            frame,
            fish_colors: [
                Color::Red,
                Color::Green,
                Color::Yellow,
                Color::Magenta,
                Color::Cyan,
                Color::LightRed,
                Color::LightGreen,
            ],
            fish: vec![],
            maximum_fish_population: 8,
            dock: Dock {frame, color: Color::Rgb(210, 180, 140)},
            water: Water {frame, color: Color::LightBlue},
            fisherman: Fisherman {frame, shoe_color: Color::DarkGray, pants_color: Color::Rgb(92, 64, 51)},
        }
    }

    pub fn run() -> io::Result<()> {
        let mut terminal = init_terminal()?;
        let mut app = Self::new();
        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(400);

        loop {
            let _ = terminal.draw(|frame| app.ui(frame));
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                        break;
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                app.on_tick();
                last_tick = Instant::now();
            }
        }
        restore_terminal()
    }

    fn on_tick(&mut self) {

    }

    fn ui(&mut self, frame: &mut Frame) {
        let horizontal =
            Layout::horizontal([Constraint::Percentage(100), Constraint::Percentage(0)]);
        let [map, _] = horizontal.areas(frame.size());

        frame.render_widget(self.fishies(), map);
    }

    fn update_fish(&mut self) {
        let fish_should_spawn = if self.fish.len() >= self.maximum_fish_population {false} else {rand::thread_rng().gen_range(0.0..1.0) > 0.7};
        if fish_should_spawn {
            let mouth_color = self.fish_colors[rand::thread_rng().gen_range(0..self.fish_colors.len())];
            let mut body_color = self.fish_colors[rand::thread_rng().gen_range(0..self.fish_colors.len())];

            while body_color == mouth_color {
                body_color = self.fish_colors[rand::thread_rng().gen_range(0..self.fish_colors.len())];
            }

            self.fish.push(Fish::new(rand::thread_rng().gen_range(self.water.left()..self.water.right()), rand::thread_rng().gen_range(self.water.bottom()..self.water.top()), self.water.top(), body_color, mouth_color));
        }

        self.fish.retain(|f| !f.is_dead());

        self.fish.iter_mut().for_each(|f| {
            f.r#move();
        })
    }

    fn fishies(&mut self) -> impl Widget + '_ {
        self.update_fish();

        Canvas::default()
            .marker(Marker::HalfBlock)
            .paint(|ctx| {
                ctx.draw(&self.dock);
                ctx.layer();
                ctx.draw(&self.fisherman);
                ctx.layer();
                ctx.draw(&self.water);
                for f in self.fish.iter() {
                    ctx.draw(f);
                }
            })
            .x_bounds([self.frame.x.into(), (self.frame.x + self.frame.width).into()])
            .y_bounds([self.frame.y.into(), (self.frame.y + self.frame.height).into()])
    }
}

fn init_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
