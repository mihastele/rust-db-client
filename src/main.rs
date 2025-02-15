use druid::widget::{Align, Flex, Label, TextBox};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WindowDesc, WidgetExt};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<DBConnectionState> = LocalizedString::new("Rust DB client!");

#[derive(Clone, Data, Lens)]
struct DBConnectionState {
    connection_string: String,
    username: String,
    password: String,
}

fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state = DBConnectionState {
        connection_string: "localhost".into(),
        username: "".into(),
        password: "".into(),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<DBConnectionState> {
    // a label that will determine its text based on the current app data.
    let label = Label::new(|_data: &DBConnectionState, _env: &Env| "Welcome to Rust DB client!");
    // a textbox that modifies `name`.
    let connection_string = TextBox::new()
        .with_placeholder("Connection string")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(DBConnectionState::connection_string);

    let username_field = TextBox::new()
        .with_placeholder("Username")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(DBConnectionState::username);

    let password_field = TextBox::new()
        .with_placeholder("Password")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(DBConnectionState::password);

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(connection_string)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(username_field)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(password_field);

    // center the two widgets in the available space
    Align::centered(layout)
}