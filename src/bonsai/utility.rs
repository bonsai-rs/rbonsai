use rand::{rngs::StdRng, Rng};

use super::BranchType;

pub enum FontStyle {
    Reset,
    Bold,
}

pub struct Style {
    pub attribute: FontStyle,
    pub foreground_color: u8,
    pub background_color: u8,
}

pub(crate) fn set_deltas(
    branch_type: &BranchType,
    life: i32,
    age: i32,
    multiplier: i32,
    rng: &mut StdRng,
) -> (i32, i32) {
    let (dx, dy): (i32, i32);

    match branch_type {
        BranchType::Trunk => {
            if age <= 2 || life < 4 {
                dy = 0;
                dx = rng.gen_range(-1..=1);
            } else if age < (multiplier * 3) {
                if age % (multiplier / 2) == 0 {
                    dy = -1;
                } else {
                    dy = 0;
                }

                dx = match rng.gen_range(0..10) {
                    0 => -2,
                    1..=3 => -1,
                    4..=5 => 0,
                    6..=8 => 1,
                    _ => 2,
                };
            } else {
                if rng.gen_range(0..10) > 2 {
                    dy = -1;
                } else {
                    dy = 0;
                }
                dx = rng.gen_range(-1..=1);
            }
        }
        BranchType::ShootLeft => {
            dy = match rng.gen_range(0..10) {
                0..=1 => -1,
                2..=7 => 0,
                _ => 1,
            };
            dx = match rng.gen_range(0..10) {
                0..=1 => -2,
                2..=5 => -1,
                6..=8 => 0,
                _ => 1,
            };
        }
        BranchType::ShootRight => {
            dy = match rng.gen_range(0..10) {
                0..=1 => -1,
                2..=7 => 0,
                _ => 1,
            };
            dx = match rng.gen_range(0..10) {
                0..=1 => 2,
                2..=5 => 1,
                6..=8 => 0,
                _ => -1,
            };
        }
        BranchType::Dying => {
            dy = match rng.gen_range(0..10) {
                0..=1 => -1,
                2..=8 => 0,
                _ => 1,
            };
            dx = match rng.gen_range(0..15) {
                0 => -3,
                1..=2 => -2,
                3..=5 => -1,
                6..=8 => 0,
                9..=11 => 1,
                12..=13 => 2,
                _ => 3,
            };
        }
        BranchType::Dead => {
            dy = match rng.gen_range(0..10) {
                0..=2 => -1,
                3..=6 => 0,
                _ => 1,
            };
            dx = rng.gen_range(-1..=1);
        }
    }

    (dx, dy)
}

pub(crate) fn choose_string(branch_type: &BranchType, life: i32, dx: i32, dy: i32) -> String {
    let mut branch_str = match branch_type {
        BranchType::Trunk => match (dx, dy) {
            (0, 0) => "/~".to_string(),
            _ if dx < 0 => "\\|".to_string(),
            (0, _) => "/|\\".to_string(),
            _ if dx > 0 => "|/".to_string(),
            _ => "?".to_string(), // Fallback
        },
        BranchType::ShootLeft => match (dx, dy) {
            _ if dy > 0 => "\\".to_string(),
            (0, 0) => "\\_".to_string(),
            _ if dx < 0 => "\\|".to_string(),
            (0, _) => "/|".to_string(),
            _ if dx > 0 => "/".to_string(),
            _ => "?".to_string(), // Fallback
        },
        BranchType::ShootRight => match (dx, dy) {
            _ if dy > 0 => "/".to_string(),
            (0, 0) => "_/".to_string(),
            _ if dx < 0 => "\\|".to_string(),
            (0, _) => "/|".to_string(),
            _ if dx > 0 => "/".to_string(),
            _ => "?".to_string(), // Fallback
        },
        BranchType::Dying | BranchType::Dead => {
            "&".to_string() // Fallback
                            // add the below if user input leaves become allowedc
                            // conf.leaves[rng.gen_range(0..conf.leaves.len())].clone()
        }
    };

    // If life < 4, override with dying or dead branch representation
    if life < 4 {
        branch_str = "&".to_string(); // Fallback
                                      // add the below if user input leaves become allowedc
                                      // return conf.leaves[rng.gen_range(0..conf.leaves.len())].clone();
    }

    branch_str
}

pub(crate) fn choose_color(
    branch_type: &BranchType,
    rng: &mut StdRng,
) -> Result<Style, std::io::Error> {
    let mut style = Style {
        attribute: FontStyle::Reset,
        foreground_color: 0,
        background_color: 0,
    };

    match branch_type {
        BranchType::Trunk | BranchType::ShootLeft | BranchType::ShootRight => {
            if rng.gen_range(0..2) == 0 {
                style.attribute = FontStyle::Bold;
                style.foreground_color = 11;
            } else {
                style.foreground_color = 3;
            }
        }
        BranchType::Dying => {
            if rng.gen_range(0..10) == 0 {
                style.attribute = FontStyle::Bold;
                style.foreground_color = 2;
            } else {
                style.foreground_color = 2;
            }
        }
        BranchType::Dead => {
            if rng.gen_range(0..3) == 0 {
                style.attribute = FontStyle::Bold;
                style.foreground_color = 10;
            } else {
                style.foreground_color = 10;
            }
        }
    }

    Ok(style)
}
