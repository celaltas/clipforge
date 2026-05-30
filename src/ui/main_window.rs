use gpui::*;
use gpui_component::StyledExt;
use crate::app::state::SharedAppState;

pub struct ClipboardList {
    state: SharedAppState,
}

impl ClipboardList {
    pub fn new(state: SharedAppState) -> Self {
        Self { state }
    }
}

impl Render for ClipboardList {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let items = self.state
            .clipboard_items
            .lock()
            .unwrap();

        println!("render called");

        div()
            .size_full()
            .v_flex()
            .gap_2()
            .p_4()
            .children(
                items.iter().rev().map(|item| {
                    div()
                        .w_full()
                        .p_3()
                        .rounded_md()
                        .border_1()
                        .border_color(rgb(0x333333))
                        .child(item.content.clone())
                })
            )
    }
}