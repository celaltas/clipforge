use gpui::{prelude::FluentBuilder, *};
use gpui_component::{
    ActiveTheme, Icon, IconName, Sizable, button::*, h_flex, input::*, scroll::ScrollableElement,
    v_flex,
};

use crate::{app::state::AppState, convert_to_ui_item};

pub struct ClipboardWorkspace {
    pub search_input: Entity<InputState>,
    pub app_state: Entity<AppState>,
    pub selected_index: Option<usize>,
}

#[derive(Clone)]
pub struct ClipboardItemView {
    pub id: i64,
    pub content_preview: String,
    pub timestamp: String,
    pub item_type: String,
    pub full_content: String,
}

impl ClipboardWorkspace {
    pub fn new(window: &mut Window, cx: &mut Context<Self>, app_state: Entity<AppState>) -> Self {
        let search_input =
            cx.new(|cx| InputState::new(window, cx).placeholder("Search clipboard history..."));

        Self {
            search_input,
            app_state,
            selected_index: None,
        }
    }
}

impl Render for ClipboardWorkspace {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entries = self.app_state.read(cx).get_items();
        let ui_items: Vec<ClipboardItemView> = entries
            .iter()
            .map(|e| convert_to_ui_item(e.clone()))
            .collect();

        v_flex()
            .size_full()
            .bg(cx.theme().background)
            .child(
                h_flex()
                    .h_11()
                    .px_4()
                    .gap_3()
                    .items_center()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .child(
                        div().w_full().max_w_96().child(
                            Input::new(&self.search_input)
                                .prefix(Icon::new(IconName::Search).small())
                                .suffix(Button::new("btn").ghost().icon(IconName::Info).xsmall()),
                        ),
                    )
                    .child(h_flex().gap_1().children(vec![
                        Button::new("all").label("All").small().ghost(),
                        Button::new("text").label("Text").small().ghost(),
                        Button::new("image").label("Image").small().ghost(),
                        Button::new("file").label("File").small().ghost(),
                    ]))
                    .child(div().flex_1())
                    .child(
                        Button::new("settings")
                            .icon(IconName::Settings)
                            .ghost()
                            .on_click(|_, _, _| println!("Settings clicked")),
                    ),
            )
            .child(
                v_flex()
                    .id("clipboard-list")
                    .flex_1()
                    .overflow_y_scrollbar() // gpui_component stili
                    .p_2()
                    .gap_1()
                    .children(ui_items.iter().enumerate().map(|(idx, item)| {
                        let is_selected = self.selected_index == Some(idx);

                        h_flex()
                            .id(item.id.to_string())
                            .px_4()
                            .py_3()
                            .gap_3()
                            .rounded_md()
                            .when(is_selected, |this| this.bg(cx.theme().accent_foreground))
                            .when(!is_selected, |this| {
                                this.hover(|s| s.bg(cx.theme().info_hover))
                            })
                            .child(
                                div()
                                    .size_8()
                                    .rounded_full()
                                    .flex_shrink_0()
                                    .items_center()
                                    .justify_center()
                                    .text_lg()
                                    .bg(match item.item_type.as_str() {
                                        "image" => gpui::rgb(0xf59e0b),
                                        "file" => gpui::rgb(0x8b5cf6),
                                        _ => gpui::rgb(0x3b82f6),
                                    })
                                    .child(match item.item_type.as_str() {
                                        "image" => "🖼",
                                        "file" => "📎",
                                        _ => "📋",
                                    }),
                            )
                            .child(
                                v_flex()
                                    .flex_1()
                                    .gap_0p5()
                                    .child(
                                        div()
                                            .text_sm()
                                            .line_clamp(2)
                                            .child(item.content_preview.clone()),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(cx.theme().muted)
                                            .child(item.timestamp.clone()),
                                    ),
                            )
                            .on_click(cx.listener(move |this, _, _, _| {
                                this.selected_index = Some(idx);
                            }))
                    })),
            )
            .child(
                h_flex()
                    .h_9()
                    .px_4()
                    .items_center()
                    .justify_between()
                    .text_sm()
                    .text_color(cx.theme().muted)
                    .border_t_1()
                    .border_color(cx.theme().border)
                    .child(format!("{} items", ui_items.len()))
                    .child("Ready • Press ↑↓ to navigate"),
            )
    }
}
