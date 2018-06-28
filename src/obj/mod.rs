use ggez::{Context, GameResult};
use ggez::graphics::{self, Point2, Vector2, Image};
// use ggez::nalgebra as na;

mod car;
pub mod parts;

pub use self::car::*;

#[derive(Debug)]
/// A simple object that can be drawn to the screen
pub struct Object {
    /// The position of the object
    pub pos: Point2,
    /// The rotation of the obejct in radians
    pub rot: f32,
}

impl Object {
    /// Make a new physics object
    pub fn new(pos: Point2) -> Self {
        Object {
            pos,
            rot: 0.,
        }
    }
    /// Draw the object
    pub fn draw(&self, ctx: &mut Context, img: &Image) -> GameResult<()> {
        let drawparams = graphics::DrawParam {
            dest: self.pos,
            rotation: self.rot,
            offset: Point2::new(0.5, 0.5),
            .. Default::default()
        };
        graphics::draw_ex(ctx, img, drawparams)
    }
}
