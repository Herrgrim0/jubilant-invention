/* Draw horizontal and vertical lines randomly*/
use nannou::prelude::*;
use std::{thread, time};

const WEIGHT: f32 = 4.0;
const NBR_OBJECTS: usize = 100;
const COUNTER_TRESHOLD: u8 = 50;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    // list containing at n a starting point and at n+1 an ending point
    // to draw a line
    pub lines: [(Point2, Point2); NBR_OBJECTS],
    pub direction: Direction,
    pub counter: u8,
    pub step: f32,
}

fn model(_app: &App) -> Model {
    Model::new()
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.counter < COUNTER_TRESHOLD {
        model.move_lines();
        model.counter += 1;
    } else {
        match model.direction {
            Direction::Up => {
                model.direction = Direction::Down;
                model.step = -2.0
            }
            Direction::Down => {
                model.direction = Direction::Left;
                model.step = 2.0
            }
            Direction::Left => {
                model.direction = Direction::Right;
                model.step = -2.0
            }
            Direction::Right => {
                model.direction = Direction::Up;
                model.step = 2.0
            }
        }
        model.counter = 0;
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
fn generate_lines() -> [(Point2, Point2); NBR_OBJECTS] {
    let mut lines: [(Vec2, Vec2); NBR_OBJECTS] = [(pt2(0.0, 0.0), pt2(0.0, 0.0)); NBR_OBJECTS];

    for (i, line) in lines.iter_mut().enumerate() {
        let start_x_coord = random_range(-300.0, 300.0);
        let start_y_coord = random_range(-300.0, 300.0);
        let line_length = random_range(-300.0, 300.0);

        let start_point = pt2(start_x_coord, start_y_coord);
        let end_point: Vec2 = if i < NBR_OBJECTS / 2 {
            pt2(start_x_coord, start_y_coord + line_length) // create vertical line
        } else {
            pt2(start_x_coord + line_length, start_y_coord) // create horizontal line
        };
        // TODO:check length of the line and put a low threshold
        *line = (start_point, end_point);
    }

    lines
}

impl Model {
    fn new() -> Model {
        Model {
            lines: generate_lines(),
            direction: Direction::Up,
            counter: 0,
            step: 2.0,
        }
    }

    fn move_lines_vertically(&mut self) {
        for i in 0..NBR_OBJECTS / 2 {
            self.lines[i].0.y += self.step;
            self.lines[i].1.y += self.step;
        }
    }

    fn move_lines_horizontally(&mut self) {
        for i in NBR_OBJECTS / 2..NBR_OBJECTS {
            self.lines[i].0.x += self.step;
            self.lines[i].1.x += self.step;
        }
    }

    fn move_lines(&mut self) {
        self.move_lines_vertically();
        self.move_lines_horizontally();
    }
}
