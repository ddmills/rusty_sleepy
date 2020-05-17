use bracket_lib::prelude::*;

mod state;
pub use state::*;
mod world;
pub use world::*;
mod color_lut;
pub use color_lut::*;

pub const WIDTH: u32 = 80;
pub const HEIGHT: u32 = 50;

#[derive(PartialEq, Copy, Clone)]
pub enum WorldViewMode {
    Biome,
    Flat,
    Precipitation,
    Elevation,
    Temperature,
}

fn handle_input(state: &mut State, ctx: &mut BTerm) {
    state.world_view_mode = match ctx.key {
        None => state.world_view_mode,
        Some(key) => match key {
            VirtualKeyCode::T => {
                state.is_dirty = true;
                WorldViewMode::Temperature
            }
            VirtualKeyCode::E => {
                state.is_dirty = true;
                WorldViewMode::Elevation
            }
            VirtualKeyCode::P => {
                state.is_dirty = true;
                WorldViewMode::Precipitation
            }
            VirtualKeyCode::B => {
                state.is_dirty = true;
                WorldViewMode::Biome
            }
            VirtualKeyCode::F => {
                state.is_dirty = true;
                WorldViewMode::Flat
            }
            VirtualKeyCode::R => {
                state.is_dirty = true;
                state.world.set_seed(state.world.seed + 1);
                state.world.generate();
                state.world_view_mode
            }
            _ => state.world_view_mode,
        },
    };
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        handle_input(self, ctx);
        let mut draw = DrawBatch::new();

        if self.is_dirty {
            draw.target(0);
            draw.cls();

            let mut y = 0;
            let mut x = 0;

            for (i, _tile) in self.world.tiles.iter().enumerate() {
                let elevation = self.world.elevation[i];
                let precipitation = self.world.precipitation[i];
                let temperature = self.world.temperature[i];

                let bg = match self.world_view_mode {
                    WorldViewMode::Biome => {
                        let mut c = HSV::from_f32(0.55, 0.5, 0.5);

                        if elevation > SEA_LEVEL {
                            let t = temperature;
                            let p = precipitation;
                            c = self.biome_lut.get_hsv(t, p);
                        }
                        c.v = (c.v + elevation) / 2.0;

                        c
                    }
                    WorldViewMode::Flat => {
                        let mut c = HSV::from_f32(0.55, 0.5, 0.5);

                        if elevation > SEA_LEVEL {
                            let t = temperature;
                            let p = precipitation;
                            c = self.biome_lut.get_hsv(t, p);
                        }

                        c
                    }
                    WorldViewMode::Elevation => HSV::from_f32(0.0, 0.5, elevation),
                    WorldViewMode::Precipitation => HSV::from_f32(0.601, 0.5, precipitation),
                    WorldViewMode::Temperature => HSV::from_f32(0.111, temperature, temperature),
                };

                draw.set(Point::new(x, y), ColorPair::new(bg, bg), 2);

                x += 1;
                if x > WIDTH - 1 {
                    x = 0;
                    y += 1;
                }
            }
            self.is_dirty = false;
        }

        ctx.set_active_console(2);
        draw.target(2);
        draw.cls();
        let mouse_pos = ctx.mouse_pos();

        draw.set(
            Point::new(mouse_pos.0, mouse_pos.1),
            ColorPair::new(YELLOW, YELLOW),
            2,
        );

        // draw.draw_hollow_box(
        //     Rect::with_size(mouse_pos.0 - 2, mouse_pos.1 - 2, 4, 4),
        //     ColorPair::new(MAGENTA, MAGENTA),
        // );

        draw.target(1);
        draw.cls();

        draw.print_color_centered(
            1,
            &format!("FPS: {}", ctx.fps),
            ColorPair::new(WHITE, BLACK),
        );
        draw.print(
            Point::new(1, HEIGHT - 2),
            "(B) Biome (F) Flat (P) Precipitation (E) Elevation (T) Temperature (R) Re-roll",
        );

        let mouse_x = mouse_pos.0;
        let mouse_y = mouse_pos.1;

        let mouse_idx = self.world.idx(mouse_x, mouse_y);

        draw.print(Point::new(1, 3), &format!("seed {}", self.world.seed));

        draw.print(Point::new(1, 4), &format!("({}, {})", mouse_x, mouse_y));
        draw.print(
            Point::new(1, 5),
            &format!("(T) Temper {}", self.world.temperature[mouse_idx]),
        );
        draw.print(
            Point::new(1, 6),
            &format!("(E) Elevat {}", self.world.elevation[mouse_idx]),
        );
        draw.print(
            Point::new(1, 7),
            &format!("(P) Precip {}", self.world.precipitation[mouse_idx]),
        );

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
        .with_sparse_console_no_bg((WIDTH * 2) as u32, HEIGHT as u32, "bizcat.png")
        .with_sparse_console_no_bg(WIDTH as u32, HEIGHT as u32, "tiles.png")
        .build()?;

    let gs = State::new();

    main_loop(ctx, gs)
}
