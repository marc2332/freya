use freya_core::prelude::*;

/// Returns `true` for accessibility roles that require IME input (text fields, terminals, etc.).
pub fn is_ime_role(role: AccessibilityRole) -> bool {
    matches!(
        role,
        AccessibilityRole::TextInput
            | AccessibilityRole::MultilineTextInput
            | AccessibilityRole::PasswordInput
            | AccessibilityRole::SearchInput
            | AccessibilityRole::DateInput
            | AccessibilityRole::DateTimeInput
            | AccessibilityRole::WeekInput
            | AccessibilityRole::MonthInput
            | AccessibilityRole::TimeInput
            | AccessibilityRole::EmailInput
            | AccessibilityRole::NumberInput
            | AccessibilityRole::PhoneNumberInput
            | AccessibilityRole::UrlInput
            | AccessibilityRole::Terminal
    )
}
