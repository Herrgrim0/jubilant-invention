/* Draw horizontal and vertical lines randomly*/
use nannou::prelude::*;
use std::{env, thread, time};

const WEIGHT: f32 = 4.0;
const NBR_OBJECTS: usize = 200;
const MAX_STEP: usize = 50;

enum Feature {
    Move,
    Extend,
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    // list containing at n a starting point and at n+1 an ending point
    // to draw a line
    feat: Feature,
    pub lines: [(Point2, Point2, f32); NBR_OBJECTS],
    step: usize,
}

fn model(_app: &App) -> Model {
    let args: Vec<String> = env::args().collect();
    let feat = match args[1].as_str() {
        "move" => Feature::Move,
        "extend" => Feature::Extend,
        _ => panic!(
            "argument didn't match!\nUse move for moving lines.\nUse extend for extending lines."
        ),
    };
    Model::new(feat)
}

fn update(app: &App, model: &mut Model, _update: Update) {
    match model.feat {
        Feature::Move => {
            for line in model.lines.iter_mut() {
                update_direction(app, line);
            }
            model.move_lines();
        }
        Feature::Extend => {
            model.extend_line();
            model.step += 1;
            if model.step > MAX_STEP {
                model.step = 0;
                for line in model.lines.iter_mut() {
                    line.2 = -line.2;
                }
            }
        }
    }

    thread::sleep(time::Duration::from_millis(50));
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    // draw the list of lines
    for i in 0..NBR_OBJECTS {
        draw.line()
            .start(model.lines[i].0)
            .end(model.lines[i].1)
            .weight(WEIGHT)
            .color(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();
}

/* Create and populate an array with a tuple representing
 * starting point and ending point of lines
 * parameters:
 * direction: enum representing either horizontal or
 * vertical direction.
 */
fn generate_lines() -> [(Point2, Point2, f32); NBR_OBJECTS] {
    let mut lines: [(Vec2, Vec2, f32); NBR_OBJECTS] =
        [(pt2(0.0, 0.0), pt2(0.0, 0.0), 0.0); NBR_OBJECTS];

    for (i, line) in lines.iter_mut().enumerate() {
        let start_x_coord = random_range(-499.0, 499.0);
        let start_y_coord = random_range(-399.0, 399.0);
        let line_length = 200.0;

        let start_point = pt2(start_x_coord, start_y_coord);
        let end_point: Vec2 = if i < NBR_OBJECTS / 2 {
            pt2(start_x_coord, start_y_coord + line_length % 300.0) // create vertical line
        } else {
            pt2(start_x_coord + line_length % 300.0, start_y_coord) // create horizontal line
        };

        if start_point.cmpge(end_point) == BVec2::new(true, true) {
            *line = (start_point, end_point, random_range(-4.0, 4.0));
        }

        *line = (end_point, start_point, random_range(-4.0, 4.0));
    }

    lines
}

fn update_direction(app: &App, line: &mut (Vec2, Vec2, f32)) {
    if line.0.x >= app.window_rect().right() || line.0.y >= app.window_rect().top() {
        line.2 = -line.2.abs();
    } else if line.1.x <= app.window_rect().left() || line.1.y <= app.window_rect().bottom() {
        line.2 = line.2.abs();
    }
}

impl Model {
    fn new(feat: Feature) -> Model {
        Model {
            feat,
            lines: generate_lines(),
            step: 0,
        }
    }

    fn move_lines_vertically(&mut self) {
        for i in 0..NBR_OBJECTS / 2 {
            self.lines[i].0.y += self.lines[i].2;
            self.lines[i].1.y += self.lines[i].2;
        }
    }

    fn move_lines_horizontally(&mut self) {
        for i in NBR_OBJECTS / 2..NBR_OBJECTS {
            self.lines[i].0.x += self.lines[i].2;
            self.lines[i].1.x += self.lines[i].2;
        }
    }

    fn extend_line(&mut self) {
        for line in self.lines.iter_mut() {
            if is_vertical(line) {
                // if the line is vertical
                if line.0.y < 0.0 {
                    line.0.y += line.2;
                } else {
                    line.0.y -= line.2;
                }
                if line.1.y < 0.0 {
                    line.1.y -= line.2;
                } else {
                    line.1.y += line.2;
                }
            } else if is_horizontal(line) {
                // line is horizontal
                if line.0.x < 0.0 {
                    line.0.x += line.2;
                } else {
                    line.0.x -= line.2;
                }
                if line.1.x < 0.0 {
                    line.1.x -= line.2;
                } else {
                    line.1.x += line.2;
                }
            }
        }
    }

    fn move_lines(&mut self) {
        self.move_lines_vertically();
        self.move_lines_horizontally();
    }
}

fn is_vertical(line: &mut (Vec2, Vec2, f32)) -> bool {
    line.0.x == line.1.x
}

fn is_horizontal(line: &mut (Vec2, Vec2, f32)) -> bool {
    line.0.y == line.1.y
}
