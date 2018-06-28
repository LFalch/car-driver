use std::f32::consts::PI;

use super::parts::{CarSetup, example};
use super::*;
use ::{InputState, DELTA, angle_to_vec, BLUE, RED, GREEN};

const C_DRAG: f32 = 0.5 * 0.30 * 2.2 * 1.29;
const ROLLING_RESISTANCE: f32 = 30. * C_DRAG;

fn c_drag(coefficient: f32, area: f32) -> f32 {
    0.5 * coefficient * area * 1.29
}

#[derive(Debug)]
pub struct Car {
    pub obj: Object,
    pub velocity: Vector2,
    pub setup: CarSetup,
    drag: f32,
    rolling_r: f32,
    brake_force: f32,
}

impl Car {
    pub fn new(x: f32, y: f32, brakes: f32) -> Self {
        Car {
            obj: Object::new(Point2::new(x, y)),
            velocity: Vector2::new(0., 0.),
            setup: example(),
            brake_force: brakes,
            drag: C_DRAG,
            rolling_r: ROLLING_RESISTANCE,
        }
    }
    pub fn update(&mut self, input: &InputState) -> (Vector2, Vector2, Vector2) {
        let ang = angle_to_vec(self.obj.rot);
        let speed_forwards = self.velocity.dot(&ang);
        let vel_forwards = speed_forwards * ang;

        self.obj.rot += input.hor() * 2. * PI * 0.01 * speed_forwards * DELTA;

        let drag = -self.drag * self.velocity.norm() * self.velocity;
        let rr = -self.rolling_r * vel_forwards;
        let traction;
        match input.ver {
            -1 => traction = ang * self.setup.get_drive_force(speed_forwards, 1, 1.),
            1 if self.velocity.norm() > 0. => traction = -self.brake_force * self.velocity.normalize(),
            _ => traction = Vector2::new(0., 0.),
        }
        let acc = (traction + drag + rr) / self.setup.mass;

        self.obj.pos += (self.velocity * DELTA + 0.5 * acc * DELTA * DELTA) * 15.;
        self.velocity += acc * DELTA;

        (traction, drag, rr)
    }
    pub fn draw_lines(&self, ctx: &mut Context, traction: Vector2, drag: Vector2, rr: Vector2) -> GameResult<()> {
        let pos = self.obj.pos;

        graphics::set_color(ctx, GREEN)?;
        graphics::line(ctx, &[pos, pos+rr], 2.)?;

        graphics::set_color(ctx, RED)?;
        graphics::line(ctx, &[pos, pos+drag], 2.)?;

        graphics::set_color(ctx, graphics::WHITE)?;
        graphics::line(ctx, &[pos, pos+traction], 2.)
    }
}
