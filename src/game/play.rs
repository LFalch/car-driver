use ::*;
use ggez::graphics::{Color, WHITE};

/// The state of the game
pub struct Play {
    car: Car,
    a1: Vector2,
    a2: Vector2,
    a3: Vector2,
    rev_meter: PosText,
    engine_performance: PosText,
    gear_text: PosText,
}

impl Play {
    pub fn new(a: &Assets, context: &mut Context) -> GameResult<Self> {
        Ok(Play {
            car: Car::new(100., 50., 10_000.),
            a1: Vector2::new(0., 0.),
            a2: Vector2::new(0., 0.),
            a3: Vector2::new(0., 0.),
            rev_meter: a.text(context, Point2::new(2., 2.), "Revs: 0000 RPM")?,
            engine_performance: a.text(context, Point2::new(2., 18.), "Torque|Power: 200 N m | 100 kW")?,
            gear_text: a.text(context, Point2::new(2., 34.), "Gear: 1")?,
        })
    }
}

/*
 * let ang = angle_to_vec(self.obj.rot);
 * let speed_forwards = self.velocity.dot(&ang);
*/
const RPM_TO_RAD_S: f32 = 2. * ::std::f32::consts::PI / 60.;
impl GameState for Play {
    fn update(&mut self, s: &mut State) {
        let (a, b, c) = self.car.update(&s.input);
        self.a1 = a; self.a2 = b; self.a3 = c;
    }
    fn logic(&mut self, s: &mut State, ctx: &mut Context) {
        // Center the camera on the player
        // let p = self.car.obj.pos;
        // s.focus_on(p);
        let ang = angle_to_vec(self.car.obj.rot);
        let speed_forwards = self.car.velocity.dot(&ang);
        let rpm = self.car.setup.get_rpm(speed_forwards, 1);
        let torque = self.car.setup.engine.torque.get_value(rpm);
        let power = rpm * torque * RPM_TO_RAD_S / 1000.;

        self.rev_meter.update_text(&s.assets, ctx, &format!("Revs: {:04.0} RPM", rpm)).unwrap();
        self.engine_performance.update_text(&s.assets, ctx, &format!("Torque|Power: {:03.0} N m | {:3.0} kW", torque, power)).unwrap();
        self.gear_text.update_text(&s.assets, ctx, &format!("Gear: {}", 1)).unwrap();
    }

    fn draw(&mut self, s: &State, ctx: &mut Context) -> GameResult<()> {
        graphics::set_color(ctx, WHITE)?;
        self.car.obj.draw(ctx, s.assets.get_img(Sprite::Ferrari))?;
        self.car.draw_lines(ctx, self.a1, self.a2, self.a3)?;

        Ok(())
    }
    fn draw_hud(&mut self, _s: &State, ctx: &mut Context) -> GameResult<()> {
        self.rev_meter.draw_text(ctx)?;
        self.engine_performance.draw_text(ctx)?;
        self.gear_text.draw_text(ctx)
    }
}
