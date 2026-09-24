use freya::{
    material_design::*,
    prelude::*,
};

use crate::showcases::{
    DemoGrid,
    inline_menu,
};

demos! {
    ButtonDemo => "Button" {
        Button::new().ripple().child("Press me")
    }

    FilledButtonDemo => "Filled Button" {
        Button::new().filled().ripple().child("Filled")
    }

    OutlineButtonDemo => "Outline Button" {
        Button::new().outline().ripple().child("Outline")
    }

    FlatButtonDemo => "Flat Button" {
        Button::new().flat().ripple().child("Flat")
    }

    CheckboxTileDemo => "Tile with Checkbox" {
        let mut toppings = use_state(|| vec!["Cheese"]);

        rect()
            .width(Size::px(220.))
            .children(["Cheese", "Olives", "Basil"].map(|topping| {
                let is_checked = toppings.read().contains(&topping);
                Tile::new()
                    .on_select(move |_| {
                        if is_checked {
                            toppings.write().retain(|item| *item != topping);
                        } else {
                            toppings.write().push(topping);
                        }
                    })
                    .ripple()
                    .leading(Checkbox::new().selected(is_checked))
                    .child(label().text(topping).width(Size::fill()))
            }))
    }

    RadioTileDemo => "Tile with RadioItem" {
        let mut density = use_state(|| 1usize);

        rect()
            .width(Size::px(220.))
            .children(
                ["Compact", "Cozy", "Comfortable"]
                    .into_iter()
                    .enumerate()
                    .map(|(index, name)| {
                        Tile::new()
                            .on_select(move |_| density.set(index))
                            .ripple()
                            .leading(RadioItem::new().selected(density() == index))
                            .child(label().text(name).width(Size::fill()))
                    }),
            )
    }

    FloatingTabDemo => "FloatingTab" {
        rect()
            .spacing(8.)
            .children(["Overview", "Details", "Activity"].map(|name| {
                FloatingTab::new().ripple().child(name)
            }))
    }

    SideBarItemDemo => "SideBarItem" {
        rect()
            .width(Size::px(200.))
            .spacing(4.)
            .child(SideBarItem::new().ripple().child("Inbox"))
            .child(SideBarItem::new().ripple().child("Drafts"))
            .child(SideBarItem::new().ripple().child("Sent"))
    }

    MenuItemDemo => "MenuItem" {
        let mut selected = use_state(|| 0usize);

        inline_menu()
            .children(["Open", "Save", "Export"].into_iter().enumerate().map(
                |(index, name)| {
                    MenuItem::new()
                        .selected(selected() == index)
                        .on_press(move |_| selected.set(index))
                        .ripple()
                        .child(name)
                },
            ))
    }

    RippleDemo => "Ripple" {
        Ripple::new().child(
            Card::new()
                .width(Size::px(220.))
                .height(Size::px(120.))
                .child("Press anywhere on this card"),
        )
    }
}

#[derive(PartialEq)]
pub struct MaterialShowcase;

impl Component for MaterialShowcase {
    fn render(&self) -> impl IntoElement {
        DemoGrid {
            title: "Material Design",
            subtitle: "Ripples under your cursor",
            demos: DEMOS,
        }
    }
}
