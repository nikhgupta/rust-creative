use crate::{common::Seedable, forces::field::ForceField};
use nannou::glam::Vec2;
use nannou::math::map_range;
use nannou::prelude::pt2;
use nannou::{color::Rgba, Draw};

pub struct FlowField {
    seed: u32,
    cols: usize,
    rows: usize,
    resolution: f32,
    extend: f32,
    field: Vec<Vec<Vec2>>,

    width: f32,
    height: f32,
    _left_x: f32,
    _right_x: f32,
    _top_y: f32,
    _bottom_y: f32,
}

impl Seedable for FlowField {
    fn set_seed(&mut self, seed: u32) -> &mut Self {
        self.seed = seed;
        self
    }

    fn seed(&self) -> u32 {
        self.seed
    }
}

impl FlowField {
    pub const DEFAULT_SEED: u32 = 0;

    pub fn new(w: f32, h: f32, resolution: f32, extend: f32) -> Self {
        let cols = f32::ceil(w * (1.0 + 2.0 * extend) / resolution) as usize;
        let rows = f32::ceil(h * (1.0 + 2.0 * extend) / resolution) as usize;

        let _left_x = -w * extend;
        let _right_x = w * (1.0 + extend);
        let _top_y = -h * extend;
        let _bottom_y = h * (1.0 + extend);

        let mut field = Vec::with_capacity(cols);
        for _i in 0..cols {
            let mut row = Vec::with_capacity(rows);
            for _j in 0..rows {
                row.push(Vec2::new(0.0, 0.0));
            }
            field.push(row);
        }

        FlowField {
            seed: Self::DEFAULT_SEED,
            field,
            cols,
            rows,
            extend,
            width: w,
            height: h,
            resolution,

            _left_x,
            _right_x,
            _top_y,
            _bottom_y,
        }
    }

    pub fn zero(&mut self) -> &mut Self {
        for i in 0..self.cols {
            for j in 0..self.rows {
                self.field[i][j] = Vec2::new(0.0, 0.0);
            }
        }

        self
    }

    pub fn reset(&self, w: f32, h: f32) -> Self {
        Self::new(w, h, self.resolution, self.extend)
    }

    pub fn merge(&mut self, force: &impl ForceField) -> &mut Self {
        for i in 0..self.cols {
            for j in 0..self.rows {
                self.field[i][j] += force.get(i as f64, j as f64);
            }
        }

        self
    }

    pub fn force_at(&self, point: [f32; 2]) -> Vec2 {
        let [x, y] = self.index_at(point);
        self.field[x as usize][y as usize]
    }

    pub fn index_at(&self, point: [f32; 2]) -> [usize; 2] {
        let x = (point[0] - self._left_x) / self.resolution;
        let y = (point[1] - self._top_y) / self.resolution;

        let x = x.max(0.0).min(self._right_x - self.resolution / 2.0);
        let y = y.max(0.0).min(self._bottom_y - self.resolution / 2.0);

        [x as usize, y as usize]
    }

    pub fn display(&self, draw: &Draw, complete: bool, color: Option<Rgba>) {
        let color: Rgba = color.unwrap_or(Rgba::new(0.0, 0.0, 0.0, 0.2));
        let indices = self.displayable_bounds(complete);

        for i in indices[0][0]..indices[1][0] {
            for j in indices[0][1]..indices[1][1] {
                let mut start = pt2(
                    i as f32 * self.resolution + self._left_x,
                    j as f32 * self.resolution + self._top_y,
                );
                let mut end = start + self.field[i][j] * self.resolution;

                if complete {
                    start = self.map_to_visible(start);
                    end = self.map_to_visible(end);
                }

                draw.line().start(start).end(end).color(color);
                draw.ellipse().xy(end).radius(1.0).color(color);
            }
        }
    }

    pub fn bounds(&self) -> [[usize; 2]; 2] {
        [[0, 0], [self.cols, self.rows]]
    }

    pub fn visible_bounds(&self) -> [[usize; 2]; 2] {
        let idx1 = self.index_at([0.0, 0.0]);
        let idx2 = self.index_at([self.width, self.height]);
        [idx1, idx2]
    }

    pub fn displayable_bounds(&self, complete: bool) -> [[usize; 2]; 2] {
        if complete {
            self.bounds()
        } else {
            self.visible_bounds()
        }
    }

    fn map_to_visible(&self, point: Vec2) -> Vec2 {
        Vec2::new(
            map_range(point[0], self._left_x, self._right_x, 0.0, self.width),
            map_range(point[1], self._top_y, self._bottom_y, 0.0, self.height),
        )
    }
}
