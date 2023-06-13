mod structs;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders},
    Terminal,
};
use std::{env, io, time::Duration};

use structs::Timer;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args.len() > 3 {
        println!("Wrong usage of the program. Correct usage: \narg1: description of the timer\narg2: time (in minutes)");
        println!("You provided {} args", args.len());
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Wrong usage of parameters",
        ));
    }

    let desc = args.get(1).unwrap().as_str();
    let time = args.get(2).unwrap().parse::<u64>().unwrap();

    let mut timer = Timer::new(desc, time);

    draw_timer(&mut timer)
}

fn draw_timer(timer: &mut Timer) -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut last_timer_w = 0;
    while !timer.is_done() {
        terminal.draw(|f| {
            let outer_area = f.size();
            let outer_block = Block::default()
                .borders(Borders::NONE)
                .title(timer.to_string());
            let inner_area = outer_block.inner(f.size());

            let timer_area;
            let timer_width;

            if !timer.is_paused() {
                timer_width = (inner_area.width as f32 * timer.get_percent()).round() as u16;
                timer_area = Rect::new(inner_area.x, inner_area.y, timer_width, inner_area.height);

                last_timer_w = timer_width;
            } else {
                timer_area = Rect::new(inner_area.x, inner_area.y, last_timer_w, inner_area.height);
            }

            let timer_style;
            if timer.is_paused() {
                timer_style = Style::default().bg(Color::Gray);
            } else {
                timer_style = Style::default().bg(Color::Cyan);
            }

            let timer_block = Block::default().style(timer_style);

            f.render_widget(outer_block, outer_area);
            f.render_widget(timer_block, timer_area);
        })?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char(' ') => {
                        if timer.is_paused() {
                            timer.unpause_timer();
                        } else {
                            timer.pause_timer();
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    Ok(())
}
