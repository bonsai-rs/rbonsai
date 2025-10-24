pub mod utility;
use rand::{rngs::StdRng, Rng};
use utility::{choose_color, choose_string, set_deltas};

use self::utility::Style;

/// The different types of each symbol
pub(crate) enum BranchType {
    /// The main trunk of the tree
    Trunk,
    /// A branch that grows to the left
    ShootLeft,
    /// A branch that grows to the right
    ShootRight,
    /// A branch that is dying
    Dead,
    /// A branch that is dead
    Dying,
}

struct Counters {
    shoots: i32,
    branches: i32,
    shoot_counter: i32,
    tree_bottom: u16,
}

pub struct Val {
    pub style: Style,
    pub char: String,
    pub pos: Position,
    pub dx: i32,
    pub dy: i32,
    pub life: i32,
    pub branch_type: String,
    pub shoots: i32,
    pub shoot_cooldown: i32,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct TreeConfig {
    pub max_x: u16,
    pub max_y: u16,
    pub life: i32,
    pub multiplier: i32,
}

pub fn grow_tree(config: &TreeConfig, rng: &mut StdRng) -> Vec<Val> {
    // Reset counters
    let mut counters = Counters {
        shoots: 0,
        branches: 0,
        // Initialize shoot counter to a random value
        shoot_counter: (rng.gen::<i32>() % 3) + 1,
        tree_bottom: config.max_y,
    };

    let mut tree = Vec::new();

    // Recursively grow tree trunk and branches
    branch(
        config,
        &mut counters,
        &mut tree,
        rng,
        Position {
            x: (config.max_x / 2) as i32,
            y: config.max_y as i32,
        },
        BranchType::Trunk,
        config.life,
    );

    tree
}

fn branch(
    config: &TreeConfig,
    counters: &mut Counters,
    tree: &mut Vec<Val>,
    rng: &mut StdRng,
    mut pos: Position,
    branch_type: BranchType,
    mut life: i32,
) {
    counters.branches += 1;
    let mut shoot_cooldown = config.multiplier;

    // This is a highly simplified loop to mimic the growth logic
    while life > 0 {
        // Decrement life
        life -= 1;
        let age = config.life - life;

        let (dx, mut dy) = set_deltas(&branch_type, life, age, config.multiplier, rng);

        if dy > 0 && pos.y > (counters.tree_bottom as i32 - 1) {
            dy -= 1;
        } // reduce dy if too close to the ground
          // Ensure x and y are within terminal bounds
        if pos.x < 0 || pos.x as u16 >= config.max_x || pos.y < 0 || pos.y as u16 >= config.max_y {
            break;
        }

        if life < 3 {
            branch(config, counters, tree, rng, pos, BranchType::Dead, life);
        } else {
            match branch_type {
                BranchType::Trunk | BranchType::ShootLeft | BranchType::ShootRight
                    if life < (config.multiplier + 2) =>
                {
                    branch(config, counters, tree, rng, pos, BranchType::Dying, life);
                }
                BranchType::Trunk
                    if (rng.gen_range(0..3) == 0 || life % config.multiplier == 0) =>
                {
                    if rng.gen_range(0..8) == 0 && life > 7 {
                        shoot_cooldown = config.multiplier * 2;
                        let random_life = life + rng.gen_range(-2..3);
                        branch(
                            config,
                            counters,
                            tree,
                            rng,
                            pos,
                            BranchType::Trunk,
                            random_life,
                        );
                    } else if shoot_cooldown <= 0 {
                        shoot_cooldown = config.multiplier * 2;
                        let shoot_life = life + config.multiplier;
                        counters.shoots += 1;
                        counters.shoot_counter += 1;
                        let shoot_direction = if counters.shoot_counter % 2 == 0 {
                            BranchType::ShootLeft
                        } else {
                            BranchType::ShootRight
                        };
                        branch(
                            config,
                            counters,
                            tree,
                            rng,
                            pos,
                            shoot_direction,
                            shoot_life,
                        );
                    }
                }
                _ => {}
            }
        }

        shoot_cooldown -= 1;

        // Update x and y for the next iteration
        pos.x += dx;
        pos.y += dy;

        if pos.x < 0 || pos.x as u16 >= config.max_x || pos.y < 0 || pos.y as u16 >= config.max_y {
            continue;
        }

        // Drawing the branch part
        let branch_str = choose_string(&branch_type, life, dx, dy);
        // Example to set color, adjust as needed
        let style = choose_color(&branch_type, rng).unwrap();
        let type_str = match branch_type {
            BranchType::Trunk => "Trunk",
            BranchType::ShootLeft => "ShootLeft",
            BranchType::ShootRight => "ShootRight",
            BranchType::Dying => "Dying",
            BranchType::Dead => "Dead",
        };
        let val = Val {
            pos,
            style,
            char: branch_str,
            branch_type: type_str.into(),
            dx,
            dy,
            life,
            shoots: counters.shoots,
            shoot_cooldown,
        };
        tree.push(val);
    }
}
