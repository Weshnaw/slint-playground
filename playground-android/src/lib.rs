#[cfg(target_os = "android")]
mod android {
    use playground_shared::increase_value;

    slint::include_modules!();
    pub fn window() {
        let ui = AppWindow::new().unwrap();
        ui.on_request_increase_value({
            let ui_handle = ui.as_weak();
            move || {
                let ui = ui_handle.unwrap();
                ui.set_counter(increase_value(ui.get_counter()));
            }
        });
        ui.run().unwrap();
    }
}

#[no_mangle]
#[cfg(target_os = "android")]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    android::window();
}
