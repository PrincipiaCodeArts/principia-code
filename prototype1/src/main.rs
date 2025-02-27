use cursive::align::HAlign;
use cursive::event::{EventResult, Key};
use cursive::traits::*;
use cursive::view::scroll::Scroller;
use cursive::views::{OnEventView, Panel, TextArea};
use cursive::{
    views::{Dialog, EditView, LinearLayout, RadioGroup, SelectView, TextView},
    Cursive,
};
use resources::{MainResource, ReturnType, Signature};

mod resources;

//fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::text("Welcome to Alien Code")
            .title("Alien Code")
            .button("Run", |s| show_answer(s, "running a resource"))
            .button("Create", |s| show_create_options(s))
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

enum ResourceOptions {
    Main,
    Function,
    Constant,
}

// Show resource type selection dialog
pub fn show_create_options(siv: &mut Cursive) {
    let mut select = SelectView::new()
        .h_align(cursive::align::HAlign::Left)
        .with_all(vec![
            ("Main Function", ResourceOptions::Main),
            ("Function", ResourceOptions::Function),
            ("Constant", ResourceOptions::Constant),
        ]);

    select.set_on_submit(|s, item| match item {
        ResourceOptions::Main => show_main_creation_form(s),
        ResourceOptions::Function => show_answer(s, "Function creation not implemented yet"),
        ResourceOptions::Constant => show_answer(s, "Constant creation not implemented yet"),
    });

    siv.add_layer(
        Dialog::around(select)
            .title("Select Resource Type")
            .button("Cancel", |s| {
                let _ = s.pop_layer();
            }),
    );
}
fn show_main_creation_form(siv: &mut Cursive) {
    let mut return_type = RadioGroup::new();

    let layout = LinearLayout::vertical()
        .child(TextView::new("Name:"))
        .child(EditView::new().with_name("name"))
        .child(TextView::new("Author:"))
        .child(EditView::new().with_name("author"))
        .child(TextView::new("Description:"))
        .child(EditView::new().with_name("description"))
        .child(TextView::new("Return Type:"))
        .child(return_type.button(ReturnType::Void, "void"))
        .child(return_type.button(ReturnType::Int, "int"));

    siv.add_layer(
        Dialog::around(layout)
            .title("Create Main Resource")
            .button("Create", move |s| {
                create_main_resource(s, &return_type);
            })
            .button("Cancel", |s| {
                let _ = s.pop_layer();
            }),
    );
}

fn create_main_resource(siv: &mut Cursive, return_type: &RadioGroup<ReturnType>) {
    let name = siv
        .call_on_name("name", |view: &mut EditView| view.get_content())
        .expect("missing name");

    let author = siv
        .call_on_name("author", |view: &mut EditView| view.get_content())
        .expect("missing author");

    let description = siv
        .call_on_name("description", |view: &mut EditView| view.get_content())
        .unwrap_or_default();

    let signature = Signature {
        return_type: *return_type.selection(),
    };

    // Validate inputs
    if name.is_empty() || author.is_empty() {
        show_error(siv, "Name and Author are required fields");
        return;
    }

    // Create the resource using the builder pattern
    let result = MainResource::new(
        name.to_string(),
        author.to_string(),
        description.to_string(),
        vec![],
        signature,
    );
    // Show the code editor for the main content
    // Add the cursive syntect to highlight the rust code.
    siv.add_fullscreen_layer(
        Dialog::around(Panel::new(
            TextArea::new()
                .with_name("source_code")
                .full_screen()
                .scrollable()
                .wrap_with(OnEventView::new)
                .on_pre_event_inner(Key::PageUp, |v, _| {
                    let scroller = v.get_scroller_mut();
                    if scroller.can_scroll_up() {
                        scroller.scroll_up(scroller.last_outer_size().y.saturating_sub(1));
                    }
                    Some(EventResult::Consumed(None))
                })
                .on_pre_event_inner(Key::PageDown, |v, _| {
                    let scroller = v.get_scroller_mut();
                    if scroller.can_scroll_down() {
                        scroller.scroll_down(scroller.last_outer_size().y.saturating_sub(1));
                    }
                    Some(EventResult::Consumed(None))
                }),
            /*
               TextView::new("This is a huge content, my friend!")
                   .scrollable()
                   .wrap_with(OnEventView::new)
                   .on_pre_event_inner(Key::PageUp, |v, _| {
                       let scroller = v.get_scroller_mut();
                       if scroller.can_scroll_up() {
                           scroller.scroll_up(scroller.last_outer_size().y.saturating_sub(1));
                       }
                       Some(EventResult::Consumed(None))
                   })
                   .on_pre_event_inner(Key::PageDown, |v, _| {
                       let scroller = v.get_scroller_mut();
                       if scroller.can_scroll_down() {
                           scroller.scroll_down(scroller.last_outer_size().y.saturating_sub(1));
                       }
                       Some(EventResult::Consumed(None))
                   }),
            */
        ))
        .title("Main content")
        // This is the alignment for the button
        .h_align(HAlign::Center)
        .button("Quit", |s| s.quit()),
    );

    //show_success(siv, &format!("Created main resource: {}", result.name()));
}

fn show_error(siv: &mut Cursive, message: &str) {
    siv.add_layer(Dialog::text(message).title("Error").button("OK", |s| {
        let _ = s.pop_layer();
    }));
}

fn show_success(siv: &mut Cursive, message: &str) {
    siv.add_layer(Dialog::text(message).title("Success").button("OK", |s| {
        // Pop both the success dialog and the form
        let _ = s.pop_layer();
        let _ = s.pop_layer();
    }));
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

fn main() {
    // The source code goes below (you can only edit between horizontal lines)
    //-----------------------------------
    let hello_world = "Hello, World!";
    println!("{hello_world}");
    //-----------------------------------
}
