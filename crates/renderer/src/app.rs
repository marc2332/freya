use accesskit::Action;
use accesskit_winit::ActionRequestEvent;
use dioxus_core::{Template, VirtualDom};
use dioxus_native_core::NodeId;
use freya_common::EventMessage;
use freya_core::prelude::*;
use freya_dom::prelude::SafeDOM;
use freya_elements::events::keyboard::{
    map_winit_key, map_winit_modifiers, map_winit_physical_key, Code, Key,
};
use freya_engine::prelude::*;
use futures::pin_mut;
use futures::FutureExt;
use std::{sync::Arc, task::Waker};
use tokio::sync::broadcast;
use tokio::{
    select,
    sync::{mpsc, watch, Notify},
};
use torin::geometry::CursorPoint;
use torin::geometry::{Area, Size2D};
use tracing::info;
use uuid::Uuid;
use winit::dpi::PhysicalSize;
use winit::event::{
    ElementState, Event, Ime, KeyEvent, MouseScrollDelta, StartCause, Touch, TouchPhase,
    WindowEvent,
};
use winit::event_loop::{EventLoop, EventLoopProxy};
use winit::keyboard::{KeyCode, ModifiersState, PhysicalKey};

use crate::{accessibility::AccessKitManager, renderer::render_skia, winit_waker::winit_waker};
use crate::{FontsConfig, HoveredNode, WindowEnv};

// https://github.com/emilk/egui/issues/461
// https://github.com/rust-windowing/winit/issues/22
// https://github.com/flutter/flutter/issues/71385
const WHEEL_SPEED_MODIFIER: f32 = 53.0;

/// Manages the Application lifecycle
pub struct App<State: 'static + Clone> {
    pub(crate) sdom: SafeDOM,
    pub(crate) vdom: VirtualDom,
    pub(crate) events: EventsQueue,
    pub(crate) vdom_waker: Waker,
    pub(crate) proxy: EventLoopProxy<EventMessage>,
    pub(crate) mutations_notifier: Option<Arc<Notify>>,
    pub(crate) event_emitter: EventEmitter,
    pub(crate) event_receiver: EventReceiver,
    pub(crate) window_env: WindowEnv<State>,
    pub(crate) layers: Layers,
    pub(crate) elements_state: ElementsState,
    pub(crate) viewports: Viewports,
    pub(crate) focus_sender: FocusSender,
    pub(crate) focus_receiver: FocusReceiver,
    pub(crate) accessibility: AccessKitManager,
    pub(crate) font_collection: FontCollection,
    pub(crate) ticker_sender: broadcast::Sender<()>,
    pub(crate) plugins: PluginsManager,
    pub(crate) navigator_state: NavigatorState,
}

