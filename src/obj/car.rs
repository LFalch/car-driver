use std::f32::consts::{PI, FRAC_PI_2, FRAC_PI_4};

use ext::FloatExt;
use super::parts::{CarSetup, example};
use super::*;
use ::{InputState, DELTA, angle_to_vec, BLUE, RED, GREEN};

#[derive(Debug)]
pub struct Car {
    pub obj: Object,
    pub velocity: Vector2,
    pub setup: CarSetup,
    pub engine_speed: f32,
    pub steering_angle: f32,
    pub brake: f32,
    pub throttle: f32,
    pub clutch: f32,
    pub gear: i8,
}

impl Car {
    pub fn new(x: f32, y: f32) -> Self {
        let setup = example();
        Car {
            obj: Object::new(Point2::new(x, y)),
            velocity: Vector2::new(0., 0.),
            engine_speed: setup.engine.idle_rpm(),
            setup,
            steering_angle: 0.,
            brake: 0.,
            throttle: 0.,
            clutch: 0.,
            gear: 0,
        }
    }
    pub fn update(&mut self, input: &InputState) -> (Vector2, Vector2, Vector2) {
        let ang = angle_to_vec(self.obj.rot+self.steering_angle);
        let speed_forwards = self.velocity.dot(&ang);
        let speed_sideways = self.velocity.perp(&ang);
        let vel_forwards = speed_forwards * ang;
        let vel_sideways = speed_sideways * Vector2::new(ang.y, -ang.x);

        if input.hor() == 0. {
            if self.steering_angle.abs() <= PI * DELTA {
                self.steering_angle = 0.;
            } else {
                self.steering_angle -= self.steering_angle.signum() * PI * DELTA;
            }
        } else {
            self.steering_angle += input.hor() * FRAC_PI_2 * DELTA;
            if self.steering_angle.abs() > FRAC_PI_4 {
                self.steering_angle = self.steering_angle.signum() * FRAC_PI_4;
            }
        }

        const L: f32 = 2.;
        let car_ang = angle_to_vec(self.obj.rot);
        let car_ang_kryds = Vector2::new(car_ang.y, -car_ang.x);

        let angle_diff = speed_forwards * self.steering_angle.sin() / L * DELTA;
        self.obj.pos += L/2. * (angle_diff.sin()*car_ang_kryds - (1. - angle_diff.cos())*car_ang);
        self.obj.rot += angle_diff;

        // NOTE implement using clutch

        let rr = -self.setup.rolling_r * vel_forwards;
        let mut traction = rr;
        match input.ver {
            -1 => self.throttle.cap_add(4. * DELTA, 1.),
            1 => {
                if self.brake >= 0.5 {
                    self.brake += 8. * DELTA;
                }
                self.brake.cap_add(8. * DELTA, 1.);
            }
            _ => {
                self.brake.cap_sub(8. * DELTA, 0.);
                self.throttle.cap_sub(8. * DELTA, 0.);
            }
        }

        traction += ang * self.setup.get_drive_force(speed_forwards, self.gear, self.throttle);

        let drag = -self.setup.drag * self.velocity.norm() * self.velocity;
        let grip_force = -vel_sideways * self.setup.mass / DELTA;

        let mut total_force = traction + drag + grip_force;

        let brake_force;
        if speed_forwards != 0. || speed_sideways != 0. {
            let brake = self.setup.brake_force * self.brake;
            let max_brake = self.velocity.norm() * self.velocity.norm() * self.setup.mass / DELTA;
            if brake > max_brake {
                brake_force = -max_brake * self.velocity.normalize();
            } else {
                brake_force = -brake * self.velocity.normalize();
            }
            total_force += brake_force;
        } else {
            brake_force = Vector2::new(0., 0.);
        }

        let acc = total_force / self.setup.mass;

        self.obj.pos += (self.velocity * DELTA + 0.5 * acc * DELTA * DELTA) * 15.;
        self.velocity += acc * DELTA;

        (traction, drag, brake_force)
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
