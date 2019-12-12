use crate::prelude::*;
use crate::compose::Compose;
use crate::GlobalOpts;
use std::error::Error;

pub mod build;
pub mod down;
pub mod pull;
pub mod push;
pub mod restart;
pub mod run;
pub mod start;
pub mod stop;
pub mod up;
pub mod util;


pub trait Command {
    fn run(compose: Compose, global: GlobalOpts, opts: Self) -> Result<(), Box<dyn Error>>;
}
