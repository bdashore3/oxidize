#![feature(rustc_private)]

extern crate libremarkable;
use libremarkable::appctx;
use libremarkable::framebuffer::cgmath;
use libremarkable::framebuffer::common::color;
use libremarkable::input::{
    gpio,
    multitouch,
    wacom,
};
use libremarkable::ui_extensions::element::{
    UIElement,
    UIElementHandle,
    UIElementWrapper,
    UIConstraintRefresh,
};

#[cfg(feature = "enable-runtime-benchmarking")]
use libremarkable::stopwatch;

#[macro_use]
extern crate lazy_static;

// #[macro_use(c)]
// extern crate cute;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate gettext;
use gettext::Catalog;

extern crate rusttype;


use std::convert::TryInto;
use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use std::sync::atomic::{
    AtomicBool,
    Ordering,
};
use std::sync::RwLock;

extern crate oxidize;

extern crate hyphenation;
use hyphenation::{Language, Load, Standard};

extern crate textwrap;
use textwrap::Wrapper;

lazy_static! {
    static ref WACOM_IN_RANGE: AtomicBool = AtomicBool::new(false);
    static ref FOLDER_PATH: RwLock<String> = RwLock::new("/".to_string());
}

fn on_wacom_input(_app: &mut appctx::ApplicationContext, input: wacom::WacomEvent) {
    match input {
        wacom::WacomEvent::InstrumentChange { pen, state } => {
            match pen {
                // Whether the pen is in range
                wacom::WacomPen::ToolPen => {
                    WACOM_IN_RANGE.store(state, Ordering::Relaxed);
                }
                // Whether the pen is actually making contact
                wacom::WacomPen::Touch => {
                    return;
                }
                _ => unreachable!(),
            }
        }
        _ => {}
    };
}

fn on_touch_handler(_app: &mut appctx::ApplicationContext, _input: multitouch::MultitouchEvent) {
    return;
}

fn on_button_press(app: &mut appctx::ApplicationContext, input: gpio::GPIOEvent) {
    let (btn, new_state) = match input {
        gpio::GPIOEvent::Press { button } => (button, true),
        gpio::GPIOEvent::Unpress { button } => (button, false),
        _ => return,
    };

    // Ignoring the unpressed event
    if !new_state {
        return;
    }

    // Simple but effective accidental button press filtering
    if WACOM_IN_RANGE.load(Ordering::Relaxed) {
        return;
    }

    match btn {
        gpio::PhysicalButton::MIDDLE => {
            println!("Exiting");
            app.clear(btn == gpio::PhysicalButton::MIDDLE);
            std::process::exit(0);
        }
        gpio::PhysicalButton::RIGHT => {
            println!("Reloading view");
            let path = FOLDER_PATH.read().unwrap();
            draw_folder(app.upgrade_ref(), &path.as_str());
        }
        gpio::PhysicalButton::WAKEUP => {
            println!("WAKEUP button(?) pressed(?)");
        }
        _=> return,
    };
}

fn on_file_click(app: &mut appctx::ApplicationContext, element: UIElementHandle){
    println!("Click");
    if let UIElement::Text { ref text, .. } = element.read().inner {
        println!("item: {}", text);
        let mut folderpath = FOLDER_PATH.write().unwrap();
        println!("Old Path: {}", folderpath);
        let path = Path::new(&format!("{0}/{1}", folderpath.clone(), text))
            .canonicalize()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();
        println!("New Path: {}", path);
        *folderpath = path.clone();
        println!("Path updated!");
        draw_folder(app, &path.as_str());
    };
}

fn draw_folder(app: &mut appctx::ApplicationContext, folder_path: &str){
    println!("CWD: {}", folder_path);
    let dir = Path::new(folder_path);
    if !dir.exists() || !dir.is_dir() {
        println!("Invalid folder");
        return;
    }
    let mut data = vec![String::from("."), String::from("..")];
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let filename = entry.file_name()
                    .to_string_lossy()
                    .into_owned();
                data.push(filename);
            }
        }
    }
    let hyphenator = Standard::from_embedded(Language::EnglishUS).unwrap();
    let (width, _ ) = app.get_dimensions();
    let wrapper = Wrapper::with_splitter(width.try_into().unwrap(), hyphenator);
    app.clear(false);
    let app2 = app.upgrade_ref();
    let keys: Vec<&String> = app.ui_elements.keys().collect();
    for key in keys {
        if key.starts_with("item.") {
            app2.remove_element(key.as_str());
        }
    }

    let scale = 50.0;
    let mut y = 10 + scale as i32;
    for path in data {
        for line in wrapper.wrap_iter(path.as_str()) {
            let text = line.into_owned();
            let key = format!("item.{}", text);
            app.add_element(
                key.as_str(),
                UIElementWrapper {
                    position: cgmath::Point2 { x: 10, y: y },
                    refresh: UIConstraintRefresh::Refresh,
                    onclick: Some(on_file_click),
                    inner: UIElement::Text {
                        foreground: color::BLACK,
                        text: text,
                        scale: scale,
                        border_px: 0,
                    },
                    ..Default::default()
                },
            );
            y += 60;
        }
    }
    app.draw_elements();
}

fn main(){
    env_logger::init();
    let filepath;
    match env::var_os("LC_ALL") {
        Some(val) => {
            filepath = format!("{}.mo", val.into_string().unwrap());
        }
        None => {
            filepath = "en.mo".to_string();
        }
    }
    if Path::new(&filepath).exists() {
        let file = File::open(filepath).unwrap();
        let _catalog = Catalog::parse(file).unwrap();
    } else {
        let _catalog = Catalog::empty();
    }
    let mut app: appctx::ApplicationContext =
        appctx::ApplicationContext::new(
            on_button_press, on_wacom_input, on_touch_handler);
    let framebuffer = app.get_framebuffer_ref();
    if Path::new("font.ttf").exists() {
        let mut fontfile = File::open("font.ttf").unwrap();
        let mut font_data = vec![];
        fontfile.read_to_end(&mut font_data).expect("Failed to read font file");
        let collection = rusttype::FontCollection::from_bytes(font_data);
        framebuffer.default_font = collection.into_font().unwrap();
    }
    app.clear(true);
    let path = FOLDER_PATH.read().unwrap();
    draw_folder(app.upgrade_ref(), &path.as_str());
    info!("Init complete. Beginning event dispatch...");
    app.dispatch_events(true, true, true);
}