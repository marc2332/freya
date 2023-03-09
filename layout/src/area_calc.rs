use dioxus_native_core::node::NodeType;
use freya_common::NodeArea;
use freya_node_state::SizeMode;

use crate::{ops_calc::run_calculations, NodeLayoutMeasurer};

pub fn calculate_area(node_measurer: &NodeLayoutMeasurer) -> NodeArea {
    let mut area = *node_measurer.remaining_area;

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

    area.width = calculate(
        &node_measurer.node.state.size.width,
        area.width,
        node_measurer.parent_area.width,
    );
    area.height = calculate(
        &node_measurer.node.state.size.height,
        area.height,
        node_measurer.parent_area.height,
    );

    if SizeMode::Auto == node_measurer.node.state.size.height {
        if let NodeType::Element { tag, .. } = &node_measurer.node.node_data.node_type {
            if tag == "label" {
                area.height = 18.0;
            }
        }
    }

    area.height = calculate_min(
        &node_measurer.node.state.size.min_height,
        area.height,
        node_measurer.parent_area.height,
    );
    area.width = calculate_min(
        &node_measurer.node.state.size.min_width,
        area.width,
        node_measurer.parent_area.width,
    );

    area.height = calculate_max(
        &node_measurer.node.state.size.max_height,
        area.height,
        node_measurer.parent_area.height,
    );
    area.width = calculate_max(
        &node_measurer.node.state.size.max_width,
        area.width,
        node_measurer.parent_area.width,
    );

    area
}
