extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::Rng;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const ANGLE_MOD: f64 = 20.0;

#[derive(Debug)]
struct Branch {
    start_x: f64,
    start_y: f64,
    angle: f64,
    length: f64,
    cur_length: f64,
    split: bool
}

impl Branch {
    fn new(start_x: f64, start_y: f64, angle: f64, length: f64) -> Branch {
        Branch {
            start_x: start_x,
            start_y: start_y,
            angle: angle,
            length: length,

            cur_length: 0.0,
            split: false
        }
    }

    fn get_end_x(&self) -> f64 {
        self.start_x + self.cur_length * self.angle.to_radians().cos()
    }

    fn get_end_y(&self) -> f64 {
        self.start_y + self.cur_length * self.angle.to_radians().sin()
    }
}

fn get_random_length(start_length: f64) -> f64 {
    start_length - rand::thread_rng().gen_range(5, 20) as f64
}

fn get_random_angle(start_angle: f64, direction: f64) -> f64 {
    start_angle + direction * (ANGLE_MOD + (rand::thread_rng().gen_range(0, 10) as f64))
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("RustFractalTree", [WIDTH, HEIGHT]).exit_on_esc(true).build().unwrap();
    let mut branches: Vec<Branch> = Vec::new();
    let mut zero_reached = false;

    let mut alpha = 1.0;

    branches.push(Branch::new(
        WIDTH as f64 / 2.0,
        HEIGHT as f64,
        -90.0,
        100.0
    ));

    while let Some(e) = window.next() {
        
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Space => {
                    branches.clear();
                    zero_reached = false;
                    branches.push(Branch::new(
                        WIDTH as f64 / 2.0,
                        HEIGHT as f64,
                        -90.0,
                        100.0
                    ));
                },
                _ => println!("Pressed keyboard key {:?}", key)
            }
        }

        if let Some(_r) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                if (branches.len() == 1) {
                    clear([0.0, 0.0, 0.0, 1.0], g);
                }
                for branch in &branches {
                    Line::new(
                        [255.0, 255.0, 255.0, alpha], // color
                        1.0 // radius
                    ).draw(
                        [
                            branch.start_x,
                            branch.start_y,
                            branch.get_end_x(),
                            branch.get_end_y()
                        ],
                        &c.draw_state, c.transform, g
                    );
                }
            });
        }

        if let Some(update_args) = e.update_args() {
            if zero_reached == false {
                let mut new_branches: Vec<Branch> = Vec::new();
                let mut nothing_updated = true;
                for branch in branches.iter_mut() {
                    if branch.split == false && branch.cur_length >= branch.length {
                        nothing_updated = false;
                        branch.split = true;

                        let branch_one = Branch::new(
                            branch.get_end_x(),
                            branch.get_end_y(),
                            get_random_angle(branch.angle, 1.0),
                            get_random_length(branch.length)
                        );

                        let branch_two = Branch::new(
                            branch.get_end_x(),
                            branch.get_end_y(),
                            get_random_angle(branch.angle, -1.0),
                            get_random_length(branch.length)
                        );

                        if branch_one.length > 0.0 {
                            new_branches.push(branch_one);
                        }
                        if branch_two.length > 0.0 {
                            new_branches.push(branch_two);
                        }
                    } else if branch.cur_length < branch.length {
                        nothing_updated = false;
                        branch.cur_length += 100.0 * update_args.dt;
                    }
                }
                zero_reached = nothing_updated;
                if new_branches.len() > 0 {
                    if(alpha > 0.1) {
                        alpha -= 0.01;
                    }
                    branches.append(&mut new_branches);
                }
            }
        }
    }
}