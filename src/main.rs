extern crate piston_window;

use piston_window::*;

#[derive(Debug)]
struct Branch {
    start_x: f64,
    start_y: f64,
    angle: f64,
    length: f64,
    cur_length: f64,
    split: bool
}

fn main() {
    let width: u32 = 1280;
    let height: u32 = 720;
    let mut depth: f64 = 9.0;

    let mut window: PistonWindow = WindowSettings::new("RustFractalTree", [width,  height]).exit_on_esc(true).build().unwrap();

    let mut branches: Vec<Branch> = Vec::new();

    branches.push(Branch {
        start_x: width as f64 / 2.0,
        start_y: height as f64,
        angle: -90.0,
        length: 100.0,
        cur_length: 0.0,
        split: false
    });

    while let Some(e) = window.next() {
        if let Some(r) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                
                for branch in &branches {
                    Line::new(
                        [1.0, 0.0, 0.0, 1.0], // color
                        1.0 // radius
                    ).draw(
                        [
                            branch.start_x,
                            branch.start_y,
                            branch.start_x + (branch.cur_length * branch.angle.to_radians().cos()),
                            branch.start_y + (branch.cur_length * branch.angle.to_radians().sin())
                        ],
                        &c.draw_state, c.transform, g
                    );
                }
            });
        }

        if let Some(update_args) = e.update_args() {
            if depth > 0.0 {
                let mut new_branches: Vec<Branch> = Vec::new();
                let mut did_split = false;
                for branch in branches.iter_mut() {
                    if branch.split == false && branch.cur_length >= branch.length {
                        branch.split = true;
                        did_split = true;

                        new_branches.push(Branch {
                            start_x: branch.start_x + (branch.length * branch.angle.to_radians().cos()),
                            start_y: branch.start_y + (branch.length * branch.angle.to_radians().sin()),
                            angle: branch.angle - 20.0,
                            length: branch.length - 7.5,
                            cur_length: 0.0,
                            split: false
                        });

                        new_branches.push(Branch {
                            start_x: branch.start_x + (branch.length * branch.angle.to_radians().cos()),
                            start_y: branch.start_y + (branch.length * branch.angle.to_radians().sin()),
                            angle: branch.angle + 20.0,
                            length: branch.length - 7.5,
                            cur_length: 0.0,
                            split: false
                        });
                    } else if branch.cur_length < branch.length {
                        branch.cur_length += 100.0 * update_args.dt;
                    }
                }

                if did_split {
                    branches.append(&mut new_branches);
                    depth -= 1.0;
                }
            }
        }
    }
}