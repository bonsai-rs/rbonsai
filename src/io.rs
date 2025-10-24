use std::{
    error::Error,
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyEventKind},
    execute, queue,
    style::{Color, Print, SetAttribute, SetBackgroundColor, SetForegroundColor},
    terminal::{self, size, Clear},
};
use rbonsai::{base::draw_base, bonsai::Val, Config};

pub fn create_message_window(message: &str) -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();

    // Get terminal size
    let (max_x, max_y) = size()?;

    // Calculate box dimensions based on message length
    let message_length = message.chars().count() as u16;
    let (box_width, box_height) = if message_length + 3 <= (0.25 * max_x as f32) as u16 {
        (message_length + 1, 0)
    } else {
        let width = (0.25 * max_x as f32) as u16;
        let height = message_length / width;
        (width, height)
    };

    // Calculate position based on terminal size
    let num_lines = box_height + 2;
    let num_cols = box_width + 4;
    let border_x_start = (max_x as f32 * 0.7) as u16 - 2;
    let border_y_start = (max_y as f32 * 0.7) as u16 - 1;
    let message_x_start = (max_x as f32 * 0.7) as u16;
    let message_y_start = (max_y as f32 * 0.7) as u16;

    // Draw the box border
    queue!(
        stdout,
        MoveTo(border_x_start, border_y_start),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Reset),
        Print("+"),
        MoveTo(border_x_start + num_cols, border_y_start),
        Print("+"),
        MoveTo(border_x_start, border_y_start + num_lines),
        Print("+"),
        MoveTo(border_x_start + num_cols, border_y_start + num_lines),
        Print("+"),
    )?;
    for i in 1..num_cols {
        queue!(
            stdout,
            MoveTo(border_x_start + i, border_y_start),
            Print("-"),
            MoveTo(border_x_start + i, border_y_start + num_lines),
            Print("-"),
        )?;
    }
    for i in 1..num_lines {
        queue!(
            stdout,
            MoveTo(border_x_start, border_y_start + i),
            Print("|"),
            MoveTo(border_x_start + num_cols, border_y_start + i),
            Print("|"),
        )?;
    }

    // Print the message inside the box
    let lines: Vec<&str> = message.split_whitespace().collect();
    let mut current_line = String::new();

    let mut line_count = 0;
    for word in lines.iter() {
        if current_line.len() + word.len() > box_width as usize {
            queue!(
                stdout,
                MoveTo(message_x_start + 1, message_y_start + line_count as u16),
                Print(&current_line)
            )?;
            current_line.clear();
            line_count += 1;
        }
        current_line.push_str(word);
        current_line.push(' ');
    }
    queue!(
        stdout,
        MoveTo(message_x_start + 1, message_y_start + line_count as u16),
        Print(&current_line)
    )?;

    stdout.flush()?;

    Ok(())
}

pub fn check_key_press() -> bool {
    match event::poll(Duration::from_millis(0)) {
        // if there was an error getting the event, we exit
        // program so return true
        Err(_) => true,
        // if there wasn't an error getting the event
        Ok(has_event) => {
            // if no event, key not pressed
            if !has_event {
                return false;
            }
            match event::read() {
                // if there was an error reading the event, exit program
                Err(_) => true,
                Ok(event) => {
                    // otherwise, exit if event is key press
                    if let Event::Key(key_event) = event {
                        if let KeyEventKind::Press = key_event.kind {
                            return true;
                        }
                    }
                    false
                }
            }
        }
    }
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

pub fn draw_base(config: &Config) {
    let base_type = config.base;
    let mut stdout = stdout();
    let (cols, rows) = terminal::size().unwrap(); // Get terminal size for centering

    match base_type {
        1 => {
            let lines = [];
            let base_width = 31; // The maximum width of the base art for base type 1
            let start_pos = (cols / 2) - (base_width / 2);
            let y = rows - lines.len() as u16;
            queue!(
                stdout,
                SetAttribute(crossterm::style::Attribute::Bold),
                SetForegroundColor(Color::AnsiValue(8)),
                SetBackgroundColor(Color::Reset),
                MoveTo(start_pos, y),
                Print(":"),
                SetForegroundColor(Color::AnsiValue(2)),
                Print("___________"),
                SetForegroundColor(Color::AnsiValue(11)),
                Print("./~~~\\."),
                SetForegroundColor(Color::AnsiValue(2)),
                Print("___________"),
                SetAttribute(crossterm::style::Attribute::Bold),
                SetForegroundColor(Color::AnsiValue(8)),
                Print(":"),
                MoveTo(start_pos, y + 1),
                Print(" \\                           / "),
                MoveTo(start_pos, y + 2),
                Print("  \\_________________________/ "),
                MoveTo(start_pos, y + 3),
                Print("  (_)                     (_)"),
            )
            .unwrap();

            // flush
            stdout.flush().unwrap();
        }
        2 => {
            let base_width = 15; // The maximum width of the base art for base type 2
            let start_pos = (cols / 2) - (base_width / 2);
            let y = rows - 3; // Assuming there are always 3 lines for base type 2

            // Adjust color and attribute settings before printing each part of the base art
            queue!(
                stdout,
                SetAttribute(crossterm::style::Attribute::NormalIntensity), // Assuming you want to reset boldness here
                SetForegroundColor(Color::AnsiValue(8)),
                SetBackgroundColor(Color::Reset),
                MoveTo(start_pos, y),
                Print("("),
                SetForegroundColor(Color::AnsiValue(2)),
                Print("---"),
                SetForegroundColor(Color::AnsiValue(11)),
                Print("./~~~\\."),
                SetForegroundColor(Color::AnsiValue(2)),
                Print("---"),
                SetForegroundColor(Color::AnsiValue(8)),
                Print(")"),
                MoveTo(start_pos, y + 1),
                Print(" (           ) "),
                MoveTo(start_pos, y + 2),
                Print("  (_________)  "),
            )
            .unwrap();

            // flush stdout to apply all queued actions
            stdout.flush().unwrap();
        }
        _ => {}
    }
}
