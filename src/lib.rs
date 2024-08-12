use iced::{
    advanced::{
        self,
        layout::{Layout, Limits, Node},
        renderer,
        widget::tree,
        widget::Tree,
        Clipboard, Shell, Widget,
    },
    event::Status,
    mouse::Cursor,
    Element, Event, Length, Rectangle, Renderer, Size, Theme,
};

/// Create new hoverable widget
pub fn hoverable<'a, Message>(widget: Element<'a, Message, Theme, Renderer>) -> Hoverable<Message> {
    Hoverable::new(widget)
}

/// Hoverable widget to allow .on_focus() and .on_unfocus() subscriptions
pub struct Hoverable<'a, Message> {
    widget: Element<'a, Message, Theme, Renderer>,
    on_focus_changed: Option<Message>,
    on_focus: Option<Message>,
    on_unfocus: Option<Message>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct State {
    is_focused: bool,
}

impl<'a, Message> Hoverable<'a, Message> {
    /// Construct a new Hoverable widget
    pub fn new(widget: Element<'a, Message, Theme, Renderer>) -> Self {
        Self {
            widget,
            on_focus_changed: None,
            on_focus: None,
            on_unfocus: None,
        }
    }

    /// Add a message to be produced when widget focus is changed
    pub fn on_focus_change(mut self, message: Message) -> Self {
        self.on_focus_changed = Some(message);
        self
    }

    /// Add a message to be produced when widget is focused
    pub fn on_focus(mut self, message: Message) -> Self {
        self.on_focus = Some(message);
        self
    }

    /// Add a message to be produced when widget is unfocused
    pub fn on_unfocus(mut self, message: Message) -> Self {
        self.on_unfocus = Some(message);
        self
    }
}

impl<Message: Clone> Widget<Message, Theme, Renderer> for Hoverable<'_, Message> {
    fn size(&self) -> Size<Length> {
        Size::new(Length::Shrink, Length::Shrink)
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(self.widget.as_widget())]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.widget));
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let child_node = self
            .widget
            .as_widget()
            .layout(&mut tree.children[0], renderer, limits);

        Node::with_children(child_node.size(), vec![child_node])
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        self.widget.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            viewport,
        );
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> Status {
        if let Status::Captured = self.widget.as_widget_mut().on_event(
            &mut state.children[0],
            event,
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        ) {
            return Status::Captured;
        }

        let state = state.state.downcast_mut::<State>();

        match cursor.position_in(layout.bounds()) {
            // cursor is in widget
            Some(_) => {
                if let Some(on_change) = &self.on_focus_changed {
                    if !state.is_focused {
                        shell.publish(on_change.clone());
                    }
                };
                if let Some(on_focus) = &self.on_focus {
                    shell.publish(on_focus.clone());
                };
                state.is_focused = true;
            }
            // cursor left widget
            None => {
                if let Some(on_change) = &self.on_focus_changed {
                    if state.is_focused {
                        shell.publish(on_change.clone());
                    }
                };
                if let Some(on_unfocus) = &self.on_unfocus {
                    shell.publish(on_unfocus.clone());
                }
                state.is_focused = false;
            }
        }
        Status::Ignored
    }
}

impl<'a, Message: 'a, Renderer> From<Hoverable<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
where
    Hoverable<'a, Message>: Widget<Message, Theme, Renderer>,
    Renderer: advanced::Renderer,
{
    fn from(widget: Hoverable<'a, Message>) -> Self {
        Self::new(widget)
    }
}
