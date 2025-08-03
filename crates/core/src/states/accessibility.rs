use std::sync::{
    Arc,
    Mutex,
};

use accesskit::{
    Action,
    AriaCurrent,
    AutoComplete,
    HasPopup,
    Invalid,
    ListStyle,
    Live,
    Node,
    NodeId as AccessibilityId,
    Orientation,
    Role,
    SortDirection,
    Toggled,
    VerticalOffset,
};
use freya_native_core::{
    attributes::AttributeName,
    exports::shipyard::Component,
    node::OwnedAttributeValue,
    node_ref::NodeView,
    prelude::{
        AttributeMaskBuilder,
        Dependancy,
        NodeMaskBuilder,
        State,
    },
    tags::TagName,
    NodeId,
    SendAnyMap,
};
use freya_native_core_macro::partial_derive_state;

use crate::{
    accessibility::{
        AccessibilityDirtyNodes,
        AccessibilityFocusStrategy,
        AccessibilityGenerator,
    },
    custom_attributes::CustomAttributeValues,
    parsing::{
        Parse,
        ParseAttribute,
        ParseError,
    },
    values::{
        Color,
        Focusable,
    },
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Default, Component)]
pub struct AccessibilityState {
    pub node_id: NodeId,
    pub a11y_id: Option<AccessibilityId>,
    pub a11y_auto_focus: bool,
    pub a11y_focusable: Focusable,
    pub builder: Option<Node>,
}

