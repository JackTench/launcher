slint::include_modules!();

fn main() {
    let window = AppWindow::new().unwrap();

    window.set_number_of_buttons(5);

    window.run().unwrap();
}
