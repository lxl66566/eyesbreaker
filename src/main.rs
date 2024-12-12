#![warn(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]

pub mod cli;
pub mod timer;

use clap::Parser;
use cli::Cli;
use log::info;
use timer::CountDown;
use tray_icon::{menu::Menu, MouseButton, TrayIconBuilder, TrayIconEvent};
use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    let _ = pretty_env_logger::try_init_timed();
    let cli = Cli::parse();

    // 用于计时
    let mut count_down = CountDown::new(cli.time);

    // Since winit doesn't use gtk on Linux, and we need gtk for
    // the tray icon to show up, we need to spawn a thread
    // where we initialize gtk and create the tray_icon
    #[cfg(target_os = "linux")]
    std::thread::spawn(|| {
        use tray_icon::menu::Menu;

        let icon = load_icon(std::path::Path::new(path));

        gtk::init().unwrap();
        let _tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(Menu::new()))
            .with_icon(icon)
            .build()
            .unwrap();

        gtk::main();
    });

    #[cfg(not(target_os = "linux"))]
    let mut tray_icon = None;

    let event_loop = EventLoop::builder().build().unwrap();
    let tray_channel = TrayIconEvent::receiver();
    let (icon1, icon2) = load_icons(include_bytes!("../icon.png"));

    event_loop
        .run(move |event, event_loop| {
            // We add delay of 1000 ms (1fps) to event_loop to reduce cpu load.
            // This can be removed to allow ControlFlow::Poll to poll on each cpu cycle
            // Alternatively, you can set ControlFlow::Wait or use
            // TrayIconEvent::set_event_handler, see https://github.com/tauri-apps/tray-icon/issues/83#issuecomment-1697773065
            event_loop.set_control_flow(ControlFlow::WaitUntil(
                std::time::Instant::now() + std::time::Duration::from_millis(1000),
            ));

            #[cfg(not(target_os = "linux"))]
            if let winit::event::Event::NewEvents(winit::event::StartCause::Init) = event {
                // We create the icon once the event loop is actually running
                // to prevent issues like https://github.com/tauri-apps/tray-icon/issues/90
                tray_icon = Some(
                    TrayIconBuilder::new()
                        .with_menu(Box::new(Menu::new()))
                        .with_icon(icon1.clone())
                        .with_title("Eyesbreaker")
                        .build()
                        .unwrap(),
                );
                // We have to request a redraw here to have the icon actually show up.
                // Winit only exposes a redraw method on the Window so we use core-foundation
                // directly.
                #[cfg(target_os = "macos")]
                unsafe {
                    use core_foundation::runloop::{CFRunLoopGetMain, CFRunLoopWakeUp};

                    let rl = CFRunLoopGetMain();
                    CFRunLoopWakeUp(rl);
                }
            }

            if let Some(tray_icon) = tray_icon.as_mut() {
                let _ = tray_icon.set_tooltip(Some(format!(
                    "Eyesbreaker - next break in {}s",
                    count_down.time_left().ceil()
                )));

                // 如果到时间了，切换图标。
                if count_down.done_once() {
                    tray_icon
                        .set_icon(Some(icon2.clone()))
                        .expect("set icon failed");
                }
            }

            if let Ok(event) = tray_channel.try_recv() {
                match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        ..
                    } => {
                        count_down.reset();
                    }
                    TrayIconEvent::Click {
                        button: MouseButton::Right,
                        ..
                    } => {
                        std::process::exit(0);
                    }
                    _ => {
                        if count_down.done() {
                            if let Some(tray_icon) = tray_icon.as_mut() {
                                tray_icon
                                    .set_icon(Some(icon1.clone()))
                                    .expect("set icon failed");
                            }
                            count_down.reset();
                            info!("count down reset");
                        }
                    }
                }
            }
        })
        .expect("event loop exited");
}

/// Loads an icon from a file and returns both the original icon and a
/// modified version with all non-transparent pixels changed to green.
///
/// The input path should point to a file containing an image in a format
/// supported by the `image` crate. The image should have an alpha channel
/// (i.e. be a 32-bit RGBA image) and the resulting icon will have the same
/// alpha channel.
///
/// Returns a tuple of two `tray_icon::Icon`s: the first is the original
/// icon, and the second is the modified icon with all non-transparent
/// pixels changed to green.
///
/// If the input path cannot be opened or the image cannot be parsed,
/// this function will panic.
fn load_icons(image_data: &[u8]) -> (tray_icon::Icon, tray_icon::Icon) {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(image_data)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let original_icon = tray_icon::Icon::from_rgba(icon_rgba.clone(), icon_width, icon_height)
        .expect("Failed to create original icon");

    // 修改图像数据为绿色（保持透明区域不变）
    let green_rgba: Vec<u8> = icon_rgba
        .chunks_exact(4)
        .flat_map(|pixel| {
            if pixel[3] == 0 {
                // 透明像素
                pixel.to_vec()
            } else {
                // 替换为绿色 (R=0, G=255, B=0, A=原来的透明度)
                vec![0, 255, 0, pixel[3]]
            }
        })
        .collect();

    let green_icon = tray_icon::Icon::from_rgba(green_rgba, icon_width, icon_height)
        .expect("Failed to create green icon");

    (original_icon, green_icon)
}
