use accesskit::{
    AriaCurrent,
    AutoComplete,
    DefaultActionVerb,
    HasPopup,
    Invalid,
    ListStyle,
    Live,
    Orientation,
    SortDirection,
    Toggled,
    VerticalOffset,
};

use crate::{
    Parse,
    ParseError,
};

impl Parse for Invalid {
    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(match value {
            "true" => Invalid::True,
            "grammar" => Invalid::Grammar,
            "spelling" => Invalid::Spelling,
            _ => Err(ParseError)?,
        })
    }
}

impl Parse for Toggled {
    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(match value {
            "true" => Toggled::True,
            "false" => Toggled::False,
            "mixed" => Toggled::Mixed,
            _ => Err(ParseError)?,
        })
    }
}

impl Parse for Live {
    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(match value {
            "assertive" => Live::Assertive,
            "off" => Live::Off,
            "polite" => Live::Polite,
            _ => Err(ParseError)?,
        })
    }
}

impl Parse for DefaultActionVerb {
    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(match value {
            "click" => DefaultActionVerb::Click,
            "focus" => DefaultActionVerb::Focus,
            "check" => DefaultActionVerb::Check,
            "uncheck" => DefaultActionVerb::Uncheck,
            "click-ancestor" => DefaultActionVerb::ClickAncestor,
            "jump" => DefaultActionVerb::Jump,
            "open" => DefaultActionVerb::Open,
            "press" => DefaultActionVerb::Press,
            "select" => DefaultActionVerb::Select,
            "unselect" => DefaultActionVerb::Unselect,
            _ => Err(ParseError)?,
        })
    }
}

impl Parse for Orientation {
    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(match value {
            "horizontal" => Orientation::Horizontal,
            "vertical" => Orientation::Vertical,
            _ => Err(ParseError)?,
        })
    }
}

impl Parse for SortDirection {
    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(match value {
            "ascending" => SortDirection::Ascending,
            "descending" => SortDirection::Descending,
            "other" => SortDirection::Other,
            _ => Err(ParseError)?,
        })
    }
}

impl Parse for AriaCurrent {
    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(match value {
            "false" => AriaCurrent::False,
            "true" => AriaCurrent::True,
            "page" => AriaCurrent::Page,
            "step" => AriaCurrent::Step,
            "location" => AriaCurrent::Location,
            "date" => AriaCurrent::Date,
            "time" => AriaCurrent::Time,
            _ => Err(ParseError)?,
        })
    }
}

impl Parse for AutoComplete {
    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(match value {
            "inline" => AutoComplete::Inline,
            "list" => AutoComplete::List,
            "both" => AutoComplete::Both,
            _ => Err(ParseError)?,
        })
    }
}

impl Parse for HasPopup {
    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(match value {
            "true" => HasPopup::True,
            "menu" => HasPopup::Menu,
            "listbox" => HasPopup::Listbox,
            "tree" => HasPopup::Tree,
            "grid" => HasPopup::Grid,
            "dialog" => HasPopup::Dialog,
            _ => Err(ParseError)?,
        })
    }
}

impl Parse for ListStyle {
    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(match value {
            "circle" => ListStyle::Circle,
            "disc" => ListStyle::Disc,
            "image" => ListStyle::Image,
            "numeric" => ListStyle::Numeric,
            "square" => ListStyle::Square,
            "other" => ListStyle::Other,
            _ => Err(ParseError)?,
        })
    }
}

impl Parse for VerticalOffset {
    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(match value {
            "subscript" => VerticalOffset::Subscript,
            "superscript" => VerticalOffset::Superscript,
            _ => Err(ParseError)?,
        })
    }
}
