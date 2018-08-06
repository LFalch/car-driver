use std::f32::consts::{PI, FRAC_PI_2, FRAC_PI_4};

use ext::FloatExt;
use super::setup::{CarSetup, example};
use super::*;
use ::{InputState, PIXELS_PER_METER, DELTA, angle_to_vec};

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
            engine_speed: setup.engine.idle_rpm,
            setup,
            steering_angle: 0.,
            brake: 0.,
            throttle: 0.,
            clutch: 1.,
            gear: 0,
        }
    }
    pub fn update(&mut self, input: &InputState) {
        let ang = angle_to_vec(self.obj.rot+self.steering_angle);
        let side_ang = Vector2::new(ang.y, -ang.x);
        let speed_forwards = self.velocity.dot(&ang);
        let speed_sideways = self.velocity.perp(&ang);

        let input_hor = input.hor();

        if input_hor == 0. {
            if self.steering_angle.abs() <= 2. * PI * DELTA {
                self.steering_angle = 0.;
            } else {
                self.steering_angle -= self.steering_angle.signum() * 2. * PI * DELTA;
            }
        } else {
            self.steering_angle += input_hor * FRAC_PI_2 * DELTA;
            if self.steering_angle.abs() > FRAC_PI_4 {
                self.steering_angle = self.steering_angle.signum() * FRAC_PI_4;
            }
        }

        let car_ang = angle_to_vec(self.obj.rot);
        let car_ang_kryds = Vector2::new(car_ang.y, -car_ang.x);

        let angle_diff = speed_forwards * self.steering_angle.sin() / (self.setup.rw_dist + self.setup.fw_dist) * DELTA;
        self.obj.pos += self.setup.rw_dist * (angle_diff.sin()*car_ang_kryds - (1. - angle_diff.cos())*car_ang) * PIXELS_PER_METER;
        self.obj.rot += angle_diff;

        // NOTE implement using clutch

        let rr = -self.setup.rolling_r * speed_forwards;
        let mut traction_f = rr;
        if input.acltr() {
            self.throttle.cap_add(4. * DELTA, 1.);
        } else {
            self.throttle.cap_sub(8. * DELTA, 0.);
        }
        if self.engine_speed >= self.setup.engine.redline_rpm {
            self.throttle = 0.;
        }
        if input.brk() {
            if self.brake >= 0.5 {
                self.brake += 8. * DELTA;
            }
            self.brake.cap_add(8. * DELTA, 1.);
        } else {
            self.brake.cap_sub(16. * DELTA, 0.);
        }

        traction_f += self.setup.get_drive_force(self.engine_speed, self.gear, self.throttle);

        let drag = -self.setup.drag * self.velocity.norm() * self.velocity;
        let grip_force = -speed_sideways * side_ang * self.setup.mass / DELTA;

        let gear_rpm = self.setup.get_engine_rpm(speed_forwards, self.gear);

        let traction;

        if self.gear == 0 {
            traction = Vector2::new(0., 0.);

            self.engine_speed += self.throttle * self.setup.engine.get_torque(self.engine_speed) * DELTA / 0.1;
            if self.throttle == 0. {
                self.engine_speed.cap_sub(1000. * DELTA, self.setup.engine.idle_rpm);
            }
        } else {
            traction = if self.clutch == 0. {
                traction_f
            } else if self.clutch == 1. {
                self.engine_speed += self.throttle * self.setup.engine.get_torque(self.engine_speed) * DELTA;

                0.
            } else {
                let rpm_diff = self.engine_speed - gear_rpm;

                (1. - (rpm_diff / 500.).max(0.).min(1.)) * (1. - self.clutch) * traction_f
            } * ang;
        }

        let brake_force;
        if speed_forwards != 0. || speed_sideways != 0. {
            let brake = self.setup.brake_force * self.brake;
            let max_brake = self.velocity.norm() * self.velocity.norm() * self.setup.mass / DELTA;
            if brake > max_brake {
                brake_force = -max_brake * self.velocity.normalize();
            } else {
                brake_force = -brake * self.velocity.normalize();
            }
        } else {
            brake_force = Vector2::new(0., 0.);
        }

        let total_force = traction + drag + grip_force + brake_force;
        let acc = total_force / self.setup.mass;

        self.obj.pos += (self.velocity * DELTA + 0.5 * acc * DELTA * DELTA) * PIXELS_PER_METER;
        self.velocity += acc * DELTA;

        if self.clutch == 0. && self.gear != 0 {
            self.engine_speed = self.setup.get_engine_rpm(speed_forwards, self.gear);
        }

        if self.engine_speed <= self.setup.engine.idle_rpm {
            self.clutch = 1. - self.throttle * 0.5;
        } else {
            self.clutch = 0.;
        }
    }
}
