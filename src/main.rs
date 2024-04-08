mod conway;

use std::time::Instant;
use std::thread;

use error_iter::ErrorIter as _;

use log::error;

use pixels::{Error, Pixels, SurfaceTexture};

use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const ALIVE_PROBABILITY: f64 = 0.1;

fn main() -> Result<(), Error> {

    let threads = thread::available_parallelism().map(|t| t.get()).unwrap_or(1);

    println!("Running on {} threads", threads);
    println!("Press ESC to exit");
    println!("");


    env_logger::init();
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let mut world = conway::Universe::new(WIDTH as usize, HEIGHT as usize, ALIVE_PROBABILITY, threads);

    let mut last_frame_time = Instant::now();

    event_loop.run(move |event, _, control_flow| {

        // Handle events
        match event {
            Event::RedrawRequested(_) => {
                world.render(pixels.frame_mut());

                match pixels.render() {
                    Ok(_) => {}
                    Err(err) => {
                        log_error("pixels.render", err);
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }
            }
            Event::MainEventsCleared => {
                world.update();
                
                let elapsed = last_frame_time.elapsed();
                
                // Remove last printed line
                print!("\x1b[1A\x1b[2K");
                println!(
                    "FPS: {}",
                    1.0 / elapsed.as_secs_f64()
                );
                
                last_frame_time = Instant::now();
                window.request_redraw();
            }
            Event::WindowEvent { window_id: _, event } => {
                match event {
                    WindowEvent::Resized(new_size) => {
                        match pixels.resize_buffer(new_size.width, new_size.height) {
                            Ok(_) => {
                                match pixels.resize_surface(new_size.width, new_size.height) {
                                    Ok(_) => {
                                        world.resize(new_size.width as usize, new_size.height as usize);
                                    }
                                    Err(err) => {
                                        log_error("pixels.resize_surface", err);
                                        *control_flow = ControlFlow::Exit;
                                        return;
                                    }
                                };
                            }
                            Err(err) => {
                                log_error("pixels.resize_buffer", err);
                                *control_flow = ControlFlow::Exit;
                                return;
                            }
                        }
                    },
                    WindowEvent::KeyboardInput { device_id: _, input, is_synthetic: _ } => {
                        match input.virtual_keycode {
                            Some(VirtualKeyCode::Escape) => {
                                *control_flow = ControlFlow::Exit;
                                return;
                            }
                            _ => {}
                        }
                    },
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    });
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}