impl ParseAttribute for AccessibilityState {
    fn parse_attribute(
        &mut self,
        attr: freya_native_core::prelude::OwnedAttributeView<CustomAttributeValues>,
    ) -> Result<(), ParseError> {
        match attr.attribute {
            AttributeName::A11yId => {
                if let OwnedAttributeValue::Custom(CustomAttributeValues::AccessibilityId(id)) =
                    attr.value
                {
                    self.a11y_id = Some(*id);
                    // Enable focus on nodes that pass a custom a11y id
                    if self.a11y_focusable.is_unknown() {
                        self.a11y_focusable = Focusable::Enabled;
                    }
                } else {
                    return Err(ParseError);
                }
            }
            AttributeName::A11yFocusable => {
                self.a11y_focusable = Focusable::parse(attr.value.as_text().ok_or(ParseError)?)?;
            }
            AttributeName::A11yAutoFocus => {
                self.a11y_auto_focus = attr
                    .value
                    .as_text()
                    .ok_or(ParseError)?
                    .parse()
                    .unwrap_or_default()
            }
            AttributeName::A11yMemberOf => {
                if let OwnedAttributeValue::Custom(CustomAttributeValues::AccessibilityId(id)) =
                    attr.value
                {
                    if let Some(builder) = self.builder.as_mut() {
                        builder.set_member_of(*id);
                    }
                }
            }
            a11y_attr => {
                if let Some(builder) = self.builder.as_mut() {
                    let attr = attr.value.as_text().ok_or(ParseError)?;
                    match a11y_attr {
                        AttributeName::A11yName => builder.set_class_name(attr),
                        AttributeName::A11yDescription => builder.set_description(attr),
                        AttributeName::A11yValue => builder.set_value(attr),
                        AttributeName::A11yAccessKey => builder.set_access_key(attr),
                        AttributeName::A11yAuthorId => builder.set_author_id(attr),
                        AttributeName::A11yKeyboardShortcut => builder.set_keyboard_shortcut(attr),
                        AttributeName::A11yLanguage => builder.set_language(attr),
                        AttributeName::A11yPlaceholder => builder.set_placeholder(attr),
                        AttributeName::A11yRoleDescription => builder.set_role_description(attr),
                        AttributeName::A11yStateDescription => builder.set_state_description(attr),
                        AttributeName::A11yTooltip => builder.set_tooltip(attr),
                        AttributeName::A11yUrl => builder.set_url(attr),
                        AttributeName::A11yRowIndexText => builder.set_row_index_text(attr),
                        AttributeName::A11yColumnIndexText => builder.set_column_index_text(attr),
                        AttributeName::A11yScrollX => {
                            builder.set_scroll_x(attr.parse().map_err(|_| ParseError)?)
                        }
                        AttributeName::A11yScrollXMin => {
                            builder.set_scroll_x_min(attr.parse().map_err(|_| ParseError)?)
                        }
                        AttributeName::A11yScrollXMax => {
                            builder.set_scroll_x_max(attr.parse().map_err(|_| ParseError)?)
                        }
                        AttributeName::A11yScrollY => {
                            builder.set_scroll_y(attr.parse().map_err(|_| ParseError)?)
                        }
                        AttributeName::A11yScrollYMin => {
                            builder.set_scroll_y_min(attr.parse().map_err(|_| ParseError)?)
                        }
                        AttributeName::A11yScrollYMax => {
                            builder.set_scroll_y_max(attr.parse().map_err(|_| ParseError)?)
                        }
                        AttributeName::A11yNumericValue => {
                            builder.set_numeric_value(attr.parse().map_err(|_| ParseError)?)
                        }
                        AttributeName::A11yMinNumericValue => {
                            builder.set_min_numeric_value(attr.parse().map_err(|_| ParseError)?)
                        }
                        AttributeName::A11yMaxNumericValue => {
                            builder.set_max_numeric_value(attr.parse().map_err(|_| ParseError)?)
                        }
                        AttributeName::A11yNumericValueStep => {
                            builder.set_numeric_value_step(attr.parse().map_err(|_| ParseError)?)
                        }
                        AttributeName::A11yNumericValueJump => {
                            builder.set_numeric_value_jump(attr.parse().map_err(|_| ParseError)?)
                        }
                        AttributeName::A11yRowCount => {
                            builder.set_row_count(attr.parse().map_err(|_| ParseError)?)
                        }
                        AttributeName::A11yColumnCount => {
                            builder.set_column_count(attr.parse().map_err(|_| ParseError)?)
                        }
                        AttributeName::A11yRowIndex => {
                            builder.set_row_index(attr.parse().map_err(|_| ParseError)?)
                        }
                        AttributeName::A11yColumnIndex => {
                            builder.set_column_index(attr.parse().map_err(|_| ParseError)?)
                        }
                        AttributeName::A11yRowSpan => {
                            builder.set_row_span(attr.parse().map_err(|_| ParseError)?)
                        }
                        AttributeName::A11yColumnSpan => {
                            builder.set_column_span(attr.parse().map_err(|_| ParseError)?)
                        }
                        AttributeName::A11yLevel => {
                            builder.set_level(attr.parse().map_err(|_| ParseError)?)
                        }
                        AttributeName::A11ySizeOfSet => {
                            builder.set_size_of_set(attr.parse().map_err(|_| ParseError)?)
                        }
                        AttributeName::A11yPositionInSet => {
                            builder.set_position_in_set(attr.parse().map_err(|_| ParseError)?)
                        }
                        AttributeName::A11yColorValue => {
                            let color = Color::parse(attr)?;
                            builder.set_color_value(
                                ((color.a() as u32) << 24)
                                    | ((color.b() as u32) << 16)
                                    | (((color.g() as u32) << 8) + (color.r() as u32)),
                            );
                        }
                        AttributeName::A11yExpanded => {
                            builder.set_expanded(attr.parse::<bool>().map_err(|_| ParseError)?);
                        }
                        AttributeName::A11ySelected => {
                            builder.set_selected(attr.parse::<bool>().map_err(|_| ParseError)?);
                        }
                        AttributeName::A11yHidden => {
                            if attr.parse::<bool>().map_err(|_| ParseError)? {
                                builder.set_hidden();
                            }
                        }
                        AttributeName::A11yMultiselectable => {
                            if attr.parse::<bool>().map_err(|_| ParseError)? {
                                builder.set_multiselectable();
                            }
                        }
                        AttributeName::A11yRequired => {
                            if attr.parse::<bool>().map_err(|_| ParseError)? {
                                builder.set_required();
                            }
                        }
                        AttributeName::A11yVisited => {
                            if attr.parse::<bool>().map_err(|_| ParseError)? {
                                builder.set_visited();
                            }
                        }
                        AttributeName::A11yBusy => {
                            if attr.parse::<bool>().map_err(|_| ParseError)? {
                                builder.set_busy();
                            }
                        }
                        AttributeName::A11yLiveAtomic => {
                            if attr.parse::<bool>().map_err(|_| ParseError)? {
                                builder.set_live_atomic();
                            }
                        }
                        AttributeName::A11yModal => {
                            if attr.parse::<bool>().map_err(|_| ParseError)? {
                                builder.set_modal();
                            }
                        }
                        AttributeName::A11yTouchTransparent => {
                            if attr.parse::<bool>().map_err(|_| ParseError)? {
                                builder.set_touch_transparent();
                            }
                        }
                        AttributeName::A11yReadOnly => {
                            if attr.parse::<bool>().map_err(|_| ParseError)? {
                                builder.set_read_only();
                            }
                        }
                        AttributeName::A11yDisabled => {
                            if attr.parse::<bool>().map_err(|_| ParseError)? {
                                builder.set_disabled();
                            }
                        }
                        AttributeName::A11yIsSpellingError => {
                            if attr.parse::<bool>().map_err(|_| ParseError)? {
                                builder.set_is_spelling_error();
                            }
                        }
                        AttributeName::A11yIsGrammarError => {
                            if attr.parse::<bool>().map_err(|_| ParseError)? {
                                builder.set_is_grammar_error();
                            }
                        }
                        AttributeName::A11yIsSearchMatch => {
                            if attr.parse::<bool>().map_err(|_| ParseError)? {
                                builder.set_is_search_match();
                            }
                        }
                        AttributeName::A11yIsSuggestion => {
                            if attr.parse::<bool>().map_err(|_| ParseError)? {
                                builder.set_is_suggestion();
                            }
                        }
                        AttributeName::A11yRole => {
                            builder.set_role(Role::parse(attr)?);
                        }
                        AttributeName::A11yInvalid => {
                            builder.set_invalid(Invalid::parse(attr)?);
                        }
                        AttributeName::A11yToggled => {
                            builder.set_toggled(Toggled::parse(attr)?);
                        }
                        AttributeName::A11yLive => {
                            builder.set_live(Live::parse(attr)?);
                        }
                        AttributeName::A11yDefaultActionVerb => {
                            builder.add_action(Action::parse(attr)?);
                        }
                        AttributeName::A11yOrientation => {
                            builder.set_orientation(Orientation::parse(attr)?);
                        }
                        AttributeName::A11ySortDirection => {
                            builder.set_sort_direction(SortDirection::parse(attr)?);
                        }
                        AttributeName::A11yCurrent => {
                            builder.set_aria_current(AriaCurrent::parse(attr)?);
                        }
                        AttributeName::A11yAutoComplete => {
                            builder.set_auto_complete(AutoComplete::parse(attr)?);
                        }
                        AttributeName::A11yHasPopup => {
                            builder.set_has_popup(HasPopup::parse(attr)?);
                        }
                        AttributeName::A11yListStyle => {
                            builder.set_list_style(ListStyle::parse(attr)?);
                        }
                        AttributeName::A11yVerticalOffset => {
                            builder.set_vertical_offset(VerticalOffset::parse(attr)?);
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }
}

#[partial_derive_state]
impl State<CustomAttributeValues> for AccessibilityState {
    type ParentDependencies = ();

    type ChildDependencies = ();

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> = NodeMaskBuilder::new()
        .with_attrs(AttributeMaskBuilder::Some(&[
            AttributeName::A11yId,
            AttributeName::A11yFocusable,
            AttributeName::A11yAutoFocus,
            AttributeName::A11yName,
            AttributeName::A11yDescription,
            AttributeName::A11yValue,
            AttributeName::A11yAccessKey,
            AttributeName::A11yAuthorId,
            AttributeName::A11yMemberOf,
            AttributeName::A11yKeyboardShortcut,
            AttributeName::A11yLanguage,
            AttributeName::A11yPlaceholder,
            AttributeName::A11yRoleDescription,
            AttributeName::A11yStateDescription,
            AttributeName::A11yTooltip,
            AttributeName::A11yUrl,
            AttributeName::A11yRowIndexText,
            AttributeName::A11yColumnIndexText,
            AttributeName::A11yScrollX,
            AttributeName::A11yScrollXMin,
            AttributeName::A11yScrollXMax,
            AttributeName::A11yScrollY,
            AttributeName::A11yScrollYMin,
            AttributeName::A11yScrollYMax,
            AttributeName::A11yNumericValue,
            AttributeName::A11yMinNumericValue,
            AttributeName::A11yMaxNumericValue,
            AttributeName::A11yNumericValueStep,
            AttributeName::A11yNumericValueJump,
            AttributeName::A11yRowCount,
            AttributeName::A11yColumnCount,
            AttributeName::A11yRowIndex,
            AttributeName::A11yColumnIndex,
            AttributeName::A11yRowSpan,
            AttributeName::A11yColumnSpan,
            AttributeName::A11yLevel,
            AttributeName::A11ySizeOfSet,
            AttributeName::A11yPositionInSet,
            AttributeName::A11yColorValue,
            AttributeName::A11yExpanded,
            AttributeName::A11ySelected,
            AttributeName::A11yHidden,
            AttributeName::A11yMultiselectable,
            AttributeName::A11yRequired,
            AttributeName::A11yVisited,
            AttributeName::A11yBusy,
            AttributeName::A11yLiveAtomic,
            AttributeName::A11yModal,
            AttributeName::A11yTouchTransparent,
            AttributeName::A11yReadOnly,
            AttributeName::A11yDisabled,
            AttributeName::A11yIsSpellingError,
            AttributeName::A11yIsGrammarError,
            AttributeName::A11yIsSearchMatch,
            AttributeName::A11yIsSuggestion,
            AttributeName::A11yRole,
            AttributeName::A11yInvalid,
            AttributeName::A11yToggled,
            AttributeName::A11yLive,
            AttributeName::A11yDefaultActionVerb,
            AttributeName::A11yOrientation,
            AttributeName::A11ySortDirection,
            AttributeName::A11yCurrent,
            AttributeName::A11yAutoComplete,
            AttributeName::A11yHasPopup,
            AttributeName::A11yListStyle,
            AttributeName::A11yVerticalOffset,
        ]))
        .with_tag();

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        _parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        context: &SendAnyMap,
    ) -> bool {
        let root_id = context.get::<NodeId>().unwrap();
        let accessibility_dirty_nodes = context
            .get::<Arc<Mutex<AccessibilityDirtyNodes>>>()
            .unwrap();
        let accessibility_generator = context.get::<Arc<AccessibilityGenerator>>().unwrap();
        let mut accessibility = AccessibilityState {
            node_id: node_view.node_id(),
            a11y_id: self.a11y_id,
            builder: node_view.tag().and_then(|tag| {
                match tag {
                    TagName::Image => Some(Node::new(Role::Image)),
                    TagName::Label => Some(Node::new(Role::Label)),
                    TagName::Paragraph => Some(Node::new(Role::Paragraph)),
                    TagName::Rect => Some(Node::new(Role::GenericContainer)),
                    TagName::Svg => Some(Node::new(Role::GraphicsObject)),
                    TagName::Root => Some(Node::new(Role::Window)),
                    // TODO: make this InlineTextBox and supply computed text span properties
                    TagName::Text => None,
                }
            }),
            ..Default::default()
        };

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                accessibility.parse_safe(attr);
            }
        }

        let changed = &accessibility != self;
        let had_id = self.a11y_id.is_some();

        *self = accessibility;

        let is_orphan = node_view.height() == 0 && node_view.node_id() != *root_id;

        if changed && !is_orphan {
            // Assign an accessibility ID if none was passed but the node has a valid builder
            //
            // In our case, builder will be `None` if the node's tag cannot be added to accessibility
            // tree.
            if self.a11y_id.is_none() && self.builder.is_some() {
                let id = AccessibilityId(accessibility_generator.new_id());
                #[cfg(debug_assertions)]
                tracing::info!("Assigned {id:?} to {:?}", node_view.node_id());

                self.a11y_id = Some(id);
            }

            // Add or update this node if it is the Root or if it has an accessibility ID
            if self.a11y_id.is_some() || node_view.node_id() == *root_id {
                accessibility_dirty_nodes
                    .lock()
                    .unwrap()
                    .add_or_update(node_view.node_id())
            }

            if let Some(a11y_id) = self.a11y_id {
                if !had_id && self.a11y_auto_focus {
                    #[cfg(debug_assertions)]
                    tracing::info!("Requested auto focus for {:?}", a11y_id);

                    accessibility_dirty_nodes
                        .lock()
                        .unwrap()
                        .request_focus(AccessibilityFocusStrategy::Node(a11y_id))
                }
            }
        }

        changed
    }
}
