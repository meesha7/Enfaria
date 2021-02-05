use druid::widget::*;
use druid::*;

fn main() -> Result<(), PlatformError> {
    dotenv::dotenv().ok();
    let main_window = WindowDesc::new(ui_builder)
        .title("Enfaria Updater")
        .window_size((500.0, 500.0));
    AppLauncher::with_window(main_window).use_simple_logger().launch(0.2)
}

fn ui_builder() -> Flex<f64> {
    let client_version = get_client_version();
    let label_one = Label::new(format!("Client Version: {}", client_version)).center();

    let server_version = get_server_version();
    let label_two = Label::new(format!("Server Version: {}", server_version)).center();

    let bar = get_bar();
    let button: Button<f64> = Button::new("Button");

    let row = Flex::row().with_flex_child(bar, 0.8).with_flex_child(button, 0.2);

    Flex::column()
        .with_flex_spacer(0.8)
        .with_flex_child(label_one, 0.05)
        .with_flex_child(label_two, 0.05)
        .with_flex_child(row, 0.1)
}

fn get_bar() -> impl Widget<f64> {
    ProgressBar::new().expand_width().padding(Insets::uniform_xy(7., 7.))
}

fn get_server_version() -> String {
    let domain = std::env::var("DOMAIN").unwrap();
    ureq::get(&format!("{}/api/version", domain))
        .call()
        .unwrap()
        .into_string()
        .unwrap()
}

fn get_client_version() -> String {
    #[cfg(target_os = "windows")]
    let path = "enfaria-game.exe";

    #[cfg(target_os = "linux")]
    let path = "enfaria-game";

    let output = std::process::Command::new(path).arg("--v").output().unwrap().stdout;

    let string = String::from_utf8(output).unwrap();

    string.lines().last().unwrap().to_owned()
}
