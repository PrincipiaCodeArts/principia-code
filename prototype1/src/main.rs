use cursive::{views::Dialog, Cursive};

mod resources;

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::text("Welcome to SE-CAD")
            .title("Software Engineering CAD")
            .button("Run", |s| show_answer(s, "running a resource"))
            .button("Create", |s| show_answer(s, "Creating a resource"))
            .button("Manage", |s| show_answer(s, "Managing a resource"))
            .button("Exit", |s| prompt_exit(s)),
    );

    // Starts the event loop.
    siv.run();
}

fn prompt_exit(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::text("Are you sure?")
            .button("Yes", |s| s.quit())
            .button("No", |s| {
                let _ = s.pop_layer();
            })
            .title("Exit"),
    );
}

fn show_answer(siv: &mut Cursive, msg: &str) {
    siv.add_layer(
        Dialog::text(msg)
            .button("OK", |s| {
                let _ = s.pop_layer();
            })
            .title("Action"),
    );
}
