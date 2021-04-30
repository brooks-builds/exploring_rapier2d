use exploring_rapier2d::MainState;
use eyre::Result;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::{event, ContextBuilder};

fn main() -> Result<()> {
    let window_mode = WindowMode::default();
    let window_setup = WindowSetup::default().title("Exploring Rapier2d");
    let (mut context, mut event_loop) = ContextBuilder::new("exploring_rapier2d", "Brookzerker")
        .window_mode(window_mode)
        .window_setup(window_setup)
        .build()?;
    let mut main_state = MainState::new(&mut context)?;

    event::run(&mut context, &mut event_loop, &mut main_state)?;
    Ok(())
}
