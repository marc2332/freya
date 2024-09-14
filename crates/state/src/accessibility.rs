use std::sync::{Arc, Mutex};

use accesskit::{
    AriaCurrent, AutoComplete, DefaultActionVerb, HasPopup, Invalid, ListStyle, Live, NodeBuilder,
    NodeId as AccessibilityId, Orientation, Role, SortDirection, Toggled, VerticalOffset,
};
use freya_common::{AccessibilityDirtyNodes, AccessibilityGenerator};
use freya_engine::prelude::Color;
use freya_native_core::{
    attributes::AttributeName,
    exports::shipyard::Component,
    node::OwnedAttributeValue,
    node_ref::NodeView,
    prelude::{AttributeMaskBuilder, Dependancy, NodeMaskBuilder, State},
    tags::TagName,
    NodeId, SendAnyMap,
};
use freya_native_core_macro::partial_derive_state;

use crate::{CustomAttributeValues, Parse, ParseAttribute, ParseError};

#[derive(Clone, Debug, PartialEq, Default, Component)]
pub struct AccessibilityNodeState {
    pub closest_accessibility_node_id: Option<NodeId>,
    pub descencent_accessibility_ids: Vec<AccessibilityId>,
    pub node_id: NodeId,
    pub focusable: bool,
    pub auto_focus: bool,
    pub a11y_id: Option<AccessibilityId>,
    pub builder: Option<NodeBuilder>,
}

