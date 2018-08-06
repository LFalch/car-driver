fn c_drag(coefficient: f32, area: f32) -> f32 {
    0.5 * coefficient * area * 1.29
}

#[derive(Debug, Clone)]
pub struct CarSetup {
    pub drag: f32,
    pub rolling_r: f32,
    pub brake_force: f32,
    pub mass: f32,
    pub wheel_radius: f32,
    pub engine: Engine,
    pub transmission: Transmission,
    pub fw_dist: f32,
    pub rw_dist: f32,
}

const RAD_S_TO_RPM: f32 = 60. / (2. * ::std::f32::consts::PI);

impl CarSetup {
    pub fn get_engine_rpm(&self, speed: f32, gear: i8) -> f32 {
        let wheel_rot = speed / self.wheel_radius;
        let rpm = wheel_rot * self.transmission.get_gear_ratio(gear) * self.transmission.final_drive_ratio * RAD_S_TO_RPM;
        rpm.max(self.engine.idle_rpm)
    }

    pub fn get_drive_force(&self, rpm: f32, gear: i8, throttle: f32) -> f32 {
        let engine_torque = throttle * self.engine.get_torque(rpm);
        self.transmission.get_drive_torque(engine_torque, gear) / self.wheel_radius
    }
}

pub fn example() -> CarSetup {
    let drag = c_drag(0.30, 2.2);
    CarSetup {
        fw_dist: 1.,
        rw_dist: 1.,
        drag,
        rolling_r: 30. * drag,
        brake_force: 14_000.,
        mass: 1500.,
        wheel_radius: 0.34,
        transmission: Transmission {
            ratios: vec![2.66, 1.78, 1.30, 1.0, 0.74, 0.50],
            reverse_ratios: vec![2.90],
            efficiency: 0.75,
            final_drive_ratio: 3.42,
        },
        engine: Engine {
            idle_rpm: 1000.,
            redline_rpm: 6000.,
            torque: TorqueCurve::new(1000., 6000., 4600., 475., 390., 380.),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Transmission {
    ratios: Vec<f32>,
    reverse_ratios: Vec<f32>,
    final_drive_ratio: f32,
    efficiency: f32,
}

impl Transmission {
    pub fn get_gear_ratio(&self, gear: i8) -> f32 {
        match gear {
            0 => 0.,
            1...127 => self.ratios[(gear - 1) as usize],
            _ => -self.reverse_ratios[(-gear - 1) as usize],
        }
    }
    pub fn get_drive_torque(&self, engine_torque: f32, gear: i8) -> f32 {
        engine_torque * self.get_gear_ratio(gear) * self.final_drive_ratio * self.efficiency
    }
    pub fn display(&self, gear: i8) -> GearDisp {
        match gear {
            0 => GearDisp::Neutral,
            -1 if self.reverse_ratios.len() == 1 => GearDisp::Reverse,
            n @ 1...127 => GearDisp::G(n as u8),
            n => GearDisp::R(-n as u8)
        }
    }
}

use std::fmt;

pub enum GearDisp{
    Neutral,
    G(u8),
    R(u8),
    Reverse,
}

impl fmt::Display for GearDisp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::GearDisp::*;
        match *self {
            Neutral => "N".fmt(f),
            Reverse => "R".fmt(f),
            G(n) => n.fmt(f),
            R(n) => write!(f, "R{}", n),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Engine {
    pub idle_rpm: f32,
    pub redline_rpm: f32,
    pub torque: TorqueCurve,
}

/// Third degree polynomial approximation of a torque curve
#[derive(Debug, Clone, Copy)]
pub struct TorqueCurve {
    a: f32,
    b: f32,
    c: f32,
    d: f32
}


impl TorqueCurve {
    pub fn new(idle_rpm: f32, redline_rpm: f32, peak_torque_rpm: f32, peak_torque: f32, idle_torque: f32, redline_torque: f32) -> Self {
        let (a, b, c, d) = solve_torque_coefficients(idle_rpm, idle_torque, redline_rpm, redline_torque, peak_torque_rpm, peak_torque);
        TorqueCurve {a, b, c, d}
    }
    pub fn get_value(&self, x: f32) -> f32 {
        let x2 = x*x;
        self.a * x2 * x + self.b * x2 + self.c * x + self.d
    }
    pub fn peak_power(&self) -> (f32, f32) {
        let a = self.a as f64;
        let b = self.b as f64;
        let c = self.c as f64;
        let d = self.d as f64;

        const CBRT2: f64 = 1.25992104989487316476721060727822835057025146470150798008197511215529967651396;

        let rpm = (((-432.*a*a*d + 216.*a*b*c - 54.*b*b*b).powi(2) + 4.*(24.*a*c - 9.*b*b).powi(3)).sqrt() - 432.*a*a*d + 216.*a*b*c - 54.*b*b*b).cbrt() /
            (12.*CBRT2*a) - (24.*a*c - 9.*b*b) / (6.*CBRT2*CBRT2*a*(((-432.*a*a*d + 216.*a*b*c - 54.*b*b*b).powi(2)
            + 4.*(24.*a*c - 9.*b*b).powi(3)).sqrt() - 432.*a*a*d + 216.*a*b*c - 54.*b*b*b).cbrt()) - b/(4.*a);
        let rpm = rpm as f32;

        (rpm, self.get_value(rpm)*rpm/7121.)
    }
}

fn solve_torque_coefficients(x1: f32, y1: f32, x2: f32, y2: f32, xm: f32, ym: f32) -> (f32, f32, f32, f32) {
    let (i, j, k, q, w, e) = (y1 as f64, y2 as f64, ym as f64, x1 as f64, x2 as f64, xm as f64);

    let e2 = e * e;
    let e3 = e2 * e;
    let e4 = e2 * e2;
    let q2 = q * q;
    let q3 = q2 * q;

    let denom = e4*q-e4*w-2.*e3*q2+2.*e3*w*w+e2*q3+3.*e2*q2*w-3.*e2*q*w*w-e2*w*w*w-2.*e*q3*w+2.*e*q*w*w*w+q3*w*w-q2*w*w*w;

    let a = (e2*i-e2*j-2.*e*i*w+2.*e*j*q-2.*e*k*q+2.*e*k*w+i*w*w-j*q2+k*q2-k*w*w) / denom;
    let b = (-2.*e3*i+2.*e3*j+3.*e2*i*w-3.*e2*j*q+3.*e2*k*q-3.*e2*k*w-i*w*w*w+j*q3-k*q3+k*w*w*w) / denom;
    let c = -3.*a*e2-2.*b*e;
    let d = i - a*q3 - b*q2 - c*q;

    (a as f32, b as f32, c as f32, d as f32)
}

impl Engine {
    pub fn get_torque(&self, rpm: f32) -> f32 {
        if rpm < self.idle_rpm {
            self.torque.get_value(self.idle_rpm) * (rpm / self.idle_rpm).max(0.5)
        } else if rpm > self.redline_rpm {
            self.torque.get_value(self.redline_rpm) * (500./ (rpm-self.redline_rpm)).max(0.).min(1.)
        } else {
            self.torque.get_value(rpm)
        }
    }
    pub fn and_power(&self, rpm: f32) -> (f32, f32) {
        let torque = self.get_torque(rpm);
        (torque, torque*rpm/7121.)
    }
}
