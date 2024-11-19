use cursive::{views::Dialog, Cursive};

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::text("This is a survey\nPress <next> when you are ready")
            .title("Hello World Survey")
            .button("Next", show_next),
    );

    // Starts the event loop.
    siv.run();
}

fn show_next(siv: &mut Cursive) {
    siv.pop_layer();
    siv.add_layer(
        Dialog::text("You have pressed <next>, congratulations!")
            .title("Questão 1")
            .button("Sim", |s| show_answer(s, "Resposta correta, parabéns!"))
            .button("Não", |s| show_answer(s))
            .button("O quê?", |s| s.add_layer(Dialog::info("Tente novamente!"))),
    );
}

fn show_answer(siv: &mut Cursive, msg: &str) {
    siv.pop_layer();
    siv.add_layer(
        Dialog::text(msg)
            .button("Finish", |s| s.quit())
            .title("Results"),
    );
}
