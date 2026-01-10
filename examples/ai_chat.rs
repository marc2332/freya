#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{env, rc::Rc};

use euclid::Length;
use freya::prelude::*;
use rig::{completion::Prompt, providers::openai::{self, Client}};
use tokio::runtime::Builder;

#[derive(Clone, Debug)]
struct Message {
    role: String,
    content: String,
}

fn main() {
    let rt = Builder::new_multi_thread().enable_all().build().unwrap();
    let _rt = rt.enter();
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(800., 600.)))
}

fn app() -> impl IntoElement {
    let mut messages = use_state(|| {
        vec![Message {
            role: "assistant".to_string(),
            content: "Hello! I'm a mock AI chat. Type a message and press Send to see a response."
                .to_string(),
        }]
    });
    let mut input_value = use_state(|| String::new());

    let send_message = move |_| {
        let user_message = input_value.read().clone();
        if user_message.trim().is_empty() {
            return;
        }

        // Add user message
        messages.write().push(Message {
            role: "user".to_string(),
            content: user_message.clone(),
        });

        // Clear input
        *input_value.write() = String::new();

        // Add AI response using rig-core
        let user_msg = user_message.clone();
        spawn(async move {
            let client = openai::Client::from_env();
            let agent = client.agent("gpt-4.1").build();
            match agent.prompt(&user_msg).await {
                Ok(response) => {
                    messages.write().push(Message {
                        role: "assistant".to_string(),
                        content: response,
                    });
                }
                Err(e) => {
                    messages.write().push(Message {
                        role: "assistant".to_string(),
                        content: format!("Error: {}", e),
                    });
                }
            }
        });
    };

    let chat_area = rect()
        .width(Size::fill())
        .height(Size::fill())
        .background((30, 30, 30))
        .child(
            ScrollView::new().child(rect().width(Size::fill()).padding(16.).children_iter(
                messages.read().iter().map(|msg| {
                    let is_user = msg.role == "user";
                    let bg_color = if is_user {
                        (59, 130, 246)
                    } else {
                        (55, 65, 81)
                    };
                    let align = if is_user {
                        Alignment::End
                    } else {
                        Alignment::Start
                    };
                    let text_align = if is_user {
                        TextAlign::End
                    } else {
                        TextAlign::Start
                    };

                    rect()
                        .width(Size::fill())
                        .margin(8.)
                        .cross_align(align)
                        .child(
                            rect()
                                .padding(12.)
                                .background(bg_color)
                                .corner_radius(8.)
                                .color((255, 255, 255))
                                .text_align(text_align)
                                .child(msg.content.clone()),
                        )
                        .into()
                }),
            )),
        );

    let input_area = rect()
        .width(Size::fill())
        .height(Size::Pixels(Length::new(60.)))
        .background((40, 40, 40))
        .padding(10.)
        .position(Position::new_absolute().bottom(0.).left(0.).right(0.))
        .child(
            rect()
                .horizontal()
                .width(Size::fill())
                .height(Size::fill())
                .spacing(8.)
                .content(Content::Flex)
                .child(
                    Input::new()
                        .value(input_value.read().clone())
                        .on_change(move |value| {
                            *input_value.write() = value;
                        })
                        .placeholder("Type your message...")
                        .width(Size::Flex(Length::new(1.))),
                )
                .child(Button::new().on_press(send_message).child("Send")),
        );

    rect()
        .width(Size::fill())
        .height(Size::fill())
        .background((50, 50, 50))
        .child(chat_area)
        .child(input_area)
}
