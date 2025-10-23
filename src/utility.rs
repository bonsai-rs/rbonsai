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
