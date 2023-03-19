use dioxus_native_core::{node::NodeType, prelude::ElementNode, real_dom::NodeImmutable};
use freya_common::NodeArea;
use freya_node_state::{Size, SizeMode};

use crate::{ops_calc::run_calculations, NodeLayoutMeasurer};

pub fn calculate_area(node_measurer: &NodeLayoutMeasurer) -> NodeArea {
    let mut area = *node_measurer.remaining_area;

    let Size {
        width,
        height,
        max_height,
        min_height,
        max_width,
        min_width,
        ..
    } = &*node_measurer.node.get::<Size>().unwrap();

    let calculate = |value: &SizeMode, area_value: f32, parent_area_value: f32| -> f32 {
        match value {
            &SizeMode::Manual(v) => v,
            SizeMode::Percentage(per) => (parent_area_value / 100.0 * per).round(),
            SizeMode::Auto => area_value,
            SizeMode::Calculation(calcs) => run_calculations(calcs, parent_area_value),
        }
    };

    let calculate_min = |value: &SizeMode, area_value: f32, parent_area_value: f32| -> f32 {
        match value {
            &SizeMode::Manual(v) => {
                if v > area_value {
                    v
                } else {
                    area_value
                }
            }
            SizeMode::Percentage(per) => {
                let by_per = (parent_area_value / 100.0 * per).round();
                if by_per > area_value {
                    by_per
                } else {
                    area_value
                }
            }
            SizeMode::Auto => area_value,
            SizeMode::Calculation(calcs) => {
                let by_calcs = run_calculations(calcs, parent_area_value);
                if by_calcs > area_value {
                    by_calcs
                } else {
                    area_value
                }
            }
        }
    };

    let calculate_max = |value: &SizeMode, area_value: f32, parent_area_value: f32| -> f32 {
        match value {
            &SizeMode::Manual(v) => {
                if v > area_value {
                    area_value
                } else {
                    v
                }
            }
            SizeMode::Percentage(per) => {
                let by_per = (parent_area_value / 100.0 * per).round();
                if by_per > area_value {
                    area_value
                } else {
                    by_per
                }
            }
            SizeMode::Auto => area_value,
            SizeMode::Calculation(calcs) => {
                let by_calcs = run_calculations(calcs, parent_area_value);
                if by_calcs > area_value {
                    area_value
                } else {
                    by_calcs
                }
            }
        }
    };

    area.width = calculate(width, area.width, node_measurer.parent_area.width);
    area.height = calculate(height, area.height, node_measurer.parent_area.height);

    if &SizeMode::Auto == height {
        if let NodeType::Element(ElementNode { tag, .. }) = &*node_measurer.node.node_type() {
            if tag == "label" {
                area.height = 18.0;
            }
        }
    }

    area.height = calculate_min(min_height, area.height, node_measurer.parent_area.height);
    area.width = calculate_min(min_width, area.width, node_measurer.parent_area.width);

    area.height = calculate_max(max_height, area.height, node_measurer.parent_area.height);
    area.width = calculate_max(max_width, area.width, node_measurer.parent_area.width);

    area
}
