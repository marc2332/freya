use freya_node_state::CalcType;

/// Calculate some chained operations with a given value.
/// This value could be for example the width of a node's parent area.
pub fn run_calculations(calcs: &Vec<CalcType>, value: f32) -> f32 {
    let mut prev_number: Option<f32> = None;
    let mut prev_op: Option<CalcType> = None;

    let mut calc_with_op = |val: f32, prev_op: Option<CalcType>| {
        if let Some(op) = prev_op {
            match op {
                CalcType::Sub => {
                    prev_number = Some(prev_number.unwrap() - val);
                }
                CalcType::Add => {
                    prev_number = Some(prev_number.unwrap() + val);
                }
                CalcType::Mul => {
                    prev_number = Some(prev_number.unwrap() * val);
                }
                CalcType::Div => {
                    prev_number = Some(prev_number.unwrap() / val);
                }
                _ => {}
            }
        } else {
            prev_number = Some(val);
        }
    };

    for calc in calcs {
        match calc {
            CalcType::Percentage(per) => {
                let val = (value / 100.0 * per).round();

                calc_with_op(val, prev_op);

                prev_op = None;
            }
            CalcType::Manual(val) => {
                calc_with_op(*val, prev_op);
                prev_op = None;
            }
            _ => prev_op = Some(*calc),
        }
    }

    prev_number.unwrap()
}
