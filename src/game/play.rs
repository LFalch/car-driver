use ::*;
use ggez::graphics::WHITE;
use std::f32::consts::PI;

/// The state of the game
pub struct Play {
    car: Car,
    rev_meter: PosText,
    engine_performance: PosText,
    gear_text: PosText,
    steer_text: PosText,
}

impl Play {
    pub fn new(a: &Assets, context: &mut Context) -> GameResult<Self> {
        Ok(Play {
            car: Car::new(100., 50.),
            rev_meter: a.text(context, Point2::new(2., 2.), "Revs: 0000 RPM")?,
            engine_performance: a.text(context, Point2::new(2., 18.), "Torque|Power: 200 N m | 100 kW")?,
            gear_text: a.text(context, Point2::new(2., 34.), "Gear: 1")?,
            steer_text: a.text(context, Point2::new(2., 50.), "Steer:  0°")?,
        })
    }
}

/*
 * let ang = angle_to_vec(self.obj.rot);
 * let speed_forwards = self.velocity.dot(&ang);
*/
impl GameState for Play {
    fn update(&mut self, s: &mut State) {
        self.car.update(&s.input);
    }
    fn key_down(&mut self, _s: &mut State, k: Keycode) {
        match k {
            Keycode::Kp0 | Keycode::Num0 => self.car.gear = 0,
            Keycode::Kp1 | Keycode::Num1 => self.car.gear = 1,
            Keycode::Kp2 | Keycode::Num2 => self.car.gear = 2,
            Keycode::Kp3 | Keycode::Num3 => self.car.gear = 3,
            Keycode::Kp4 | Keycode::Num4 => self.car.gear = 4,
            Keycode::Kp5 | Keycode::Num5 => self.car.gear = 5,
            Keycode::Kp6 | Keycode::Num6 => self.car.gear = 6,
            Keycode::Kp9 | Keycode::Num9 => self.car.gear = -1,
            _ => (),
        }
    }
    fn logic(&mut self, s: &mut State, ctx: &mut Context) {
        // Center the camera on the player
        // let p = self.car.obj.pos;
        // s.focus_on(p);
        let ang = angle_to_vec(self.car.obj.rot);
        let speed_forwards = self.car.velocity.dot(&ang);
        let slip_speed = self.car.velocity.perp(&ang);
        let rpm = self.car.engine_speed;
        let (torque, power) = self.car.setup.engine.and_power(rpm);

        self.rev_meter.update_text(&s.assets, ctx, &format!("Revs: {:04.0} RPM  Speed: {:4.0} km/h | ({:2.0} km/h)", rpm, speed_forwards*3.6, slip_speed*3.6)).unwrap();
        self.engine_performance.update_text(&s.assets, ctx, &format!("Torque|Power: {:03.0} N m | {:3.0} hp", torque, power)).unwrap();
        self.gear_text.update_text(&s.assets, ctx, &format!("Gear: {}  |  C: {:4.2} B: {:4.2} T: {:4.2}",
            self.car.setup.transmission.display(self.car.gear), self.car.clutch, self.car.brake, self.car.throttle)).unwrap();
        self.steer_text.update_text(&s.assets, ctx, &format!("Steer: {:2.0}°", self.car.steering_angle*180./PI)).unwrap();
    }

    fn draw(&mut self, s: &State, ctx: &mut Context) -> GameResult<()> {
        graphics::set_color(ctx, WHITE)?;
        self.car.obj.draw(ctx, s.assets.get_img(Sprite::Ferrari))?;

        Ok(())
    }
    fn draw_hud(&mut self, _s: &State, ctx: &mut Context) -> GameResult<()> {
        self.rev_meter.draw_text(ctx)?;
        self.engine_performance.draw_text(ctx)?;
        self.gear_text.draw_text(ctx)?;
        self.steer_text.draw_text(ctx)
    }
}
