use bracket_lib::prelude::*;

mod state;
pub use state::*;

pub const WIDTH: i32 = 40;
pub const HEIGHT: i32 = 25;

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut dbatch = DrawBatch::new();

        dbatch.target(0);
        dbatch.cls();

        let mut y = 0;
        let mut x = 0;

        for _tile in self.tiles.iter() {
            let fg = RGB::from_f32(1.0, 1.0, 1.0);

            dbatch.set(
                Point::new(x, y),
                ColorPair::new(fg, RGB::from_f32(0., 0., 0.)),
                0,
            );

            x += 1;
            if x > WIDTH - 1 {
                x = 0;
                y += 1;
            }
        }

        dbatch.submit(0).expect("Batch error");
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::new()
        .with_title("sleepy roguelike")
        .with_dimensions(WIDTH as u32, HEIGHT as u32)
        .with_tile_dimensions(16u32, 16u32)
        .with_font("tiles.png", 16u32, 16u32)
        .with_simple_console(WIDTH as u32, HEIGHT as u32, "tiles.png")
        .build()?;

    let gs = State::new();

    main_loop(ctx, gs)
}
