/* Draw horizontal and vertical lines randomly*/
use nannou::prelude::*;
use std::{thread, time};

const WEIGHT: f32 = 4.0;
const NBR_OBJECTS: usize = 100;
//const COUNTER_TRESHOLD: u8 = 50;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    // list containing at n a starting point and at n+1 an ending point
    // to draw a line
    pub lines: [(Point2, Point2, f32); NBR_OBJECTS],
}

fn model(_app: &App) -> Model {
    Model::new()
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for i in 0..NBR_OBJECTS / 2 {
        if f32::max(model.lines[i].0.y, model.lines[i].1.y) >= 500.0
            || f32::min(model.lines[i].0.y, model.lines[i].1.y) <= -500.0
        {
            model.lines[i].2 = -model.lines[i].2;
        }
    }

    for i in NBR_OBJECTS / 2..NBR_OBJECTS {
        if f32::max(model.lines[i].0.x, model.lines[i].1.x) >= 500.0
            || f32::min(model.lines[i].1.x, model.lines[i].0.x) <= -500.0
        {
            model.lines[i].2 = -model.lines[i].2;
        }
    }
    model.move_lines();
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
        let start_x_coord = random_range(-299.0, 299.0);
        let start_y_coord = random_range(-299.0, 299.0);
        let line_length = match random::<u8>() % 2 {
            0 => random_range(30.0, 250.0),
            1 => random_range(-250.0, -30.0),
            _ => 50.0,
        };

        let start_point = pt2(start_x_coord, start_y_coord);
        let end_point: Vec2 = if i < NBR_OBJECTS / 2 {
            pt2(start_x_coord, start_y_coord + line_length % 300.0) // create vertical line
        } else {
            pt2(start_x_coord + line_length % 300.0, start_y_coord) // create horizontal line
        };
        // TODO:check length of the line and put a low threshold
        *line = (start_point, end_point, random_range(-4.0, 4.0));
    }

    lines
}

impl Model {
    fn new() -> Model {
        Model {
            lines: generate_lines(),
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

    fn move_lines(&mut self) {
        self.move_lines_vertically();
        self.move_lines_horizontally();
    }
}
