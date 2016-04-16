extern crate piston_window;
extern crate lambda_oxide;

use piston_window::*;

fn main() {
   lambda_oxide::interpreter();
   let window: PistonWindow =
      WindowSettings::new("Hello Piston!", [640, 480])
      .exit_on_esc(true).build().unwrap();
   for e in window {
      e.draw_2d(|c, g| {
         clear([1.0; 4], g);
         rectangle([1.0, 0.0, 0.0, 1.0], // red
                   [0.0, 0.0, 100.0, 100.0],
                   c.transform, g);
      });
   }
}

