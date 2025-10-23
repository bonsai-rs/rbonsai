use std::{
    io::stdout,
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use clap::Parser;
use crossterm::{
    cursor::{self, MoveTo},
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::{rngs::StdRng, SeedableRng};
use rbonsai::{
    bonsai::{
        draw_tree, grow_tree, init,
        utility::{check_key_press, create_message_window},
    },
    Config,
};

fn main() {
    let mut args = Config::parse();

    if args.screensaver {
        args.live = true;
        args.infinite = true;
    }

    let mut stdout = stdout();

    let seed = args.seed.unwrap_or_else(|| {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_secs()
    });

    let mut rng = StdRng::seed_from_u64(seed);

    enable_raw_mode().unwrap();
    execute!(stdout, cursor::Hide).unwrap();
    execute!(stdout, EnterAlternateScreen).unwrap();

    // Flush any pending events
    while event::poll(Duration::from_millis(10)).unwrap() {
        let _ = event::read();
    }

    let mut should_exit: bool;

    let last_tree = loop {
        init(&args);
        let tree = grow_tree(&args, &mut rng);

        // if the user exited before the tree is finished being drawn, should
        // exit program
        should_exit = !draw_tree(&args, &tree);

        if should_exit {
            break tree;
        }

        if let Some(message) = &args.message {
            create_message_window(message).unwrap();
        }

        if !args.infinite {
            break tree;
        }
        let start = Instant::now();
        let mut finished = false;
        while start.elapsed() < Duration::from_secs_f64(args.wait) {
            if check_key_press() {
                finished = true;
                break;
            }
            thread::sleep(Duration::from_millis(50)); // Sleep to avoid busy-waiting
        }

        if finished {
            should_exit = true;
            break tree;
        }
    };

    let (_, rows) = crossterm::terminal::size().unwrap();
    if should_exit {
        execute!(stdout, LeaveAlternateScreen).unwrap();
    } else if args.print {
        args.live = false;
        execute!(stdout, LeaveAlternateScreen).unwrap();
        for _ in 0..rows {
            println!();
        }

        init(&args);
        draw_tree(&args, &last_tree);
        if let Some(message) = &args.message {
            create_message_window(message).unwrap();
        }
        execute!(stdout, MoveTo(0, rows - 1),).unwrap();
        println!();
    } else {
        while let Ok(event) = event::read() {
            // break if key event
            if let Event::Key(key_event) = event {
                if let KeyEventKind::Press = key_event.kind {
                    break;
                }
            }
        }
        execute!(stdout, LeaveAlternateScreen).unwrap();
    }

    // move cursor to bottom of terminal
    execute!(stdout, MoveTo(0, rows - 1),).unwrap();
    let _ = disable_raw_mode();
    execute!(stdout, cursor::Show).unwrap();
}

pub fn init_terminal(args: &Config) {
    let mut stdout = stdout();
    execute!(stdout, Clear(terminal::ClearType::All)).unwrap();
    draw_base(args);
}

// returns true if the tree finished drawing. Returns false if it didn't and
// the user chose to exit early
pub fn draw_tree(config: &Config, tree: &Vec<Val>) -> bool {
    let mut stdout = stdout();
    for val in tree {
        if config.verbose {
            // Queueing the commands instead of executing them immediately
            // This allows for batching the writes, which can be more efficient
            stdout
                .queue(MoveTo(5, 3))
                .unwrap()
                .queue(Print(format!("life: {}", val.life)))
                .unwrap()
                .queue(MoveTo(5, 4))
                .unwrap()
                .queue(Print(format!("shoots: {:02}", val.shoots)))
                .unwrap()
                .queue(MoveTo(5, 5))
                .unwrap()
                .queue(Print(format!("dx: {:02}", val.dx)))
                .unwrap()
                .queue(MoveTo(5, 6))
                .unwrap()
                .queue(Print(format!("dy: {:02}", val.dy)))
                .unwrap()
                .queue(MoveTo(5, 7))
                .unwrap()
                .queue(Print(format!("type: {}", val.branch_type)))
                .unwrap()
                .queue(MoveTo(5, 8))
                .unwrap()
                .queue(Print(format!("shootCooldown: {:3}", val.shoot_cooldown)))
                .unwrap();

            // Flush the stdout to apply the queued operations
            stdout.flush().unwrap();
        }

        let _ = execute!(
            stdout,
            SetAttribute(val.style.attribute),
            SetForegroundColor(val.style.foreground_color),
            SetBackgroundColor(val.style.background_color),
        );
        let _ = execute!(
            stdout,
            MoveTo(val.pos.x as u16, val.pos.y as u16),
            Print(val.char.clone()),
        );
        // reset color
        let _ = execute!(stdout, SetColors(Colors::new(Color::Reset, Color::Reset)),);
        if config.live {
            let start = Instant::now();
            let mut finished = false;
            while start.elapsed() < Duration::from_secs_f64(config.time) {
                if check_key_press() {
                    finished = true;
                    break;
                }
                thread::sleep(Duration::from_millis(50)); // Sleep to avoid busy-waiting
            }

            if finished {
                return false;
            }
        }
    }

    true
}
