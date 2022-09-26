use freya_layout::run_calculations;
use freya_node_state::node::CalcType;

#[test]
fn works_per_add_man() {
    // 50% + 50 = 250 + 50 = 300
    let result = run_calculations(
        &vec![
            CalcType::Percentage(50.0),
            CalcType::Add,
            CalcType::Manual(50.0),
        ],
        500.0,
    );
    assert_eq!(result, 300.0);
}

#[test]
fn works_per_sub_man_sum_per() {
    //10% - 25 + 75% = 50 - 25 + 375 = 400
    let result = run_calculations(
        &vec![
            CalcType::Percentage(10.0),
            CalcType::Sub,
            CalcType::Manual(25.0),
            CalcType::Add,
            CalcType::Percentage(75.0),
        ],
        500.0,
    );
    assert_eq!(result, 400.0);
}

#[test]
fn works_man_div_per_mul_man() {
    // 1000 / 50% * 25 = 1000 / 250 * 25 = 100
    let result = run_calculations(
        &vec![
            CalcType::Manual(1000.0),
            CalcType::Div,
            CalcType::Percentage(50.0),
            CalcType::Mul,
            CalcType::Manual(25.0),
        ],
        500.0,
    );
    assert_eq!(result, 100.0);
}
