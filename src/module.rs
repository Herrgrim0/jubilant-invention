use nannou::prelude::*;

use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum Feature {
    /// Move the lines
    Move,
    /// Extend the lines
    Extend,
}

pub struct Model {
    // list containing at n a starting point and at n+1 an ending point
    // to draw a line
    feat: Feature,
    pub lines: Vec<(Point2, Point2, f32)>,
    step: usize,
    max_step: usize,
    nbr_objects: usize,
    line_weight: f32,
}

impl Model {
    pub fn _new(feat: Feature) -> Model {
        Model {
            feat,
            lines: vec![],
            step: 0,
            max_step: 50,
            nbr_objects: 200,
            line_weight: 4.0,
        }
    }

    pub fn get_feature(&self) -> &Feature {
        &self.feat
    }

    pub fn get_line_weight(&self) -> f32 {
        self.line_weight
    }

    pub fn get_nbr_objects(&self) -> usize {
        self.nbr_objects
    }

    /* Create and populate an array with a tuple representing
     * starting point and ending point of lines
     * parameters:
     * direction: enum representing either horizontal or
     * vertical direction.
     */
    pub fn generate_lines(&mut self) {
        self.lines = vec![(pt2(0.0, 0.0), pt2(0.0, 0.0), 0.0); self.nbr_objects];

        for (i, line) in self.lines.iter_mut().enumerate() {
            let start_x_coord = random_range(-499.0, 499.0);
            let start_y_coord = random_range(-399.0, 399.0);
            let line_length = 200.0;

            let start_point = pt2(start_x_coord, start_y_coord);
            let end_point: Vec2 = if i < self.nbr_objects / 2 {
                pt2(start_x_coord, start_y_coord + line_length % 300.0) // create vertical line
            } else {
                pt2(start_x_coord + line_length % 300.0, start_y_coord) // create horizontal line
            };

            if start_point.cmpge(end_point) == BVec2::new(true, true) {
                *line = (start_point, end_point, random_range(-4.0, 4.0));
            }

            *line = (end_point, start_point, random_range(-4.0, 4.0));
        }
    }

    pub fn update_movement(&mut self, app: &App) {
        for line in self.lines.iter_mut() {
            if line.0.x >= app.window_rect().right() || line.0.y >= app.window_rect().top() {
                line.2 = -line.2.abs();
            } else if line.1.x <= app.window_rect().left() || line.1.y <= app.window_rect().bottom()
            {
                line.2 = line.2.abs();
            }
        }
        self.move_lines();
    }

    pub fn update_extension(&mut self) {
        self.extend_line();
        self.step += 1;
        if self.step > self.max_step {
            self.step = 0;
            for line in self.lines.iter_mut() {
                line.2 = -line.2;
            }
        }
    }

    pub fn move_lines_vertically(&mut self) {
        for i in 0..self.nbr_objects / 2 {
            self.lines[i].0.y += self.lines[i].2;
            self.lines[i].1.y += self.lines[i].2;
        }
    }

    pub fn move_lines_horizontally(&mut self) {
        for i in self.nbr_objects / 2..self.nbr_objects {
            self.lines[i].0.x += self.lines[i].2;
            self.lines[i].1.x += self.lines[i].2;
        }
    }

    /* extend or shrink the lines
     * depending on the location and the direction of the line
     * e.g.: it takes into account if the line is in a negative,
     * a positive or a mixed part of the plan.
     */
    pub fn extend_line(&mut self) {
        for line in self.lines.iter_mut() {
            if is_vertical(line) {
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

    pub fn move_lines(&mut self) {
        self.move_lines_vertically();
        self.move_lines_horizontally();
    }
}

pub fn is_vertical(line: &mut (Vec2, Vec2, f32)) -> bool {
    line.0.x == line.1.x
}

pub fn is_horizontal(line: &mut (Vec2, Vec2, f32)) -> bool {
    line.0.y == line.1.y
}

pub struct ModelBuilder {
    feat: Feature,
    nbr_objects: usize,
    line_weight: f32,
}

impl ModelBuilder {
    pub fn new() -> Self {
        ModelBuilder {
            feat: Feature::Move,
            nbr_objects: 200,
            line_weight: 4.0,
        }
    }

    pub fn feature(&mut self, feat: Feature) -> &mut Self {
        self.feat = feat;
        self
    }

    pub fn nbr_objects(&mut self, nbr_objects: Option<usize>) -> &mut Self {
        match nbr_objects {
            Some(nbr_objects) => self.nbr_objects = nbr_objects,
            None => self.nbr_objects = 200,
        }
        self
    }

    pub fn line_weight(&mut self, line_weight: Option<f32>) -> &mut Self {
        match line_weight {
            Some(line_weight) => self.line_weight = line_weight,
            None => self.line_weight = 4.0,
        }
        self
    }

    pub fn build(&mut self) -> Model {
        Model {
            feat: self.feat.clone(),
            lines: vec![],
            step: 0,
            max_step: 50,
            nbr_objects: self.nbr_objects,
            line_weight: self.line_weight,
        }
    }
}