impl<State: 'static + Clone> App<State> {
    pub fn new(
        sdom: SafeDOM,
        vdom: VirtualDom,
        proxy: &EventLoopProxy<EventMessage>,
        mutations_notifier: Option<Arc<Notify>>,
        mut window_env: WindowEnv<State>,
        fonts_config: FontsConfig,
        mut plugins: PluginsManager,
    ) -> Self {
        let accessibility = AccessKitManager::new(&window_env.window, proxy.clone());

        window_env.window_mut().set_visible(true);

        let mut font_collection = FontCollection::new();
        let def_mgr = FontMgr::default();

        let mut provider = TypefaceFontProvider::new();

        for (font_name, font_data) in fonts_config {
            let ft_type = def_mgr.new_from_data(font_data, None).unwrap();
            provider.register_typeface(ft_type, Some(font_name));
        }

        let mgr: FontMgr = provider.into();
        font_collection.set_default_font_manager(def_mgr, "Fira Sans");
        font_collection.set_dynamic_font_manager(mgr);

        let (event_emitter, event_receiver) = mpsc::unbounded_channel::<DomEvent>();
        let (focus_sender, focus_receiver) = watch::channel(ACCESSIBILITY_ROOT_ID);

        plugins.send(PluginEvent::WindowCreated(window_env.window_mut()));

        Self {
            sdom,
            vdom,
            events: EventsQueue::new(),
            vdom_waker: winit_waker(proxy),
            proxy: proxy.clone(),
            mutations_notifier,
            event_emitter,
            event_receiver,
            window_env,
            layers: Layers::default(),
            elements_state: ElementsState::default(),
            viewports: Viewports::default(),
            accessibility,
            focus_sender,
            focus_receiver,
            font_collection,
            ticker_sender: broadcast::channel(5).0,
            plugins,
            navigator_state: NavigatorState::new(NavigationMode::NotKeyboard),
        }
    }

    /// Provide the launch state and few other utilities like the EventLoopProxy
    pub fn provide_vdom_contexts(&mut self) {
        if let Some(state) = self.window_env.window_config.state.clone() {
            self.vdom.insert_any_root_context(Box::new(state));
        }
        self.vdom
            .insert_any_root_context(Box::new(self.proxy.clone()));
        self.vdom
            .insert_any_root_context(Box::new(self.focus_receiver.clone()));
        self.vdom
            .insert_any_root_context(Box::new(Arc::new(self.ticker_sender.subscribe())));
        self.vdom
            .insert_any_root_context(Box::new(self.navigator_state.clone()));
    }

    /// Make the first build of the VirtualDOM and sync it with the RealDOM.
    pub fn init_doms(&mut self) {
        let scale_factor = self.window_env.window.scale_factor() as f32;
        self.provide_vdom_contexts();

        self.sdom.get_mut().init_dom(&mut self.vdom, scale_factor);
    }

    /// Update the DOM with the mutations from the VirtualDOM.
    pub fn apply_vdom_changes(&mut self) -> (bool, bool) {
        let scale_factor = self.window_env.window.scale_factor() as f32;
        let (repaint, relayout) = self
            .sdom
            .get_mut()
            .render_mutations(&mut self.vdom, scale_factor);

        if repaint {
            if let Some(mutations_notifier) = &self.mutations_notifier {
                mutations_notifier.notify_one();
            }
        }

        (repaint, relayout)
    }

    /// Poll the VirtualDOM for any new change
    pub fn poll_vdom(&mut self) {
        let waker = &self.vdom_waker.clone();
        let mut cx = std::task::Context::from_waker(waker);

        loop {
            {
                let fut = async {
                    select! {
                        ev = self.event_receiver.recv() => {
                            if let Some(ev) = ev {
                                let data = ev.data.any();
                                self.vdom.handle_event(&ev.name, data, ev.element_id, true);

                                self.vdom.process_events();
                            }
                        },
                        _ = self.vdom.wait_for_work() => {},
                    }
                };
                pin_mut!(fut);

                match fut.poll_unpin(&mut cx) {
                    std::task::Poll::Ready(_) => {}
                    std::task::Poll::Pending => break,
                }
            }

            let (must_repaint, must_relayout) = self.apply_vdom_changes();

            if must_relayout {
                self.window_env.window.request_redraw();
            } else if must_repaint {
                self.proxy.send_event(EventMessage::RequestRedraw).unwrap();
            }
        }
    }

    /// Process the events queue
    pub fn process_events(&mut self) {
        let scale_factor = self.window_env.window.scale_factor();
        process_events(
            &self.sdom.get(),
            &self.layers,
            &mut self.events,
            &self.event_emitter,
            &mut self.elements_state,
            &self.viewports,
            scale_factor,
        )
    }

    /// Create the Accessibility tree
    /// This will iterater the DOM ordered by layers (top to bottom)
    /// and add every element with an accessibility ID to the Accessibility Tree
    pub fn process_accessibility(&mut self) {
        let fdom = &self.sdom.get();
        let layout = fdom.layout();
        let rdom = fdom.rdom();
        let layers = &self.layers;

        process_accessibility(
            layers,
            &layout,
            rdom,
            &mut self.accessibility.accessibility_manager().lock().unwrap(),
        );
    }

    /// Send an event
    pub fn send_event(&mut self, event: FreyaEvent) {
        self.events.push(event);
        self.process_events();
    }

    /// Replace a VirtualDOM Template
    pub fn vdom_replace_template(&mut self, template: Template) {
        self.vdom.replace_template(template);
    }

    /// Render the App into the Window Canvas
    pub fn render(&mut self, hovered_node: &HoveredNode) {
        self.plugins.send(PluginEvent::BeforeRender {
            canvas: self.window_env.canvas_mut(),
            font_collection: &self.font_collection,
            freya_dom: &self.sdom.get(),
            viewports: &self.viewports,
        });

        self.start_render(hovered_node);

        self.accessibility
            .render_accessibility(self.window_env.window.title().as_str());

        self.plugins.send(PluginEvent::AfterRender {
            canvas: self.window_env.canvas_mut(),
            font_collection: &self.font_collection,
            freya_dom: &self.sdom.get(),
            viewports: &self.viewports,
        });

        self.finish_render();
    }

    /// Resize the Window
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.sdom.get().layout().reset();
        self.window_env.resize(size);
    }

    /// Measure the a text group given it's ID.
    pub fn measure_text_group(&self, text_id: &Uuid) {
        let scale_factor = self.window_env.window.scale_factor() as f32;
        self.layers.measure_paragraph_elements(
            text_id,
            &self.sdom.get(),
            &self.font_collection,
            scale_factor,
        );
    }

    pub fn focus_next_node(&self, direction: AccessibilityFocusDirection) {
        self.accessibility
            .focus_next_node(direction, &self.focus_sender, &self.window_env.window)
    }

    /// Notify components subscribed to event loop ticks.
    pub fn event_loop_tick(&self) {
        self.ticker_sender.send(()).ok();
    }

    /// Update the [NavigationMode].
    pub fn set_navigation_mode(&mut self, mode: NavigationMode) {
        self.navigator_state.set(mode);
    }

    /// Measure the layout
    pub fn process_layout(&mut self) {
        self.accessibility.clear_accessibility();

        {
            let fdom = self.sdom.get();

            self.plugins
                .send(PluginEvent::StartedLayout(&fdom.layout()));

            let window_size = self.window_env.window.inner_size();
            let scale_factor = self.window_env.window.scale_factor() as f32;
            let (layers, viewports) = process_layout(
                &fdom,
                Area::from_size(Size2D::from((
                    window_size.width as f32,
                    window_size.height as f32,
                ))),
                &mut self.font_collection,
                scale_factor,
            );
            self.layers = layers;
            self.viewports = viewports;

            self.plugins
                .send(PluginEvent::FinishedLayout(&fdom.layout()));
        }

        if let Some(mutations_notifier) = &self.mutations_notifier {
            mutations_notifier.notify_one();
        }

        self.process_accessibility();

        info!(
            "Processed {} layers and {} group of paragraph elements",
            self.layers.len_layers(),
            self.layers.len_paragraph_elements()
        );
        info!("Processed {} viewports", self.viewports.size());
    }

    /// Start rendering the RealDOM to Window
    pub fn start_render(&mut self, hovered_node: &HoveredNode) {
        self.window_env.clear();

        let canvas = self.window_env.canvas_mut();
        let fdom = self.sdom.get();

        let mut matrices: Vec<(Matrix, Vec<NodeId>)> = Vec::default();
        let mut opacities: Vec<(f32, Vec<NodeId>)> = Vec::default();

        process_render(
            &self.viewports,
            &fdom,
            &mut self.font_collection,
            &self.layers,
            &mut (canvas, &mut matrices, &mut opacities),
            |dom, node_id, area, font_collection, viewports, (canvas, matrices, opacities)| {
                let render_wireframe = if let Some(hovered_node) = &hovered_node {
                    hovered_node
                        .lock()
                        .unwrap()
                        .map(|id| id == *node_id)
                        .unwrap_or_default()
                } else {
                    false
                };
                if let Some(dioxus_node) = dom.rdom().get(*node_id) {
                    render_skia(
                        canvas,
                        area,
                        &dioxus_node,
                        font_collection,
                        viewports,
                        render_wireframe,
                        matrices,
                        opacities,
                    );
                }
            },
        );
    }

    /// Finish all rendering in the Window
    pub fn finish_render(&mut self) {
        self.window_env.flush_and_submit();
    }

    /// Run the application.
    pub fn run(
        &mut self,
        event_loop: EventLoop<EventMessage>,
        proxy: EventLoopProxy<EventMessage>,
        hovered_node: HoveredNode,
    ) {
        let mut cursor_pos = CursorPoint::default();
        let mut modifiers_state = ModifiersState::empty();

        self.window_env.run_on_setup();

        event_loop
            .run(move |event, event_loop| match event {
                Event::NewEvents(StartCause::Init) => {
                    _ = proxy.send_event(EventMessage::PollVDOM);
                }
                Event::UserEvent(EventMessage::FocusAccessibilityNode(id)) => {
                    self.accessibility
                        .set_accessibility_focus(id, &self.window_env.window);
                }
                Event::UserEvent(EventMessage::RequestRerender) => {
                    self.window_env.window_mut().request_redraw();
                }
                Event::UserEvent(EventMessage::RequestRedraw) => self.render(&hovered_node),
                Event::UserEvent(EventMessage::RequestRelayout) => {
                    self.process_layout();
                }
                Event::UserEvent(EventMessage::RemeasureTextGroup(text_id)) => {
                    self.measure_text_group(&text_id);
                }
                Event::UserEvent(EventMessage::ActionRequestEvent(ActionRequestEvent {
                    request,
                    ..
                })) => {
                    if Action::Focus == request.action {
                        self.accessibility
                            .set_accessibility_focus(request.target, &self.window_env.window);
                    }
                }
                Event::UserEvent(EventMessage::SetCursorIcon(icon)) => {
                    self.window_env.window.set_cursor_icon(icon)
                }
                Event::UserEvent(ev) => {
                    if let EventMessage::UpdateTemplate(template) = ev {
                        self.vdom_replace_template(template);
                    }

                    if matches!(ev, EventMessage::PollVDOM)
                        || matches!(ev, EventMessage::UpdateTemplate(_))
                    {
                        self.poll_vdom();
                    }
                }
                Event::WindowEvent { event, .. } => {
                    self.accessibility
                        .process_accessibility_event(&event, &self.window_env.window);
                    match event {
                        WindowEvent::CloseRequested => event_loop.exit(),
                        WindowEvent::Ime(Ime::Commit(text)) => {
                            self.send_event(FreyaEvent::Keyboard {
                                name: "keydown".to_string(),
                                key: Key::Character(text),
                                code: Code::Unidentified,
                                modifiers: map_winit_modifiers(modifiers_state),
                            });
                        }
                        WindowEvent::RedrawRequested => {
                            self.process_layout();
                            self.render(&hovered_node);
                            self.event_loop_tick();
                        }
                        WindowEvent::MouseInput { state, button, .. } => {
                            self.set_navigation_mode(NavigationMode::NotKeyboard);

                            let event_name = match state {
                                ElementState::Pressed => "mousedown",
                                ElementState::Released => "click",
                            };

                            self.send_event(FreyaEvent::Mouse {
                                name: event_name.to_string(),
                                cursor: cursor_pos,
                                button: Some(button),
                            });
                        }
                        WindowEvent::MouseWheel { delta, phase, .. } => {
                            if TouchPhase::Moved == phase {
                                let scroll_data = {
                                    match delta {
                                        MouseScrollDelta::LineDelta(x, y) => (
                                            (x * WHEEL_SPEED_MODIFIER) as f64,
                                            (y * WHEEL_SPEED_MODIFIER) as f64,
                                        ),
                                        MouseScrollDelta::PixelDelta(pos) => (pos.x, pos.y),
                                    }
                                };

                                self.send_event(FreyaEvent::Wheel {
                                    name: "wheel".to_string(),
                                    scroll: CursorPoint::from(scroll_data),
                                    cursor: cursor_pos,
                                });
                            }
                        }
                        WindowEvent::ModifiersChanged(modifiers) => {
                            modifiers_state = modifiers.state();
                        }
                        WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    physical_key,
                                    logical_key,
                                    state,
                                    ..
                                },
                            ..
                        } => {
                            if state == ElementState::Pressed
                                && physical_key == PhysicalKey::Code(KeyCode::Tab)
                            {
                                self.set_navigation_mode(NavigationMode::Keyboard);

                                let direction = if modifiers_state.shift_key() {
                                    AccessibilityFocusDirection::Backward
                                } else {
                                    AccessibilityFocusDirection::Forward
                                };

                                self.focus_next_node(direction);

                                return;
                            }

                            let event_name = match state {
                                ElementState::Pressed => "keydown",
                                ElementState::Released => "keyup",
                            };
                            self.send_event(FreyaEvent::Keyboard {
                                name: event_name.to_string(),
                                key: map_winit_key(&logical_key),
                                code: map_winit_physical_key(&physical_key),
                                modifiers: map_winit_modifiers(modifiers_state),
                            })
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            cursor_pos = CursorPoint::from((position.x, position.y));

                            self.send_event(FreyaEvent::Mouse {
                                name: "mouseover".to_string(),
                                cursor: cursor_pos,
                                button: None,
                            });
                        }
                        WindowEvent::Touch(Touch {
                            location,
                            phase,
                            id,
                            force,
                            ..
                        }) => {
                            cursor_pos = CursorPoint::from((location.x, location.y));

                            let event_name = match phase {
                                TouchPhase::Cancelled => "touchcancel",
                                TouchPhase::Ended => "touchend",
                                TouchPhase::Moved => "touchmove",
                                TouchPhase::Started => "touchstart",
                            };

                            self.send_event(FreyaEvent::Touch {
                                name: event_name.to_string(),
                                location: cursor_pos,
                                finger_id: id,
                                phase,
                                force,
                            });
                        }
                        WindowEvent::Resized(size) => {
                            self.resize(size);
                        }
                        _ => {}
                    }
                }
                Event::LoopExiting => {
                    self.window_env.run_on_exit();
                }
                _ => (),
            })
            .expect("Failed to run Eventloop.");
    }
}
