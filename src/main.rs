/* Draw horizontal and vertical lines randomly*/
use clap::Parser;
use nannou::prelude::*;
use std::{thread, time};

mod module;

#[derive(Parser)]
#[command(name = "jubilant-invention")]
#[command(about = "Draw horizontal and vertical lines randomly")]
#[command(version = "0.1")]
struct Cli {
    /// Feature to use
    feature: module::Feature,

    #[arg(short, long, default_value = "200")]
    /// Number of objects to draw
    objects: Option<usize>,

    #[arg(short, long, default_value = "4.0")]
    /// Thickness of the lines
    line_weight: Option<f32>,
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

fn model(_app: &App) -> module::Model {
    let cli = Cli::parse();
    let mut model = module::ModelBuilder::new()
        .feature(cli.feature)
        .nbr_objects(cli.objects)
        .line_weight(cli.line_weight)
        .build();

    model.generate_lines();
    model
}

fn update(app: &App, model: &mut module::Model, _update: Update) {
    match model.get_feature() {
        module::Feature::Move => {
            model.update_movement(app);
        }
        module::Feature::Extend => {
            model.update_extension();
        }
    }

    thread::sleep(time::Duration::from_millis(50));
}

fn view(app: &App, model: &module::Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    // draw the list of lines
    for i in 0..model.get_nbr_objects() {
        draw.line()
            .start(model.lines[i].0)
            .end(model.lines[i].1)
            .weight(model.get_line_weight())
            .color(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();
}
