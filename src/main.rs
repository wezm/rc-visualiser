use std::error::Error;
use std::f64::consts::TAU;
use std::process::Command;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureAccess, TextureQuery, WindowCanvas};

use cairo::{Format, ImageSurface};
use gilrs::{Axis, Event as GilEvent, EventType, Gilrs};

use crate::config::{ChannelConfig, Config};

mod config;

#[derive(Debug, Default)]
struct State {
    /// Aileron
    channel_1: f32,
    /// Elevator
    channel_2: f32,
    /// Throttle
    channel_3: f32,
    /// Rudder
    channel_4: f32,
}

const WINDOW_WIDTH: u32 = 900;
const WINDOW_HEIGHT: u32 = 500;

fn main() -> Result<(), Box<dyn Error>> {
    println!("RC Visualiser {}", env!("CARGO_PKG_VERSION"));
    let config = Config::load("config.toml")?;
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;

    let scale: u16 = config
        .gui
        .scale
        .or_else(|| {
            std::env::var("GDK_SCALE")
                .ok()
                .and_then(|scale| scale.parse().ok())
                .or_else(|| {
                    Command::new("gsettings")
                        .args(&["get", "org.gnome.desktop.interface", "scaling-factor"])
                        .output()
                        .ok()
                        .and_then(|output| {
                            if output.status.success() {
                                std::str::from_utf8(&output.stdout)
                                    .ok()
                                    .and_then(|text| text.split_ascii_whitespace().last())
                                    .and_then(|scale| scale.parse().ok())
                            } else {
                                None
                            }
                        })
                })
        })
        .unwrap_or(1);

    let mut gilrs = Gilrs::new().unwrap();
    let mut gamepad_name = None;

    // Iterate over all connected gamepads, get the name of the first one
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
        if gamepad_name.is_none() {
            gamepad_name = Some(gamepad.name().to_owned());
        }
    }

    let window = video_subsys
        .window(
            gamepad_name.as_deref().unwrap_or("RC Transmitter"),
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        )
        // .resizable()
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let size = window.drawable_size();
    println!("Scale: {}", scale);
    println!("Window: '{}' {:?}", window.title(), window.size());
    println!("Drawable: {:?}", size);

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let mut cairo_texture = texture_creator.create_texture(
        Some(PixelFormatEnum::RGB888),
        TextureAccess::Streaming,
        size.0,
        size.1,
    )?;
    // dbg!(cairo_texture.query());

    let mut state = State::default();

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => {
                    println!("shutting down");
                    break 'mainloop;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {}
                _ => {
                    // println!("Event {:?}", event);
                }
            }
        }

        while let Some(GilEvent { id, event, time }) = gilrs.next_event() {
            let _ = id; // Suppress warnings when println below is commented out
            let _ = time;
            // println!("{:?} New event from {}: {:?}", time, id, event);
            match event {
                EventType::ButtonPressed(_button, _code) => {}
                EventType::ButtonRepeated(_button, _code) => {}
                EventType::ButtonReleased(_button, _code) => {}
                EventType::ButtonChanged(_button, _value, _code) => {}
                EventType::AxisChanged(Axis::LeftStickX, value, _code) => {
                    state.channel_1 = scale_value(value, config.channels.channel1())
                }
                EventType::AxisChanged(Axis::LeftStickY, value, _code) => {
                    state.channel_2 = scale_value(value, config.channels.channel2())
                }
                EventType::AxisChanged(Axis::LeftZ, value, _code) => {
                    state.channel_3 = scale_value(value, config.channels.channel3())
                }
                EventType::AxisChanged(Axis::RightStickX, value, _code) => {
                    state.channel_4 = scale_value(value, config.channels.channel4())
                }
                EventType::AxisChanged(_axis, _value, _code) => {}
                EventType::Connected => {}
                EventType::Disconnected => {}
                EventType::Dropped => {}
            }
        }

        clear(&mut canvas);
        paint(&mut cairo_texture, scale, &state);
        let TextureQuery { width, height, .. } = cairo_texture.query();
        let target = Rect::new(0, 0, width, height);
        canvas.copy(&cairo_texture, None, Some(target)).unwrap();
        canvas.present();
    }

    Ok(())
}

// Map the raw value from the receiver to -0.5..0.5
fn scale_value(value: f32, config: ChannelConfig) -> f32 {
    let scaled = value / config.max * 0.5;
    if config.invert {
        -scaled
    } else {
        scaled
    }
}

fn paint(texture: &mut Texture, scale: u16, state: &State) {
    let scale = scale as f64;
    let margin = scale * 10.;
    let TextureQuery { width, height, .. } = texture.query();
    texture
        .with_lock(None, |data, pitch| {
            let surface = unsafe {
                ImageSurface::create_for_data_unsafe(
                    data.as_mut_ptr(),
                    Format::Rgb24,
                    width as i32,
                    height as i32,
                    pitch as i32,
                )
            }
            .expect("unable to create cairo surface");
            let cr = cairo::Context::new(&surface).expect("unable to create cairo context");

            // Clear texture
            cr.rectangle(0., 0., width as f64, height as f64);
            cr.set_source_rgb(1.0, 1.0, 1.0);
            cr.fill().unwrap();

            cr.scale(scale as f64, scale as f64); // dpi scale

            let gimbals_width = 2. * 100. + 5. * margin;
            let gimbals_height = 100.;
            cr.translate(
                (width as f64 / scale / 2.) - (gimbals_width / 2.),
                (height as f64 / scale / 2.) - (gimbals_height / 2.),
            );

            cr.save().unwrap();
            cr.translate(50., 50.);
            cr.scale(100., 100.); // scale gimbal, nominally 100 points
            draw_gimbal(&cr, scale, state.channel_4, state.channel_3);
            cr.restore().unwrap();

            cr.save().unwrap();
            cr.translate(150. + (5. * margin), 50.);
            cr.scale(100., 100.);
            draw_gimbal(&cr, scale, state.channel_1, state.channel_2);
            cr.restore().unwrap();
        })
        .unwrap();
}

fn clear(canvas: &mut WindowCanvas) {
    canvas.set_draw_color(Color::GRAY);
    canvas.clear();
}

fn draw_gimbal(cr: &cairo::Context, scale: f64, x_val: f32, y_val: f32) {
    cr.set_source_rgb(0.8, 0.8, 0.8);
    cr.arc(0., 0., 0.825, 0., TAU);
    cr.fill_preserve().unwrap();

    cr.move_to(-0.5, 0.);
    cr.line_to(0.5, 0.);
    cr.move_to(0., -0.5);
    cr.line_to(0., 0.5);

    cr.save().unwrap();
    cr.identity_matrix();
    cr.set_line_width(2. * scale as f64);
    cr.set_source_rgb(0.7, 0.7, 0.7);
    cr.stroke().unwrap();
    cr.restore().unwrap();

    cr.set_source_rgb(0.3, 0.3, 0.3);
    cr.rectangle(-0.5, -0.5, 1., 1.);

    cr.save().unwrap();
    cr.identity_matrix();
    cr.set_line_width(1. * scale as f64);
    cr.stroke().unwrap();
    cr.restore().unwrap();

    cr.translate(x_val as f64, y_val as f64);
    cr.set_source_rgb(179. / 255., 52. / 255., 121. / 255.);
    cr.arc(0., 0., 0.1, 0., TAU);
    cr.fill().unwrap();
}
