use std::{
    collections::HashMap,
    fmt::Debug,
    time::Duration,
};

use freya_animation::prelude::*;
use freya_core::{
    prelude::*,
    scope_id::ScopeId,
};
use torin::{
    prelude::{
        Area,
        Position,
    },
    size::Size,
};

#[derive(PartialEq)]
pub struct Portal<T> {
    key: DiffKey,
    children: Vec<Element>,
    id: T,
    function: Function,
    duration: Duration,
    ease: Ease,
    width: Size,
    height: Size,
    show: bool,
}

impl<T> ChildrenExt for Portal<T> {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl<T> Portal<T> {
    pub fn new(id: T) -> Self {
        Self {
            key: DiffKey::None,
            children: vec![],
            id,
            function: Function::default(),
            duration: Duration::from_millis(750),
            ease: Ease::default(),
            width: Size::auto(),
            height: Size::auto(),
            show: true,
        }
    }

    pub fn function(mut self, function: Function) -> Self {
        self.function = function;
        self
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn ease(mut self, ease: Ease) -> Self {
        self.ease = ease;
        self
    }

    pub fn width(mut self, width: Size) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Size) -> Self {
        self.height = height;
        self
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = show;
        self
    }
}

impl<T> KeyExt for Portal<T> {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl<T: PartialEq + 'static + Clone + std::hash::Hash + Eq + Debug> Render for Portal<T> {
    fn render(&self) -> impl IntoElement {
        let mut positions = use_hook(|| match try_consume_context::<PortalsMap<T>>() {
            Some(ctx) => ctx,
            None => {
                let ctx = PortalsMap {
                    ids: State::create_in_scope(HashMap::default(), ScopeId::ROOT),
                };
                provide_context_for_scope_id(ctx.clone(), ScopeId::ROOT);
                ctx
            }
        });
        let id = self.id.clone();
        let init_size = use_hook(move || positions.ids.write().remove(&id));
        let mut previous_size = use_state::<Option<Area>>(|| None);
        let mut current_size = use_state::<Option<Area>>(|| None);

        let mut animation = use_animation_with_dependencies(
            &(self.function, self.duration, self.ease),
            move |_conf, (function, duration, ease)| {
                let from_size = previous_size
                    .read()
                    .unwrap_or(init_size.unwrap_or_default());
                let to_size = current_size.read().unwrap_or_default();
                (
                    AnimNum::new(from_size.origin.x, to_size.origin.x)
                        .duration(*duration)
                        .ease(*ease)
                        .function(*function),
                    AnimNum::new(from_size.origin.y, to_size.origin.y)
                        .duration(*duration)
                        .ease(*ease)
                        .function(*function),
                    AnimNum::new(from_size.size.width, to_size.size.width)
                        .duration(*duration)
                        .ease(*ease)
                        .function(*function),
                    AnimNum::new(from_size.size.height, to_size.size.height)
                        .duration(*duration)
                        .ease(*ease)
                        .function(*function),
                )
            },
        );

        let (offset_x, offset_y, width, height) = animation.get().value();
        let id = self.id.clone();
        let show = self.show;

        rect()
            .on_sized(move |e: Event<SizedEventData>| {
                if *current_size.peek() != Some(e.area) && show {
                    previous_size.set(current_size());
                    current_size.set(Some(e.area));
                    positions.ids.write().insert(id.clone(), e.area);

                    spawn(async move {
                        let has_init_size = init_size.is_some();
                        let has_previous_size = previous_size.peek().is_some();

                        if !*animation.has_run_yet().read() && !has_init_size {
                            // Mark the animation as finished if the component was just created and has no init size
                            animation.finish();
                        } else if has_init_size || has_previous_size {
                            // Start the animation if the component size changed and has a previous size
                            animation.start();
                        }
                    });
                }
            })
            .width(self.width.clone())
            .height(self.height.clone())
            .child(
                rect()
                    .offset_x(offset_x)
                    .offset_y(offset_y)
                    .position(Position::new_global())
                    .child(
                        rect()
                            .width(Size::px(width))
                            .height(Size::px(height))
                            // Only show the element after it has been sized
                            .opacity(
                                if init_size.is_some()
                                    || previous_size.read().is_some()
                                    || current_size.read().is_some()
                                {
                                    1.
                                } else {
                                    0.
                                },
                            )
                            .children(if self.show {
                                self.children.clone()
                            } else {
                                vec![]
                            }),
                    ),
            )
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

#[derive(Clone)]
pub struct PortalsMap<T: Clone + PartialEq + 'static> {
    pub ids: State<HashMap<T, Area>>,
}
