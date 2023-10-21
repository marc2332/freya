use crate::prelude::{
    get_align_axis, AlignAxis, AlignmentDirection, Area, DirectionMode, Gaps, Node, Size, Size2D,
};

/// Measurement data for the inner Nodes of a Node
#[derive(Debug)]
pub enum MeasureMode<'a> {
    ParentIsCached {
        inner_area: &'a Area,
    },
    ParentIsNotCached {
        area: &'a mut Area,
        inner_area: &'a mut Area,
        padding: Gaps,
    },
}

impl<'a> MeasureMode<'a> {
    /// Get a reference to the inner area
    pub fn inner_area(&'a self) -> &'a Area {
        match self {
            Self::ParentIsCached { inner_area } => inner_area,
            Self::ParentIsNotCached { inner_area, .. } => inner_area,
        }
    }

    /// Create an owned version of [MeasureMode]
    pub fn to_owned(&self) -> OwnedMeasureMode {
        match self {
            MeasureMode::ParentIsCached { inner_area } => OwnedMeasureMode::ParentIsCached {
                inner_area: *inner_area.to_owned(),
            },
            MeasureMode::ParentIsNotCached {
                area,
                inner_area,
                padding,
            } => OwnedMeasureMode::ParentIsNotCached {
                area: **area.clone(),
                inner_area: **inner_area.clone(),
                padding: *padding,
            },
        }
    }

    /// This will fit the available area and inner area of a parent node when for example height is set to "auto",
    /// direction is vertical and main_alignment is set to "center" or "end".
    /// The intended usage is to call this after the first measurement and before the second,
    /// this way the second measurement will align the content relatively to the parent element instead
    /// of overflowing due to being aligned relatively to the upper parent element
    pub fn fit_bounds_when_unspecified_and_aligned(
        &mut self,
        node: &Node,
        alignment_direction: AlignmentDirection,
        available_area: &mut Area,
    ) {
        struct NodeData<'a> {
            pub inner_origin: &'a mut f32,
            pub inner_size: &'a mut f32,
            pub area_origin: &'a mut f32,
            pub area_size: &'a mut f32,
            pub one_side_padding: f32,
            pub two_sides_padding: f32,
            pub one_side_margin: f32,
            pub two_sides_margin: f32,
            pub available_size: &'a mut f32,
        }

        let axis = get_align_axis(&node.direction, alignment_direction);
        let (is_vertical_not_start, is_horizontal_not_start) = match node.direction {
            DirectionMode::Vertical => (
                node.main_alignment.is_not_start(),
                node.cross_alignment.is_not_start(),
            ),
            DirectionMode::Horizontal => (
                node.cross_alignment.is_not_start(),
                node.main_alignment.is_not_start(),
            ),
        };
        let params = if let MeasureMode::ParentIsNotCached {
            area,
            inner_area,
            padding,
        } = self
        {
            match axis {
                AlignAxis::Height if Size::Inner == node.height && is_vertical_not_start => {
                    Some(NodeData {
                        inner_origin: &mut inner_area.origin.y,
                        inner_size: &mut inner_area.size.height,
                        area_origin: &mut area.origin.y,
                        area_size: &mut area.size.height,
                        one_side_padding: node.padding.top(),
                        two_sides_padding: padding.vertical(),
                        one_side_margin: node.margin.top(),
                        two_sides_margin: node.margin.vertical(),
                        available_size: &mut available_area.size.height,
                    })
                }
                AlignAxis::Width if Size::Inner == node.width && is_horizontal_not_start => {
                    Some(NodeData {
                        inner_origin: &mut inner_area.origin.x,
                        inner_size: &mut inner_area.size.width,
                        area_origin: &mut area.origin.x,
                        area_size: &mut area.size.width,
                        one_side_padding: node.padding.left(),
                        two_sides_padding: padding.horizontal(),
                        one_side_margin: node.margin.left(),
                        two_sides_margin: node.margin.horizontal(),
                        available_size: &mut available_area.size.width,
                    })
                }
                _ => None,
            }
        } else {
            None
        };

        if let Some(NodeData {
            inner_origin,
            inner_size,
            area_origin,
            area_size,
            one_side_padding,
            two_sides_padding,
            one_side_margin,
            two_sides_margin,
            available_size,
        }) = params
        {
            // Set the origin of the inner area to the origin of the area plus the padding and margin for the given axis
            *inner_origin = *area_origin + one_side_padding + one_side_margin;
            // Set the size of the inner area to the size of the area minus the padding and margin for the given axis
            *inner_size = *area_size - two_sides_padding - two_sides_margin;
            // Set the same available size as the inner area for the given axis
            *available_size = *inner_size;
        }
    }

    /// Stack a Node into another Node
    pub fn stack_node(
        &mut self,
        node: &Node,
        available_area: &mut Area,
        content_area: &Area,
        inner_sizes: &mut Size2D,
    ) {
        match node.direction {
            DirectionMode::Horizontal => {
                // Move the available area
                available_area.origin.x = content_area.max_x();
                available_area.size.width -= content_area.size.width;

                if let MeasureMode::ParentIsNotCached {
                    area,
                    padding,
                    inner_area,
                } = self
                {
                    inner_sizes.height = content_area.height().max(inner_sizes.height);
                    inner_sizes.width += content_area.width();

                    // Keep the biggest height
                    if node.height == Size::Inner {
                        area.size.height = area.size.height.max(
                            content_area.size.height + padding.vertical() + node.margin.vertical(),
                        );
                        // Keep the inner area in sync
                        inner_area.size.height =
                            area.size.height - padding.vertical() - node.margin.vertical();
                    }

                    // Accumulate width
                    if node.width == Size::Inner {
                        area.size.width += content_area.size.width;
                    }
                }
            }
            DirectionMode::Vertical => {
                // Move the available area
                available_area.origin.y = content_area.max_y();
                available_area.size.height -= content_area.size.height;

                if let MeasureMode::ParentIsNotCached {
                    area,
                    padding,
                    inner_area,
                } = self
                {
                    inner_sizes.width = content_area.width().max(inner_sizes.width);
                    inner_sizes.height += content_area.height();

                    // Keep the biggest width
                    if node.width == Size::Inner {
                        area.size.width = area.size.width.max(
                            content_area.size.width
                                + padding.horizontal()
                                + node.margin.horizontal(),
                        );
                        // Keep the inner area in sync
                        inner_area.size.width =
                            area.size.width - padding.horizontal() - node.margin.horizontal();
                    }

                    // Accumulate height
                    if node.height == Size::Inner {
                        area.size.height += content_area.size.height;
                    }
                }
            }
        }
    }
}

/// Just an owned version of [MeasureMode]
#[derive(Debug)]
pub enum OwnedMeasureMode {
    ParentIsCached {
        inner_area: Area,
    },
    ParentIsNotCached {
        area: Area,
        inner_area: Area,
        padding: Gaps,
    },
}

impl OwnedMeasureMode {
    pub fn to_mut(&mut self) -> MeasureMode<'_> {
        match self {
            Self::ParentIsCached { inner_area } => MeasureMode::ParentIsCached { inner_area },
            Self::ParentIsNotCached {
                area,
                inner_area,
                padding,
            } => MeasureMode::ParentIsNotCached {
                area,
                inner_area,
                padding: *padding,
            },
        }
    }
}
