use bracket_lib::prelude::*;

mod state;
pub use state::*;
mod map;
pub use map::*;

pub const WIDTH: i32 = 80;
pub const HEIGHT: i32 = 50;

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut draw = DrawBatch::new();

        draw.target(0);
        draw.cls();

        let mut y = 0;
        let mut x = 0;

        for (i, tile) in self.map.tiles.iter().enumerate() {
            let mut bg = HSV::from_f32(0.25, 0.6, 1.0);

            match tile {
                TileType::Water => {
                    bg.h = 0.5;
                    bg.v = self.map.altitude[i];
                }
                TileType::Ground => {
                    bg.h = 0.25;
                    bg.v = (self.map.altitude[i] / 2.0) + 0.3;
                }
            }

            draw.set(Point::new(x, y), ColorPair::new(bg, bg), 2);

            x += 1;
            if x > WIDTH - 1 {
                x = 0;
                y += 1;
            }
        }

        draw.target(1);
        draw.cls();

        draw.print_color_centered(
            4,
            "Sleepy Crawler, a tired roguelike",
            ColorPair::new(WHITE, BLACK),
        );
        draw.print_color_centered(5, "by Dalton Mills", ColorPair::new(WHITE, BLACK));

        draw.submit(0).expect("Batch error");
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::new()
        .with_title("sleepy crawler")
        .with_dimensions(WIDTH as u32, HEIGHT as u32)
        .with_tile_dimensions(16u32, 16u32)
        .with_font("tiles.png", 16u32, 16u32)
        .with_font("vga8x16.png", 8u32, 16u32)
        .with_font("bizcat.png", 8u32, 16u32)
        .with_simple_console(WIDTH as u32, HEIGHT as u32, "tiles.png")
        .with_sparse_console_no_bg((WIDTH * 2) as u32, HEIGHT as u32, "vga8x16.png")
        .build()?;

    let gs = State::new();

    main_loop(ctx, gs)
}
