#![feature(try_trait)]

extern crate glutin;
#[macro_use] extern crate error_chain;

mod error;
pub mod glutin_window;
pub mod input_state;

pub use error::*;
pub use input_state::InputState;
pub use glutin_window::GlutinWindow;


