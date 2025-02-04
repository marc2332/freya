use accesskit::{
    AriaCurrent,
    AutoComplete,
    DefaultActionVerb,
    HasPopup,
    Invalid,
    ListStyle,
    Live,
    Orientation,
    Role,
    SortDirection,
    Toggled,
    VerticalOffset,
};

use crate::parsing::{
    Parse,
    ParseError,
};

impl Parse for Role {
    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(match value {
            "unknown" => Self::Unknown,
            "inline-text-box" => Self::InlineTextBox,
            "cell" => Self::Cell,
            "label" => Self::Label,
            "image" => Self::Image,
            "link" => Self::Link,
            "row" => Self::Row,
            "list-item" => Self::ListItem,
            "list-marker" => Self::ListMarker,
            "tree-item" => Self::TreeItem,
            "list-box-option" => Self::ListBoxOption,
            "menu-item" => Self::MenuItem,
            "menu-list-option" => Self::MenuListOption,
            "paragraph" => Self::Paragraph,
            "generic-container" => Self::GenericContainer,
            "check-box" => Self::CheckBox,
            "radio-button" => Self::RadioButton,
            "text-input" => Self::TextInput,
            "button" => Self::Button,
            "default-button" => Self::DefaultButton,
            "pane" => Self::Pane,
            "row-header" => Self::RowHeader,
            "column-header" => Self::ColumnHeader,
            "row-group" => Self::RowGroup,
            "list" => Self::List,
            "table" => Self::Table,
            "layout-table-cell" => Self::LayoutTableCell,
            "layout-table-row" => Self::LayoutTableRow,
            "layout-table" => Self::LayoutTable,
            "switch" => Self::Switch,
            "menu" => Self::Menu,
            "multiline-text-input" => Self::MultilineTextInput,
            "search-input" => Self::SearchInput,
            "date-input" => Self::DateInput,
            "date-time-input" => Self::DateTimeInput,
            "week-input" => Self::WeekInput,
            "month-input" => Self::MonthInput,
            "time-input" => Self::TimeInput,
            "email-input" => Self::EmailInput,
            "number-input" => Self::NumberInput,
            "password-input" => Self::PasswordInput,
            "phone-number-input" => Self::PhoneNumberInput,
            "url-input" => Self::UrlInput,
            "abbr" => Self::Abbr,
            "alert" => Self::Alert,
            "alert-dialog" => Self::AlertDialog,
            "application" => Self::Application,
            "article" => Self::Article,
            "audio" => Self::Audio,
            "banner" => Self::Banner,
            "blockquote" => Self::Blockquote,
            "canvas" => Self::Canvas,
            "caption" => Self::Caption,
            "caret" => Self::Caret,
            "code" => Self::Code,
            "color-well" => Self::ColorWell,
            "combo-box" => Self::ComboBox,
            "editable-combo-box" => Self::EditableComboBox,
            "complementary" => Self::Complementary,
            "comment" => Self::Comment,
            "content-deletion" => Self::ContentDeletion,
            "content-insertion" => Self::ContentInsertion,
            "content-info" => Self::ContentInfo,
            "definition" => Self::Definition,
            "description-list" => Self::DescriptionList,
            "description-list-detail" => Self::DescriptionListDetail,
            "description-list-term" => Self::DescriptionListTerm,
            "details" => Self::Details,
            "dialog" => Self::Dialog,
            "directory" => Self::Directory,
            "disclosure-triangle" => Self::DisclosureTriangle,
            "document" => Self::Document,
            "embedded-object" => Self::EmbeddedObject,
            "emphasis" => Self::Emphasis,
            "feed" => Self::Feed,
            "figure-caption" => Self::FigureCaption,
            "figure" => Self::Figure,
            "footer" => Self::Footer,
            "footer-as-non-landmark" => Self::FooterAsNonLandmark,
            "form" => Self::Form,
            "grid" => Self::Grid,
            "group" => Self::Group,
            "header" => Self::Header,
            "header-as-non-landmark" => Self::HeaderAsNonLandmark,
            "heading" => Self::Heading,
            "iframe" => Self::Iframe,
            "iframe-presentational" => Self::IframePresentational,
            "ime-candidate" => Self::ImeCandidate,
            "keyboard" => Self::Keyboard,
            "legend" => Self::Legend,
            "line-break" => Self::LineBreak,
            "list-box" => Self::ListBox,
            "log" => Self::Log,
            "main" => Self::Main,
            "mark" => Self::Mark,
            "marquee" => Self::Marquee,
            "math" => Self::Math,
            "menu-bar" => Self::MenuBar,
            "menu-item-check-box" => Self::MenuItemCheckBox,
            "menu-item-radio" => Self::MenuItemRadio,
            "menu-list-popup" => Self::MenuListPopup,
            "meter" => Self::Meter,
            "navigation" => Self::Navigation,
            "note" => Self::Note,
            "plugin-object" => Self::PluginObject,
            "portal" => Self::Portal,
            "pre" => Self::Pre,
            "progress-indicator" => Self::ProgressIndicator,
            "radio-group" => Self::RadioGroup,
            "region" => Self::Region,
            "root-web-area" => Self::RootWebArea,
            "ruby" => Self::Ruby,
            "ruby-annotation" => Self::RubyAnnotation,
            "scroll-bar" => Self::ScrollBar,
            "scroll-view" => Self::ScrollView,
            "search" => Self::Search,
            "section" => Self::Section,
            "slider" => Self::Slider,
            "spin-button" => Self::SpinButton,
            "splitter" => Self::Splitter,
            "status" => Self::Status,
            "strong" => Self::Strong,
            "suggestion" => Self::Suggestion,
            "svg-root" => Self::SvgRoot,
            "tab" => Self::Tab,
            "tab-list" => Self::TabList,
            "tab-panel" => Self::TabPanel,
            "term" => Self::Term,
            "time" => Self::Time,
            "timer" => Self::Timer,
            "title-bar" => Self::TitleBar,
            "toolbar" => Self::Toolbar,
            "tooltip" => Self::Tooltip,
            "tree" => Self::Tree,
            "tree-grid" => Self::TreeGrid,
            "video" => Self::Video,
            "web-view" => Self::WebView,
            "window" => Self::Window,
            "pdf-actionable-highlight" => Self::PdfActionableHighlight,
            "pdf-root" => Self::PdfRoot,
            "graphics-document" => Self::GraphicsDocument,
            "graphics-object" => Self::GraphicsObject,
            "graphics-symbol" => Self::GraphicsSymbol,
            "doc-abstract" => Self::DocAbstract,
            "doc-acknowledgements" => Self::DocAcknowledgements,
            "doc-afterword" => Self::DocAfterword,
            "doc-appendix" => Self::DocAppendix,
            "doc-back-link" => Self::DocBackLink,
            "doc-biblio-entry" => Self::DocBiblioEntry,
            "doc-bibliography" => Self::DocBibliography,
            "doc-biblio-ref" => Self::DocBiblioRef,
            "doc-chapter" => Self::DocChapter,
            "doc-colophon" => Self::DocColophon,
            "doc-conclusion" => Self::DocConclusion,
            "doc-cover" => Self::DocCover,
            "doc-credit" => Self::DocCredit,
            "doc-credits" => Self::DocCredits,
            "doc-dedication" => Self::DocDedication,
            "doc-endnote" => Self::DocEndnote,
            "doc-endnotes" => Self::DocEndnotes,
            "doc-epigraph" => Self::DocEpigraph,
            "doc-epilogue" => Self::DocEpilogue,
            "doc-errata" => Self::DocErrata,
            "doc-example" => Self::DocExample,
            "doc-footnote" => Self::DocFootnote,
            "doc-foreword" => Self::DocForeword,
            "doc-glossary" => Self::DocGlossary,
            "doc-gloss-ref" => Self::DocGlossRef,
            "doc-index" => Self::DocIndex,
            "doc-introduction" => Self::DocIntroduction,
            "doc-note-ref" => Self::DocNoteRef,
            "doc-notice" => Self::DocNotice,
            "doc-page-break" => Self::DocPageBreak,
            "doc-page-footer" => Self::DocPageFooter,
            "doc-page-header" => Self::DocPageHeader,
            "doc-page-list" => Self::DocPageList,
            "doc-part" => Self::DocPart,
            "doc-preface" => Self::DocPreface,
            "doc-prologue" => Self::DocPrologue,
            "doc-pullquote" => Self::DocPullquote,
            "doc-qna" => Self::DocQna,
            "doc-subtitle" => Self::DocSubtitle,
            "doc-tip" => Self::DocTip,
            "doc-toc" => Self::DocToc,
            "list-grid" => Self::ListGrid,
            "terminal" => Self::Terminal,
            _ => Err(ParseError)?,
        })
    }
}

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
