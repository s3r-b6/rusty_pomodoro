mod structs;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders},
    Terminal,
};
use std::{
    env,
    io::{self, Stdout},
    time::Duration,
};

use structs::Timer;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args.len() > 3 {
        eprintln!(
            "Wrong usage of the program. Correct usage:\n -arg1: description of the timer\n -arg2: time (in minutes)\nYou provided {} args",
            args.len() - 1
        );
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Wrong usage of parameters",
        ));
    }

    let desc = args.get(1).unwrap().as_str();
    let time = args.get(2).unwrap().parse::<u64>().unwrap_or_else(|_| {
        eprintln!("Wrong time parameter, unable to parse");
        panic!("Unable to parse time parameter")
    });

    if time > 720 {
        //A timer bigger than 12hours seems not really useful
        eprintln!("Wrong time parameter, maximum of minutes is 720 (12h)");
        panic!("Unable to parse time parameter")
    }

    let mut timer = Timer::new(desc, time);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    draw_app(&mut terminal, &mut timer)?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    disable_raw_mode()?;

    Ok(())
}

fn draw_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    timer: &mut Timer,
) -> Result<(), io::Error> {
    let mut last_timer_w = 0;

    while !timer.is_done() {
        terminal.draw(|f| draw_frame(f, timer, &mut last_timer_w))?;

        //Check for keys pressed
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

    Ok(())
}

fn draw_frame(
    f: &mut ratatui::Frame<CrosstermBackend<Stdout>>,
    timer: &mut Timer,
    last_timer_w: &mut u16,
) {
    let outer_area = f.size();
    let outer_block = Block::default()
        .borders(Borders::NONE)
        .title(timer.to_string());
    let inner_area = outer_block.inner(f.size());

    let timer_area;
    let timer_width;

    if !timer.is_paused() {
        timer_width = (inner_area.width as f64 * timer.get_percent()) as u16;
        timer_area = Rect::new(inner_area.x, inner_area.y, timer_width, inner_area.height);

        *last_timer_w = timer_width;
    } else {
        timer_area = Rect::new(inner_area.x, inner_area.y, *last_timer_w, inner_area.height);
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
}
