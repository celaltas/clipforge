use gpui::{prelude::FluentBuilder, *};
use gpui_component::{
    ActiveTheme, Icon, IconName, Sizable, button::*, h_flex, input::*, scroll::ScrollableElement,
    v_flex,
};

use crate::{
    CopySelected, DeleteSelected, SelectNext, SelectPrevious, TogglePinSelected,
    app::{event::UiAction, state::AppState},
    ui::{ClipboardItemView, ItemType},
};

pub struct ClipboardWorkspace {
    pub focus_handle: FocusHandle,
    pub search_input: Entity<InputState>,
    pub app_state: Entity<AppState>,
    pub selected_index: Option<usize>,
    pub action_sender: flume::Sender<UiAction>,
}

impl ClipboardWorkspace {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        app_state: Entity<AppState>,
        action_sender: flume::Sender<UiAction>,
    ) -> Self {
        let search_input =
            cx.new(|cx| InputState::new(window, cx).placeholder("Search clipboard history..."));

        let sender_clone = action_sender.clone();
        cx.observe(&search_input, move |this, state, cx| {
            let text = state.read(cx).value().to_string();
            let _ = sender_clone.send(UiAction::Search(text));
        })
        .detach();

        Self {
            focus_handle: cx.focus_handle(),
            search_input,
            app_state,
            selected_index: Some(0),
            action_sender,
        }
    }

    fn select_next(&mut self, _: &SelectNext, _: &mut Window, cx: &mut Context<Self>) {
        let len = self.app_state.read(cx).get_items().len();
        if len == 0 {
            return;
        }

        let current = self.selected_index.unwrap_or(0);
        self.selected_index = Some((current + 1).min(len - 1));

        cx.notify();
    }

    fn select_previous(&mut self, _: &SelectPrevious, _: &mut Window, cx: &mut Context<Self>) {
        let current = self.selected_index.unwrap_or(0);
        self.selected_index = Some(current.saturating_sub(1));
        cx.notify();
    }

    fn copy_selected(&mut self, _: &CopySelected, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(idx) = self.selected_index {
            let items = self.app_state.read(cx).get_items();

            if let Some(item) = items.get(idx) {
                println!("COPY: {}", item.id);
            }
        }
    }
    fn delete_selected(&mut self, _: &DeleteSelected, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(idx) = self.selected_index {
            let items = self.app_state.read(cx).get_items();

            if let Some(item) = items.get(idx) {
                let _ = self.action_sender.send(UiAction::Delete(item.id));
                if idx >= items.len().saturating_sub(1) && idx > 0 {
                    self.selected_index = Some(idx - 1);
                }
            }
        }
    }
    fn toggle_pin_selected(
        &mut self,
        _: &TogglePinSelected,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(idx) = self.selected_index {
            let items = self.app_state.read(cx).get_items();

            if let Some(item) = items.get(idx) {
                let _ = self
                    .action_sender
                    .send(UiAction::TogglePin(item.id, !item.pinned));
            }
        }
    }
}

impl Render for ClipboardWorkspace {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entries = self.app_state.read(cx).get_items();

        let ui_items: Vec<ClipboardItemView> =
            entries.iter().map(ClipboardItemView::from).collect();

        v_flex()
            .key_context("ClipboardWorkspace")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::copy_selected))
            .on_action(cx.listener(Self::delete_selected))
            .on_action(cx.listener(Self::toggle_pin_selected))
            .size_full()
            .bg(cx.theme().background)
            .child(
                h_flex()
                    .h_11()
                    .px_4()
                    .gap_3()
                    .items_center()
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
                            .when(is_selected, |this| {
                                this.bg(cx.theme().info_hover)
                                    .border_1()
                                    .border_color(cx.theme().blue)
                            })
                            .when(!is_selected, |this| {
                                this.when(item.pinned, |this| this.bg(cx.theme().info_hover)) // Pinli olanlara hafif arka plan tonu
                                            .hover(|s| s.bg(cx.theme().info_hover))
                            })
                            .child(
                                div()
                                    .size_8()
                                    .rounded_full()
                                    .flex_shrink_0()
                                    .items_center()
                                    .justify_center()
                                    .text_lg()
                                    .bg(match item.item_type {
                                        ItemType::Image => gpui::rgb(0xf59e0b),
                                        ItemType::File => gpui::rgb(0x8b5cf6),
                                        ItemType::Link => gpui::rgb(0x10b981),
                                        ItemType::Text => gpui::rgb(0x3b82f6),
                                    })
                                    .child(match item.item_type {
                                        ItemType::Image => "🖼",
                                        ItemType::File => "📎",
                                        ItemType::Link => "🔗",
                                        ItemType::Text => "📋",
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
                                    .when(item.pinned, |this| {
                                        this.child(
                                            Icon::new(IconName::Ellipsis)
                                                .small()
                                                .text_color(cx.theme().blue),
                                        )
                                    }),
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
                    .text_color(cx.theme().blue)
                    .border_t_1()
                    .border_color(cx.theme().border)
                    .child(format!("{} items", ui_items.len()))
                    .child(
                        h_flex()
                            .gap_4()
                            .child("↵ Copy")
                            .child("⌘P Pin")
                            .child("⌫ Delete")
                            .child("↑↓ Navigate"),
                    ),
            )
    }
}
