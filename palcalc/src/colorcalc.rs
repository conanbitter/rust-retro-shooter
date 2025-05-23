use std::{path::Path, sync::Mutex};

use crate::{
    colors::FloatColor,
    interface::{StatusCalculating, Tui},
};
use anyhow::Result;
use image::io::Reader as ImageReader;
use rand::Rng;
use rayon::prelude::*;
pub struct ColorData(Vec<Vec<Vec<u64>>>);

impl ColorData {
    pub fn new() -> ColorData {
        ColorData(vec![vec![vec![0u64; 256]; 256]; 256])
    }

    pub fn add(&mut self, filename: &Path) -> Result<()> {
        let img = ImageReader::open(filename)?.decode()?.to_rgb8();
        for pixel in img.enumerate_pixels() {
            let color = pixel.2;
            self.0[color[0] as usize][color[1] as usize][color[2] as usize] += 1;
        }
        Ok(())
    }
}

struct ColorPoint {
    color: FloatColor,
    segment: i32,
    count: u64,
    distance: f64,
}

impl ColorPoint {
    fn distance_squared(&mut self, c: FloatColor) -> f64 {
        let dist = self.color.distance_squared(c);
        if dist < self.distance {
            self.distance = dist;
            return dist;
        }
        self.distance
    }
}

pub struct ColorCalc {
    points: Vec<ColorPoint>,
    centroids: Vec<FloatColor>,

    colors: i32,
    point_count: u64,

    total_distance: f64,
    points_changed: u64,

    pub unique_colors: u64,
    pub fixed_colors: u64,
}

impl ColorCalc {
    pub fn new(color_count: i32, adjustable_colors: ColorData, fixed_colors: ColorData) -> ColorCalc {
        let mut total_colors = {
            if color_count > 256 {
                256u64
            } else if color_count < 1 {
                1u64
            } else {
                color_count as u64
            }
        };

        let mut points = vec![];

        for r in 0..256 {
            for g in 0..256 {
                for b in 0..256 {
                    if adjustable_colors.0[r][g][b] > 0 {
                        points.push(ColorPoint {
                            color: FloatColor::new(r as i32, g as i32, b as i32),
                            segment: 0,
                            count: adjustable_colors.0[r][g][b],
                            distance: f64::MAX,
                        })
                    }
                }
            }
        }

        let unique_colors = points.len() as u64;
        if total_colors > unique_colors {
            total_colors = unique_colors;
        }

        ColorCalc {
            colors: color_count,
            points,
            centroids: vec![FloatColor::BLACK; total_colors as usize],
            point_count: unique_colors,
            total_distance: 0.0,
            points_changed: 0,
            unique_colors,
            fixed_colors: 0,
        }
    }

    fn init_centroids(&mut self) {
        let mut rng = rand::thread_rng();
        self.points.swap(0, rng.gen_range(0..self.point_count) as usize);
        for cent_ind in 1..(self.colors - 1) as usize {
            let mut sum = 0.0;
            let cent_color = self.points[cent_ind - 1].color;
            for i in cent_ind - 1..self.point_count as usize {
                sum += self.points[i].distance_squared(cent_color);
            }

            let rnd = sum * rng.gen::<f64>();
            sum = 0.0;
            let mut next = self.point_count as usize - 1;
            for i in cent_ind + 1..self.point_count as usize {
                sum += self.points[i].distance;
                if sum > rnd {
                    next = i;
                    break;
                }
            }
            self.points.swap(cent_ind, next);
        }
        for i in 0..self.colors {
            self.centroids[i as usize] = self.points[i as usize].color;
        }
    }

    fn calc_centroids(&mut self) {
        let mut new_centroids = vec![FloatColor::BLACK; self.colors as usize];
        let mut counts = vec![0u64; self.colors as usize];
        for point in &self.points {
            counts[point.segment as usize] += point.count;
            let c = &mut new_centroids[point.segment as usize];
            c.r += point.color.r * (point.count as f64);
            c.g += point.color.g * (point.count as f64);
            c.b += point.color.b * (point.count as f64);
        }

        self.total_distance = 0.0;

        for (i, c) in self.centroids.iter_mut().enumerate() {
            if counts[i] == 0 {
                continue;
            }

            let count = counts[i] as f64;
            new_centroids[i].r /= count;
            new_centroids[i].g /= count;
            new_centroids[i].b /= count;
            self.total_distance += new_centroids[i].distance(*c);
            *c = new_centroids[i];
        }
    }

    fn calc_segments(&mut self) {
        let points_changed = Mutex::new(0u64);

        self.points.par_iter_mut().for_each(|point| {
            let old_seg = point.segment;
            let mut new_seg = old_seg;
            let mut min_dist = point.color.distance(self.centroids[old_seg as usize]);
            for (i, c) in self.centroids.iter().enumerate() {
                let dist = point.color.distance(*c);
                if min_dist > dist {
                    min_dist = dist;
                    new_seg = i as i32;
                }
            }
            if new_seg != old_seg {
                point.segment = new_seg;
                let mut changed = points_changed.lock().unwrap();
                *changed += 1;
            }
        });

        self.points_changed = *points_changed.lock().unwrap();
    }

    fn update_stats(
        &self,
        block: &mut StatusCalculating,
        tui: &mut Tui,
        attempt: usize,
        step: usize,
        passed: usize,
    ) -> Result<()> {
        let step_current;
        let steps_total;
        if attempt == 0 {
            step_current = step;
            steps_total = 5 * 1000;
        } else {
            step_current = passed + step;
            steps_total = passed + passed / attempt * (5 - attempt);
        }

        block.update(
            tui,
            attempt as u32,
            step as u32,
            self.points_changed,
            self.total_distance * 100.0,
            step_current as u32,
            steps_total as u32,
        )?;
        Ok(())
    }

    pub fn run(&mut self, block: &mut StatusCalculating, tui: &mut Tui) -> Result<()> {
        let mut steps_passed = 0;
        for a in 0..5 {
            self.init_centroids();
            for s in 0..1000 {
                self.calc_segments();
                if self.points_changed == 0 {
                    self.update_stats(block, tui, a, s, steps_passed)?;
                    steps_passed += s;
                    break;
                }
                self.calc_centroids();
                if block.timer.needs_update() {
                    self.update_stats(block, tui, a, s, steps_passed)?;
                }
            }
        }
        Ok(())
    }
}
