pub fn does_element_have_intrinsic_layout(tag: &str) -> bool {
    tag != "text"
}

pub fn does_element_have_children_with_intrinsic_layout(tag: &str) -> bool {
    tag != "paragraph" && tag != "text"
}
