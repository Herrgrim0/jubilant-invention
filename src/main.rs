/* Draw horizontal and vertical lines randomly*/
use nannou::prelude::*;
use std::time::Duration;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    std::thread::sleep(Duration::from_millis(50));
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    const WEIGHT: f32 = 4.0;
    const NBR_OBJECTS: usize = 100;
    // list containing at n a starting point and at n+1 an ending point
    // to draw a line
    let mut lst_obj: [Point2; NBR_OBJECTS * 2] = [pt2(0.0, 0.0); NBR_OBJECTS * 2];

    draw.background().color(BLACK);

    // create a list of vertical or horizontal lines with random coordinates
    let mut i: usize = 0;
    while i < NBR_OBJECTS * 2 {
        let start_x_coord = random_range(-300.0, 300.0);
        let start_y_coord = random_range(-300.0, 300.0);
        let line_length = random_range(-300.0, 300.0);

        let start_point = pt2(start_x_coord, start_y_coord);
        let mut end_point: Vec2 = pt2(0.0, 0.0);

        let horizontal_or_vertical = random::<usize>() % 2;
        match horizontal_or_vertical {
            0 => end_point = pt2(start_x_coord, start_y_coord + line_length),
            1 => end_point = pt2(start_x_coord + line_length, start_y_coord),
            _ => {}
        }

        lst_obj[i] = start_point;
        lst_obj[i + 1] = end_point;
        i += 2;
    }

    i = 0;

    // draw the list of lines
    while i < NBR_OBJECTS * 2 {
        draw.line()
            .start(lst_obj[i])
            .end(lst_obj[i + 1])
            .weight(WEIGHT)
            .color(WHITE);
        i += 2;
    }

    draw.to_frame(app, &frame).unwrap();
}