impl ParseAttribute for AccessibilityNodeState {
    fn parse_attribute(
        &mut self,
        attr: freya_native_core::prelude::OwnedAttributeView<CustomAttributeValues>,
    ) -> Result<(), crate::ParseError> {
        match attr.attribute {
            AttributeName::Focus => {
                if let OwnedAttributeValue::Custom(CustomAttributeValues::AccessibilityId(id)) =
                    attr.value
                {
                    if self.builder.is_some() {
                        self.focusable = true;
                        self.a11y_id = Some(*id);
                    }
                }
            }
            AttributeName::AutoFocus => {
                if let OwnedAttributeValue::Text(attr) = attr.value {
                    self.auto_focus = attr.parse().unwrap_or_default()
                }
            }
            a11y_attr => {
                if let OwnedAttributeValue::Text(attr) = attr.value {
                    if let Some(builder) = self.builder.as_mut() {
                        match a11y_attr {
                            AttributeName::A11yName => builder.set_name(attr.clone()),
                            AttributeName::A11yDescription => builder.set_description(attr.clone()),
                            AttributeName::A11yValue => builder.set_value(attr.clone()),
                            AttributeName::A11yAccessKey => builder.set_access_key(attr.clone()),
                            AttributeName::A11yAuthorId => builder.set_author_id(attr.clone()),
                            AttributeName::A11yKeyboardShortcut => {
                                builder.set_keyboard_shortcut(attr.clone())
                            }
                            AttributeName::A11yLanguage => builder.set_language(attr.clone()),
                            AttributeName::A11yPlaceholder => builder.set_placeholder(attr.clone()),
                            AttributeName::A11yRoleDescription => {
                                builder.set_role_description(attr.clone())
                            }
                            AttributeName::A11yStateDescription => {
                                builder.set_state_description(attr.clone())
                            }
                            AttributeName::A11yTooltip => builder.set_tooltip(attr.clone()),
                            AttributeName::A11yUrl => builder.set_url(attr.clone()),
                            AttributeName::A11yRowIndexText => {
                                builder.set_row_index_text(attr.clone())
                            }
                            AttributeName::A11yColumnIndexText => {
                                builder.set_column_index_text(attr.clone())
                            }
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
                            AttributeName::A11yNumericValueStep => builder
                                .set_numeric_value_step(attr.parse().map_err(|_| ParseError)?),
                            AttributeName::A11yNumericValueJump => builder
                                .set_numeric_value_jump(attr.parse().map_err(|_| ParseError)?),
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
                                let color = Color::parse(&attr)?;
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
                            AttributeName::A11yHovered => {
                                if attr.parse::<bool>().map_err(|_| ParseError)? {
                                    builder.set_hovered();
                                }
                            }
                            AttributeName::A11yHidden => {
                                if attr.parse::<bool>().map_err(|_| ParseError)? {
                                    builder.set_hidden();
                                }
                            }
                            AttributeName::A11yLinked => {
                                if attr.parse::<bool>().map_err(|_| ParseError)? {
                                    builder.set_linked();
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
                            AttributeName::A11yRole => builder.set_role(
                                serde_json::from_str::<Role>(&format!("\"{attr}\""))
                                    .map_err(|_| ParseError)?,
                            ),
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
                                builder.set_default_action_verb(DefaultActionVerb::parse(attr)?);
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
        }

        Ok(())
    }
}

#[partial_derive_state]
impl State<CustomAttributeValues> for AccessibilityNodeState {
    type ParentDependencies = (Self,);

    type ChildDependencies = (Self,);

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> =
        NodeMaskBuilder::new().with_attrs(AttributeMaskBuilder::Some(&[
            AttributeName::Focus,
            AttributeName::AutoFocus,
            AttributeName::A11yName,
            AttributeName::A11yDescription,
            AttributeName::A11yValue,
            AttributeName::A11yAccessKey,
            AttributeName::A11yAuthorId,
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
            AttributeName::A11yHovered,
            AttributeName::A11yHidden,
            AttributeName::A11yLinked,
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
        ]));

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        context: &SendAnyMap,
    ) -> bool {
        let root_id = context.get::<NodeId>().unwrap();
        let accessibility_dirty_nodes = context
            .get::<Arc<Mutex<AccessibilityDirtyNodes>>>()
            .unwrap();
        let accessibility_generator = context.get::<Arc<AccessibilityGenerator>>().unwrap();
        let mut accessibility = AccessibilityNodeState {
            node_id: node_view.node_id(),
            a11y_id: self.a11y_id,
            builder: node_view.tag().and_then(|tag| {
                match tag {
                    TagName::Image => Some(NodeBuilder::new(Role::Image)),
                    TagName::Label => Some(NodeBuilder::new(Role::Label)),
                    TagName::Paragraph => Some(NodeBuilder::new(Role::Paragraph)),
                    TagName::Rect => Some(NodeBuilder::new(Role::GenericContainer)),
                    TagName::Svg => Some(NodeBuilder::new(Role::GraphicsObject)),
                    TagName::Root => Some(NodeBuilder::new(Role::Window)),
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

        for (child,) in children {
            if let Some(child_id) = child.a11y_id {
                // Mark this child as descendent if it has an ID
                accessibility.descencent_accessibility_ids.push(child_id)
            } else {
                // If it doesn't have an ID then use its descencent accessibility IDs
                accessibility
                    .descencent_accessibility_ids
                    .extend(child.descencent_accessibility_ids.iter());
            }
        }

        if let Some(parent) = parent {
            // Mark the parent accessibility ID as the closest to this node or
            // fallback to its closest ID.
            accessibility.closest_accessibility_node_id = parent
                .0
                .a11y_id
                .map(|_| parent.0.node_id)
                .or(parent.0.closest_accessibility_node_id);
        }

        let changed = &accessibility != self;
        let had_id = self.a11y_id.is_some();

        *self = accessibility;

        if changed {
            // Assign an accessibility ID if none was passed but the node has a valid builder
            //
            // In our case, builder will be `None` if the node's tag cannot be added to accessibility
            // tree.
            if self.a11y_id.is_none() && self.builder.is_some() {
                let id = AccessibilityId(accessibility_generator.new_id());
                #[cfg(debug_assertions)]
                tracing::info!("Assigned {id:?} to {:?}", node_view.node_id());

                self.a11y_id = Some(id)
            }

            let was_just_created = !had_id && self.a11y_id.is_some();

            // Add or update this node if it is the Root or if it has an accessibility ID
            if self.a11y_id.is_some() || node_view.node_id() == *root_id {
                accessibility_dirty_nodes
                    .lock()
                    .unwrap()
                    .add_or_update(node_view.node_id())
            }

            if was_just_created && self.auto_focus {
                #[cfg(debug_assertions)]
                tracing::info!("Requested auto focus for {:?}", self.a11y_id.unwrap());

                accessibility_dirty_nodes
                    .lock()
                    .unwrap()
                    .request_focus(node_view.node_id())
            }
        }

        changed
    }
}
