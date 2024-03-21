use bracket_lib::prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // clear window
        ctx.cls();

        // print something
        ctx.print(1, 1, "Hello, flappy developer");
    }
}

fn main() -> BError {
    // build game terminal instance
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Bird")
        .build()?;

    // game main loop
    main_loop(context, State {})
}
