use std::{
    collections::HashMap,
    fmt::Debug,
    hash::{
        DefaultHasher,
        Hash,
        Hasher,
    },
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
        Point2D,
        Position,
        Size2D,
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
    layout: LayoutData,
    show: bool,
    dependency: Option<u64>,
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
            layout: LayoutData::default(),
            show: true,
            dependency: None,
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

    pub fn show(mut self, show: bool) -> Self {
        self.show = show;
        self
    }

    /// Only animate when this value changes, other layout changes move the portal instantly.
    pub fn animation_dependency(mut self, dependency: impl Hash) -> Self {
        let mut hasher = DefaultHasher::default();
        dependency.hash(&mut hasher);
        self.dependency = Some(hasher.finish());
        self
    }
}

impl<T> LayoutExt for Portal<T> {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl<T> ContainerSizeExt for Portal<T> {}

impl<T> KeyExt for Portal<T> {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl<T: Clone + Eq + Hash + Debug + 'static> Component for Portal<T> {
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
        let dependency = self.dependency;
        // Area left by the previous mount of this same portal
        let init_size = use_hook(move || {
            positions
                .ids
                .write()
                .remove(&id)
                .filter(|(_, last)| dependency.is_none() || *last != dependency)
                .map(|(area, _)| area)
        });
        let mut previous_size = use_state::<Option<Area>>(|| None);
        let mut current_size = use_state::<Option<Area>>(|| None);
        let mut last_dependency = use_state::<Option<u64>>(|| None);
        let mut animating = use_state(|| false);

        let mut animation = use_animation_with_dependencies(
            &(self.function, self.duration, self.ease),
            move |conf, (function, duration, ease)| {
                conf.on_change(OnChange::Nothing);
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

        // Rest the portal once the animation stops
        use_side_effect(move || {
            if !*animation.is_running().read() {
                animating.set_if_modified(false);
            }
        });

        // Area the children are placed at
        let at_rest = !animating() && current_size.read().is_some();
        let area = if at_rest {
            current_size.read().unwrap_or_default()
        } else {
            let (x, y, width, height) = animation.get().value();
            Area::new(Point2D::new(x, y), Size2D::new(width, height))
        };

        // Resting portals are placed by the layout rather than positioned globally
        let is_new = init_size.is_none() && current_size.read().is_none();
        let is_stacked = self.dependency.is_some()
            && (is_new || (at_rest && self.dependency == *last_dependency.read()));
        let global_area = (!is_stacked).then_some(area);

        let id = self.id.clone();
        let show = self.show;

        rect()
            .a11y_focusable(false)
            .on_sized(move |e: Event<SizedEventData>| {
                if !show || *current_size.peek() == Some(e.area) {
                    return;
                }

                positions
                    .ids
                    .write()
                    .insert(id.clone(), (e.area, dependency));

                // Portals with a dependency only animate when it changes
                let dependency_changed =
                    dependency.is_none() || *last_dependency.peek() != dependency;
                last_dependency.set_if_modified(dependency);
                let animate =
                    dependency_changed && (init_size.is_some() || current_size.peek().is_some());

                previous_size.set(current_size());
                current_size.set(Some(e.area));
                animating.set_if_modified(animate);

                spawn(async move {
                    if animate {
                        animation.start();
                    } else {
                        animation.finish();
                    }
                });
            })
            .width(self.layout.width.clone())
            .height(self.layout.height.clone())
            .child(
                rect()
                    .map(global_area, |el, area| {
                        el.offset_x(area.min_x())
                            .offset_y(area.min_y())
                            .position(Position::new_global())
                    })
                    .child(
                        rect()
                            .width(global_area.map_or(Size::fill(), |area| Size::px(area.width())))
                            .height(
                                global_area.map_or(Size::fill(), |area| Size::px(area.height())),
                            )
                            // Only show the element after it has been sized
                            .opacity(if is_stacked || !is_new { 1. } else { 0. })
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
    pub ids: State<HashMap<T, (Area, Option<u64>)>>,
}
