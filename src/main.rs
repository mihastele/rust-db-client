use druid::widget::{Align, Flex, Label, TextBox, Controller, RadioGroup, Radio, LineBreaking};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WindowDesc, WidgetExt, UpdateCtx};


const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<DBConnectionState> = LocalizedString::new("Rust DB client!");

#[derive(Clone, Data, Lens)]
struct DBConnectionState {
    connection_string: String,
    username: String,
    password: String,
    db_type: DbType,
}

// Define the possible database types as an enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Data)]
enum DbType {
    Postgres,
    MySql,
    MSSQL
}

// impl Data for DbType {
//     fn same(&self, other: &Self) -> bool {
//         std::mem::discriminant(self) == std::mem::discriminant(other)
//     }
// }
//
// impl PartialEq for DbType {
//     fn eq(&self, other: &Self) -> bool {
//         self.same(other)
//     }
// }

struct PasswordController;

impl<W: Widget<DBConnectionState>> Controller<DBConnectionState, W> for PasswordController {
    fn update(&mut self, child: &mut W, ctx: &mut UpdateCtx, old_data: &DBConnectionState, data: &DBConnectionState, env: &Env) {
        if old_data.password != data.password {
            // Replace the displayed text with asterisks
            let masked_text = "*".repeat(data.password.len());
            ctx.request_paint();
            let mut temp_data = data.clone();
            temp_data.password = masked_text;
            child.update(ctx, old_data, &temp_data, env);
        } else {
            child.update(ctx, old_data, data, env);
        }
    }
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
        db_type: DbType::MySql,
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

    // Create radio buttons for database selection
    let radio_group = RadioGroup::column(vec![
        ("MySQL", DbType::MySql),
        ("Postgres", DbType::Postgres),
        ("Microsoft SQL", DbType::MSSQL),
    ]).lens(DBConnectionState::db_type);



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
        .lens(DBConnectionState::password)
        .controller(PasswordController);

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(radio_group)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(connection_string)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(username_field)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(password_field);

    // center the two widgets in the available space
    Align::centered(layout)
}