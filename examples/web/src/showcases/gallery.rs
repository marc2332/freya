use freya::{
    animation::*,
    icons::lucide,
    prelude::*,
};

use crate::showcases::{
    DemoGrid,
    inline_menu,
};

const RUST_LOGO: &[u8] = include_bytes!("../rust_logo.png");
const PREVIEW_URL: &str =
    "https://raw.githubusercontent.com/marc2332/freya/main/website/public/preview.png";
const FROG_URL: &str =
    "https://raw.githubusercontent.com/marc2332/freya/main/examples/frog_typing.gif";

demos! {
    ButtonDemo => "Button" {
        let mut presses = use_state(|| 0);

        button_variants(move || Button::new().on_press(move |_| *presses.write() += 1))
            .child(format!("{} presses", presses()))
    }

    FilledButtonDemo => "Filled Button" {
        button_variants(|| Button::new().filled())
    }

    OutlineButtonDemo => "Outline Button" {
        button_variants(|| Button::new().outline())
    }

    FlatButtonDemo => "Flat Button" {
        button_variants(|| Button::new().flat())
    }

    SwitchDemo => "Switch" {
        let mut toggled = use_state(|| true);

        rect()
            .spacing(12.)
            .cross_align(Alignment::center())
            .child(
                Switch::new()
                    .toggled(toggled())
                    .on_toggle(move |_| toggled.toggle()),
            )
            .child(if toggled() { "On" } else { "Off" })
            .child(Switch::new().toggled(false).enabled(false))
            .child("Disabled")
    }

    CheckboxDemo => "Checkbox" {
        let mut toppings = use_state(|| vec!["Cheese"]);

        rect()
            .width(Size::px(200.))
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
                    .leading(Checkbox::new().selected(is_checked))
                    .child(label().text(topping).width(Size::fill()))
            }))
    }

    RadioItemDemo => "RadioItem" {
        let mut density = use_state(|| 1usize);

        rect()
            .width(Size::px(200.))
            .children(
                ["Compact", "Cozy", "Comfortable"]
                    .into_iter()
                    .enumerate()
                    .map(|(index, name)| {
                        Tile::new()
                            .on_select(move |_| density.set(index))
                            .leading(RadioItem::new().selected(density() == index))
                            .child(label().text(name).width(Size::fill()))
                    }),
            )
    }

    TileDemo => "Tile" {
        let mut selected = use_state(|| false);

        Tile::new()
            .on_select(move |_| selected.toggle())
            .leading(Checkbox::new().selected(selected()))
            .child(label().text("Press the tile").width(Size::px(160.)))
    }

    SliderDemo => "Slider" {
        let mut value = use_state(|| 45.0f64);

        rect()
            .spacing(10.)
            .width(Size::px(220.))
            .cross_align(Alignment::center())
            .child(format!("{}%", value().floor()))
            .child(
                Slider::new(move |new_value| value.set(new_value))
                    .value(value())
                    .size(Size::fill()),
            )
    }

    VerticalSliderDemo => "Vertical Slider" {
        let mut value = use_state(|| 70.0f64);

        Slider::new(move |new_value| value.set(new_value))
            .value(value())
            .direction(Direction::Vertical)
            .size(Size::px(160.))
    }

    ProgressBarDemo => "ProgressBar" {
        let progress = use_animation(|conf| {
            conf.on_creation(OnCreation::Run);
            conf.on_finish(OnFinish::restart());
            AnimNum::new(0., 100.).time(3000).function(Function::Linear)
        });
        let value = progress.get().value();

        rect()
            .spacing(12.)
            .width(Size::px(220.))
            .child(ProgressBar::new(value).width(Size::fill()))
            .child(
                ProgressBar::new(value)
                    .show_progress(false)
                    .width(Size::fill()),
            )
    }

    InputDemo => "Input" {
        input_variants(|| Input::new(use_state(String::new)))
    }

    FilledInputDemo => "Filled Input" {
        input_variants(|| Input::new(use_state(String::new)).filled())
    }

    FlatInputDemo => "Flat Input" {
        input_variants(|| Input::new(use_state(String::new)).flat())
    }

    PasswordInputDemo => "Password Input" {
        let text = use_state(|| "hunter2".to_string());

        Input::new(text)
            .mode(InputMode::new_password())
            .width(Size::px(220.))
    }

    MultilineInputDemo => "Multiline Input" {
        let text = use_state(|| "Write as many\nlines as you want".to_string());

        Input::new(text)
            .multiline(true)
            .width(Size::px(220.))
            .height(Size::px(120.))
    }

    SelectDemo => "Select" {
        let coffees = ["Espresso", "Flat white", "Cortado"];
        let mut selected = use_state(|| 0usize);

        Select::new()
            .selected_item(coffees[selected()])
            .children(coffees.iter().enumerate().map(|(index, name)| {
                MenuItem::new()
                    .selected(selected() == index)
                    .on_press(move |_| selected.set(index))
                    .child(*name)
            }))
    }

    MenuDemo => "Menu" {
        let mut selected = use_state(|| 0usize);

        inline_menu()
            .children(["Open", "Save", "Export"].into_iter().enumerate().map(
                |(index, name)| {
                    MenuItem::new()
                        .selected(selected() == index)
                        .on_press(move |_| selected.set(index))
                        .child(name)
                },
            ))
    }

    AccordionDemo => "Accordion" {
        rect()
            .width(Size::px(240.))
            .spacing(4.)
            .child(
                Accordion::new()
                    .header("What is Freya?")
                    .child("A Rust library to build native GUIs."),
            )
            .child(
                Accordion::new()
                    .header("Where does it run?")
                    .child("Desktop, Android and the web."),
            )
    }

    CardDemo => "Card" {
        Card::new()
            .filled()
            .width(Size::px(200.))
            .height(Size::px(120.))
            .child("A filled card")
    }

    OutlineCardDemo => "Outline Card" {
        Card::new()
            .outline()
            .width(Size::px(200.))
            .height(Size::px(120.))
            .child("An outlined card")
    }

    ChipDemo => "Chip" {
        let mut selected = use_state(|| 0usize);

        rect()
            .horizontal()
            .spacing(8.)
            .children(["Rust", "Skia", "Wasm"].into_iter().enumerate().map(
                |(index, name)| {
                    Chip::new()
                        .selected(selected() == index)
                        .on_press(move |_| selected.set(index))
                        .child(name)
                },
            ))
    }

    SegmentedButtonDemo => "SegmentedButton" {
        let mut selected = use_state(|| 0usize);

        SegmentedButton::new().children((0..3).map(|index| {
            ButtonSegment::new()
                .key(index)
                .selected(selected() == index)
                .on_press(move |_| selected.set(index))
                .child(format!("Tab {}", index + 1))
        }))
    }

    TooltipDemo => "Tooltip" {
        TooltipContainer::new(Tooltip::new_text("Hello there!"))
            .position(AttachedPosition::Top)
            .child(Button::new().child("Hover me"))
    }

    PopupDemo => "Popup" {
        let mut show = use_state(|| false);

        rect()
            .child(Button::new().on_press(move |_| show.toggle()).child("Open"))
            .child(
                Popup::new()
                    .on_close_request(move |_| show.set(false))
                    .maybe(show(), |popup| {
                        popup
                            .child(PopupTitle::new("Hello there".to_string()))
                            .child(PopupContent::new().child("Nothing important here."))
                            .child(
                                PopupButtons::new().child(
                                    Button::new()
                                        .on_press(move |_| show.set(false))
                                        .expanded()
                                        .filled()
                                        .child("Close"),
                                ),
                            )
                    }),
            )
    }

    CalendarDemo => "Calendar" {
        let mut selected = use_state(|| None::<CalendarDate>);
        let mut view_date = use_state(CalendarDate::now);

        Calendar::new()
            .selected(selected())
            .view_date(view_date())
            .on_change(move |date| selected.set(Some(date)))
            .on_view_change(move |date| view_date.set(date))
    }

    ColorPickerDemo => "ColorPicker" {
        let mut color = use_state(|| Color::from_hsv(220., 0.8, 1.));

        ColorPicker::new(move |new_color| color.set(new_color))
            .value(color())
            .width(Size::px(200.))
    }

    CircularLoaderDemo => "CircularLoader" {
        rect()
            .horizontal()
            .spacing(16.)
            .cross_align(Alignment::center())
            .child(CircularLoader::new().size(24.))
            .child(CircularLoader::new().size(48.))
    }

    SkeletonDemo => "Skeleton" {
        let mut loading = use_state(|| true);

        rect()
            .spacing(12.)
            .width(Size::px(220.))
            .cross_align(Alignment::center())
            .child(
                Skeleton::new(loading())
                    .width(Size::fill())
                    .height(Size::px(80.))
                    .animation(SkeletonAnimation::Shimmer)
                    .child(
                        rect()
                            .expanded()
                            .center()
                            .background((79, 70, 229))
                            .corner_radius(8.)
                            .color((255, 255, 255))
                            .child("Loaded"),
                    ),
            )
            .child(
                Button::new()
                    .on_press(move |_| loading.toggle())
                    .child(if loading() { "Load" } else { "Reset" }),
            )
    }

    TableDemo => "Table" {
        let mut ascending = use_state(|| true);

        let mut gods = [
            ("Zeus", "Sky"),
            ("Poseidon", "Sea"),
            ("Athena", "Strategy"),
            ("Hermes", "Messenger"),
        ];
        gods.sort_by_key(|(name, _)| *name);
        if !ascending() {
            gods.reverse();
        }

        Table::new()
            .column_widths([Size::flex(1.), Size::flex(1.)])
            .child(
                TableHead::new().child(
                    TableRow::new()
                        .child(
                            TableCell::new()
                                .order_direction(Some(if ascending() {
                                    OrderDirection::Down
                                } else {
                                    OrderDirection::Up
                                }))
                                .on_press(move |_| ascending.toggle())
                                .child("Name"),
                        )
                        .child(TableCell::new().child("Type")),
                ),
            )
            .child(
                TableBody::new()
                    .children(gods.into_iter().map(|(name, kind)| {
                        TableRow::new()
                            .key(name)
                            .child(TableCell::new().child(name))
                            .child(TableCell::new().child(kind))
                    })),
            )
    }

    ScrollViewDemo => "ScrollView" {
        ScrollView::new()
            .children((0..30).map(|index| {
                rect()
                    .key(index)
                    .height(Size::px(32.))
                    .main_align(Alignment::center())
                    .child(format!("Item {index}"))
            }))
    }

    VirtualScrollViewDemo => "VirtualScrollView" {
        VirtualScrollView::new(|item, _| {
            rect()
                .key(item.index)
                .height(Size::px(item.size))
                .main_align(Alignment::center())
                .child(format!("Item {}", item.index))
                .into()
        })
        .length(100_000usize)
        .item_size(32.)
        .expanded()
    }

    ImageViewerDemo => "ImageViewer" {
        rect()
            .horizontal()
            .spacing(12.)
            .cross_align(Alignment::center())
            .child(
                ImageViewer::new(("rust-logo", RUST_LOGO))
                    .image_cover(ImageCover::Center)
                    .width(Size::px(100.))
                    .height(Size::px(100.)),
            )
            .child(
                ImageViewer::new(PREVIEW_URL)
                    .image_cover(ImageCover::Center)
                    .width(Size::px(140.))
                    .height(Size::px(100.)),
            )
    }

    SvgViewerDemo => "SvgViewer" {
        let color = use_theme().read().colors.text_primary;

        rect()
            .horizontal()
            .spacing(14.)
            .children(
                [
                    lucide::house as fn() -> Bytes,
                    lucide::heart,
                    lucide::star,
                    lucide::settings,
                ]
                .map(|icon| {
                    SvgViewer::new(icon())
                        .stroke(color)
                        .width(Size::px(28.))
                        .height(Size::px(28.))
                }),
            )
    }

    GifViewerDemo => "GifViewer" {
        GifViewer::new(FROG_URL)
            .width(Size::px(200.))
            .height(Size::px(150.))
    }

    FloatingTabDemo => "FloatingTab" {
        rect()
            .spacing(8.)
            .child(FloatingTab::new().child("Page 1"))
            .child(FloatingTab::new().child("Page 2"))
    }

    SideBarItemDemo => "SideBarItem" {
        rect()
            .width(Size::px(200.))
            .spacing(4.)
            .child(SideBarItem::new().child("Inbox"))
            .child(SideBarItem::new().child("Drafts"))
            .child(SideBarItem::new().child("Sent"))
    }

    LinkDemo => "Link" {
        Link::new("https://github.com/marc2332/freya").child("github.com/marc2332/freya")
    }

    SelectableTextDemo => "SelectableText" {
        rect().width(Size::px(220.)).child(
            SelectableText::new().span("Select this text with the cursor, it can be copied."),
        )
    }

    ResizableContainerDemo => "ResizableContainer" {
        let panel = |text: &'static str| {
            rect()
                .expanded()
                .center()
                .background((79, 70, 229))
                .color((255, 255, 255))
                .child(text)
        };

        rect()
            .width(Size::px(220.))
            .height(Size::px(160.))
            .corner_radius(8.)
            .overflow(Overflow::Clip)
            .child(
                ResizableContainer::new()
                    .direction(Direction::Horizontal)
                    .panel(ResizablePanel::new(PanelSize::percent(50.)).child(panel("Drag")))
                    .panel(ResizablePanel::new(PanelSize::percent(50.)).child(panel("the bar"))),
            )
    }

    OverflowedContentDemo => "OverflowedContent" {
        OverflowedContent::new().width(Size::px(200.)).child(
            label()
                .text("This text is too long to fit, so it scrolls on its own.")
                .max_lines(1),
        )
    }
}

#[derive(PartialEq)]
pub struct GalleryShowcase;

impl Component for GalleryShowcase {
    fn render(&self) -> impl IntoElement {
        DemoGrid {
            title: "Gallery",
            subtitle: "Every component of Freya",
            demos: DEMOS,
        }
    }
}

fn button_variants(button: impl Fn() -> Button) -> Rect {
    rect()
        .spacing(8.)
        .cross_align(Alignment::center())
        .child(button().compact().child("Compact"))
        .child(button().child("Default"))
        .child(button().expanded().child("Expanded"))
}

fn input_variants(input: impl Fn() -> Input) -> Rect {
    rect()
        .spacing(8.)
        .width(Size::px(220.))
        .child(input().compact().width(Size::fill()).placeholder("Compact"))
        .child(input().width(Size::fill()).placeholder("Default"))
        .child(
            input()
                .expanded()
                .width(Size::fill())
                .placeholder("Expanded"),
        )
}
