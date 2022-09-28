use fltk::{app, enums::Color, frame, prelude::*, window::Window};
use fltk_theme::colors::aqua::dark::{
    controlAccentColor, controlColor, labelColor, windowBackgroundColor,
};
use fltk_theme::{SchemeType, WidgetScheme};

use sysinfo::{self, SystemExt};

const UPDATE_TIMEOUT: u128 = 500;

struct MyApp {
    sys_info: sysinfo::System,
    last_update: std::time::Instant,
}

#[derive(Debug)]
enum SysInfo {
    RamUsage,
    RamTotal,
    DiskName(usize), // index
    DiskType(usize),
    DiskFileSystem(usize),
    DiskMountPoint(usize),
    DiskAvailableSpace(usize),
    DiskTotalSpace(usize),
    NetworkName(usize),
    NetworkDataReceived(usize),
    NetworkDataTransmited(usize),
}

impl MyApp {
    fn new() -> Self {
        let sys_info = sysinfo::System::new_all();
        let last_update = std::time::Instant::now();
        Self {
            sys_info,
            last_update,
        }
    }

    fn update(&mut self) {
        if self.last_update.elapsed().as_millis() > UPDATE_TIMEOUT {
            self.sys_info.refresh_all();
            self.last_update = std::time::Instant::now()
        }
    }
    fn get_info(&self, info_type: SysInfo) -> String {
        match info_type {
            SysInfo::RamUsage => self.sys_info.used_memory().to_string(),
            SysInfo::RamTotal => self.sys_info.total_memory().to_string(),
            _ => {
                println!("Not yet handled: {:?}", info_type);
                String::new()
            }
        }
    }
}

fn convert_size(byte_size: f32) -> (i32, String) {
    if byte_size == 0. {
        return (0, "B".to_string());
    }
    let size_names: Vec<&str> = vec!["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let i = byte_size.log(1024.).floor();
    let p = 1024_f32.powf(i);
    let s = byte_size / p;

    (s as i32, size_names[i as usize].to_string())
}

fn main() {
    let a = app::App::default();
    app::background(
        windowBackgroundColor.0,
        windowBackgroundColor.1,
        windowBackgroundColor.2,
    );
    app::background2(
        controlAccentColor.0,
        controlAccentColor.1,
        controlAccentColor.2,
    );
    app::foreground(labelColor.0, labelColor.1, labelColor.2);
    app::set_color(Color::Selection, 255, 255, 255);

    let widget_scheme = WidgetScheme::new(SchemeType::Aqua);
    widget_scheme.apply();

    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    let mut frame = frame::Frame::new(0, 0, 200, 50, "");

    let mut my_app = MyApp::new();

    frame.set_color(Color::from_rgba_tuple(*controlColor));

    wind.end();
    wind.show();

    while a.wait() {
        my_app.update();
        let total_ram =
            convert_size(my_app.get_info(SysInfo::RamTotal).parse::<f32>().unwrap() * 1024.);
        let ram_usage =
            convert_size(my_app.get_info(SysInfo::RamUsage).parse::<f32>().unwrap() * 1024.);

        let f = format!(
            "Ram: {} {}/ {} {}",
            ram_usage.0, ram_usage.1, total_ram.0, total_ram.1
        );
        frame.set_label(&f);

        wind.redraw();
        app::sleep(0.016)
    }
}
