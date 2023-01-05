use dioxus_native_core::node::NodeType;
use freya_common::NodeArea;
use freya_layers::DOMNode;
use freya_node_state::SizeMode;

use crate::ops_calc::run_calculations;

/// Calculate the area of a node given the remaining and it's parent areas
pub fn calculate_area(
    node_data: &DOMNode,
    mut remaining_area: NodeArea,
    parent_area: NodeArea,
) -> NodeArea {
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

    remaining_area.width = calculate(
        &node_data.get_state().size.width,
        remaining_area.width,
        parent_area.width,
    );
    remaining_area.height = calculate(
        &node_data.get_state().size.height,
        remaining_area.height,
        parent_area.height,
    );

    if SizeMode::Auto == node_data.get_state().size.height {
        if let NodeType::Element { tag, .. } = &node_data.get_type() {
            if tag == "label" {
                remaining_area.height = 18.0;
            }
        }
    }

    remaining_area.height = calculate_min(
        &node_data.get_state().size.min_height,
        remaining_area.height,
        parent_area.height,
    );
    remaining_area.width = calculate_min(
        &node_data.get_state().size.min_width,
        remaining_area.width,
        parent_area.width,
    );

    remaining_area.height = calculate_max(
        &node_data.get_state().size.max_height,
        remaining_area.height,
        parent_area.height,
    );
    remaining_area.width = calculate_max(
        &node_data.get_state().size.max_width,
        remaining_area.width,
        parent_area.width,
    );

    remaining_area
}
