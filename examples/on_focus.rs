use iced::{
    widget::{column, container, text},
    Element, Sandbox, Settings,
};
use iced_on_focus_text_input::hoverable;

fn main() {
    App::run(Settings::default()).unwrap()
}

struct App {
    status: String,
}

#[derive(Debug, Clone)]
enum Messages {
    OnFocus,
    OnUnfocus,
}

impl Sandbox for App {
    type Message = Messages;

    fn new() -> Self {
        App {
            status: String::from(""),
        }
    }

    fn title(&self) -> String {
        String::from("Simple On Focus Example")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Messages::OnFocus => self.status = "Focused".into(),
            Messages::OnUnfocus => self.status = "Unfocused".into(),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let widget = text("Some text here");
        let hoverable_widget = hoverable(widget.into())
            .on_focus(Messages::OnFocus)
            .on_unfocus(Messages::OnUnfocus);

        let status = text(self.status.clone());

        column!(hoverable_widget, status).into()
    }
}
