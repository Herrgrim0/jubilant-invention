/* Draw horizontal and vertical lines randomly
*/
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    let start_x_coord = random_range(-300.0, 300.0);
    let start_y_coord = random_range(-200.0, 200.0);
    let line_length = random_range(-100.0, 100.0);

    let start_point = pt2(start_x_coord, start_y_coord);
    let mut end_point: Vec2 = pt2(0.0, 0.0);

    let horizontal_or_vertical = random::<usize>() % 2;
    match horizontal_or_vertical {
        0 => end_point = pt2(start_x_coord, start_y_coord + line_length),
        1 => end_point = pt2(start_x_coord + line_length, start_y_coord),
        _ => {}
    }

    draw.line()
        .start(start_point)
        .end(end_point)
        .weight(4.0)
        .color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}
