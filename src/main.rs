use bracket_lib::prelude::*;

mod state;
pub use state::*;
mod map;
pub use map::*;

pub const WIDTH: i32 = 80;
pub const HEIGHT: i32 = 50;

#[derive(PartialEq, Copy, Clone)]
pub enum MapViewMode {
    Pretty,
    Precipitation,
    Elevation,
    Temperature,
}

fn handle_input(state: &State, ctx: &mut BTerm) -> MapViewMode {
    match ctx.key {
        None => return state.map_view_mode,
        Some(key) => match key {
            VirtualKeyCode::T => MapViewMode::Temperature,
            VirtualKeyCode::E => MapViewMode::Elevation,
            VirtualKeyCode::M => MapViewMode::Precipitation,
            VirtualKeyCode::P => MapViewMode::Pretty,
            _ => state.map_view_mode
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.map_view_mode = handle_input(self, ctx);
        let mut draw = DrawBatch::new();

        draw.target(0);
        draw.cls();

        let mut y = 0;
        let mut x = 0;

        for (i, _tile) in self.map.tiles.iter().enumerate() {
            let bg = match self.map_view_mode {
                MapViewMode::Pretty => self.map.hsv[i],
                MapViewMode::Elevation => HSV::from_f32(0.333, 0.5, self.map.elevation[i]),
                MapViewMode::Precipitation => HSV::from_f32(0.601, 0.5, self.map.precipitation[i]),
                MapViewMode::Temperature => HSV::from_f32(0.111, self.map.temperature[i], self.map.temperature[i] / 1.25 + 0.25),
            };

            draw.set(Point::new(x, y), ColorPair::new(bg, bg), 2);

            x += 1;
            if x > WIDTH - 1 {
                x = 0;
                y += 1;
            }
        }

        draw.target(1);
        draw.cls();

        draw.print_color_centered(4, "Sleepy Crawler, a tired roguelike", ColorPair::new(WHITE, BLACK));
        draw.print_color_centered(5, "by Dalton Mills", ColorPair::new(WHITE, BLACK));
        draw.print_color_centered(1, &format!("FPS: {}", ctx.fps), ColorPair::new(WHITE, BLACK));

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
        .with_font("terrain4x4.png", 4u32, 4u32)
        .with_simple_console(WIDTH as u32, HEIGHT as u32, "tiles.png")
        .with_sparse_console_no_bg((WIDTH * 2) as u32, HEIGHT as u32, "vga8x16.png")
        .build()?;

    let gs = State::new();

    main_loop(ctx, gs)
}
