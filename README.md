# Iced Widget focus subscription
### Subscribe to focus changes on widgets

``` Rust
let widget = text("Some text here");
let hoverable_widget = hoverable(widget.into())
   .on_focus(Messages::OnFocus)
   .on_unfocus(Messages::OnUnfocus);
```
