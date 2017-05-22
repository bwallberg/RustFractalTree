extern crate piston_window;

use piston_window::*;

#[derive(Debug)]
struct Branch {
    start_x: f64,
    start_y: f64,
    angle: f64,
    length: f64,
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
        length: 150.0,
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
                            branch.start_x + (50.0 * branch.angle.to_radians().cos()),
                            branch.start_y + (50.0 * branch.angle.to_radians().sin())
                        ],
                        &c.draw_state, c.transform, g
                    );
                }
            });
        }

        if let Some(u) = e.update_args() {
            if depth > 0.0 {
                let mut new_branches: Vec<Branch> = Vec::new();
                for branch in branches.iter_mut() {
                    if branch.split == false {
                        branch.split = true;

                        new_branches.push(Branch {
                            start_x: branch.start_x + (50.0 * branch.angle.to_radians().cos()),
                            start_y: branch.start_y + (50.0 * branch.angle.to_radians().sin()),
                            angle: branch.angle - 20.0,
                            length: branch.length - 10.0,
                            split: false
                        });

                        new_branches.push(Branch {
                            start_x: branch.start_x + (50.0 * branch.angle.to_radians().cos()),
                            start_y: branch.start_y + (50.0 * branch.angle.to_radians().sin()),
                            angle: branch.angle + 20.0,
                            length: branch.length - 10.0,
                            split: false
                        });
                    }
                }

                branches.append(&mut new_branches);
                depth -= 1.0;
            }
        }
    }
}