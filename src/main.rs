use eframe::{egui, epaint::Color32};

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        DeadPixelCheck::APP_NAME,
        DeadPixelCheck::native_options(),
        DeadPixelCheck::app_creator(),
    )?;
    return Ok(());
}

pub struct DeadPixelCheck {
    color: Color32,
}
impl DeadPixelCheck {
    pub const APP_NAME: &'static str = "Dead Pixel Check";
    pub fn native_options() -> eframe::NativeOptions {
        return eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_fullscreen(true)
                .with_resizable(false)
                .with_active(true),
            ..Default::default()
        };
    }
    pub fn app_creator() -> eframe::AppCreator {
        return Box::new(|_cc| {
            Box::new(Self {
                color: Color32::WHITE,
            })
        });
    }
    fn change_color(&mut self) {
        self.color = match self.color {
            Color32::WHITE => Color32::BLACK,
            Color32::BLACK => Color32::RED,
            Color32::RED => Color32::GREEN,
            Color32::GREEN => Color32::BLUE,
            _ => Color32::WHITE,
        };
    }
}
impl eframe::App for DeadPixelCheck {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        use egui::{CentralPanel, Key, Frame};

        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            std::process::exit(0);
        }

        if ctx.input(|i| i.key_pressed(Key::Space) || i.pointer.any_click() || i.any_touches()) {
            self.change_color();
        }

        CentralPanel::default()
            .frame(Frame {
                fill: self.color,
                ..Default::default()
            })
            .show(ctx, |_| {});
    }
}
