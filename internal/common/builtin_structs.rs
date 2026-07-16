// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR LicenseRef-Slint-Software-3.0

//! This module contains all builtin structures exposed in the .slint language.

use alloc::string::String;

/// A field default value declared in [`for_each_builtin_structs!`](crate::for_each_builtin_structs),
/// classified from its `stringify!`-ed tokens. Each consumer renders it for its target language.
#[derive(Debug, Clone, PartialEq)]
pub enum BuiltinStructFieldDefault {
    Bool(bool),
    /// A number literal; `text` preserves the declared spelling, with whitespace stripped
    Number {
        value: f64,
        text: String,
    },
    /// An enum value `Enum::Variant`; the variant keeps its Rust casing,
    /// with any `r#` prefix removed
    EnumValue {
        enum_name: String,
        variant: String,
    },
}

/// Classify the `stringify!`-ed tokens of a field default declared in
/// [`for_each_builtin_structs!`](crate::for_each_builtin_structs).
///
/// Panics on anything outside the supported subset (number literals, bool literals,
/// and enum values): that is a programmer error in the table in this file, and the
/// panic fails the offending consumer's build script, test run, or code generator.
pub fn parse_builtin_struct_field_default(tokens: &str) -> BuiltinStructFieldDefault {
    // Raw tokens stringify with spaces, such as "- 1.0" or "SortOrder :: Unsorted"
    let text: String = tokens.chars().filter(|c| !c.is_whitespace()).collect();
    match text.as_str() {
        "true" => return BuiltinStructFieldDefault::Bool(true),
        "false" => return BuiltinStructFieldDefault::Bool(false),
        _ => (),
    }
    if let Some((enum_name, variant)) = text.split_once("::") {
        return BuiltinStructFieldDefault::EnumValue {
            enum_name: enum_name.into(),
            variant: variant.trim_start_matches("r#").into(),
        };
    }
    let value = text.parse::<f64>().unwrap_or_else(|_| {
        panic!(
            "unsupported builtin struct field default `{tokens}` \
            (only numbers, bools, and enum values)"
        )
    });
    BuiltinStructFieldDefault::Number { value, text }
}

/// Maps the optional `$(= $field_default:expr)?` capture of a
/// [`for_each_builtin_structs!`](crate::for_each_builtin_structs) consumer to an `Option`
/// of the default's `stringify!`-ed tokens, for
/// [`parse_builtin_struct_field_default`](crate::builtin_structs::parse_builtin_struct_field_default):
/// `builtin_struct_field_default_tokens!($($field_default)?)`.
#[macro_export]
macro_rules! builtin_struct_field_default_tokens {
    () => {
        None
    };
    ($field_default:expr) => {
        Some(stringify!($field_default))
    };
}

/// Call a macro with every builtin structures exposed in the .slint language
///
/// Each struct is declared with `pub struct` if it should be re-exported in a public
/// language-binding module (e.g. `slint::language` in the Rust crate), or plain `struct`
/// to stay private. Consumers can dispatch on `$vis:vis`.
///
/// A field can declare a default value with `= expression` after its type.
/// The expression is limited to number literals, bool literals, and enum values,
/// because the consumers translate it to every target language.
/// Fields without a default value default to the zero value of their type.
/// Consumers capture the default with
/// [`builtin_struct_field_default_tokens!`](crate::builtin_struct_field_default_tokens)
/// and classify its tokens with
/// [`parse_builtin_struct_field_default`](crate::builtin_structs::parse_builtin_struct_field_default),
/// so a malformed declaration fails in every consumer's build instead of producing garbage output.
///
/// ## Example
/// ```rust
/// macro_rules! print_builtin_structs {
///     ($(
///         $(#[$struct_attr:meta])*
///         $vis:vis struct $Name:ident {
///             $( $(#[$field_attr:meta])* $field:ident : $field_type:ty $(= $field_default:expr)?, )*
///         }
///     )*) => {
///         $(println!("{} ({}) => [{}]", stringify!($Name), stringify!($vis), stringify!($($field),*));)*
///     };
/// }
/// i_slint_common::for_each_builtin_structs!(print_builtin_structs);
/// ```
#[macro_export]
macro_rules! for_each_builtin_structs {
    ($macro:ident) => {
        $macro! {
            /// The `KeyboardModifiers` struct provides booleans to indicate possible modifier keys on a keyboard, such as Shift, Control, etc.
            /// It is provided as part of `KeyEvent`'s `modifiers` field.
            ///
            /// Keyboard shortcuts on Apple platforms typically use the Command key (⌘), such as Command+C for "Copy". On other platforms
            /// the same shortcut is typically represented using Control+C. To make it easier to develop cross-platform applications, on macOS,
            /// Slint maps the Command key to the control modifier, and the Control key to the meta modifier.
            ///
            /// On Windows, the Windows key is mapped to the meta modifier.
            #[non_exhaustive]
            #[derive(Copy, Eq)]
            pub struct KeyboardModifiers {
                /// Indicates the Alt key on a keyboard.
                alt: bool,
                /// Indicates the Control key on a keyboard, except on macOS, where it is the Command key (⌘).
                control: bool,
                /// Indicates the Shift key on a keyboard.
                shift: bool,
                /// Indicates the Control key on macos, and the Windows key on Windows.
                meta: bool,
            }

            /// Represents a Pointer event sent by the windowing system.
            /// This structure is passed to the `pointer-event` callback of the `TouchArea` element.
            #[non_exhaustive]
            pub struct PointerEvent {
                /// The button that was pressed or released
                button: PointerEventButton,
                /// The kind of the event
                kind: PointerEventKind,
                /// The keyboard modifiers pressed during the event
                modifiers: KeyboardModifiers,
                /// The unique ID of the touch point, indicating the finger ID. 0 means it's not a touch event (e.g., mouse).
                touch_finger_id: i32,
            }

            /// Represents a Pointer scroll (or wheel) event sent by the windowing system.
            /// This structure is passed to the `scroll-event` callback of the `TouchArea` element.
            #[non_exhaustive]
            pub struct PointerScrollEvent {
                /// The amount of pixel in the horizontal direction
                delta_x: Coord,
                /// The amount of pixel in the vertical direction
                delta_y: Coord,
                /// The keyboard modifiers pressed during the event
                modifiers: KeyboardModifiers,
            }

            /// This structure is generated and passed to the key press and release callbacks of the `FocusScope` element.
            #[non_exhaustive]
            pub struct KeyEvent {
                /// The unicode representation of the key pressed.
                text: SharedString,
                /// The keyboard modifiers active at the time of the key press event.
                modifiers: KeyboardModifiers,
                /// This field is set to true for key press events that are repeated,
                /// i.e. the key is held down. It's always false for key release events.
                repeat: bool,
            }

            /// This structure is passed to the callbacks of the `DropArea` element
            #[non_exhaustive]
            pub struct DropEvent {
                /// The payload set on the source `DragArea`.
                data: DataTransfer,

                /// The cursor position in the `DropArea`'s local coordinates.
                position: LogicalPosition,

                /// The action negotiated from current modifier state, clamped to the allowed set;
                /// when no modifier is pressed, the first allowed of move, copy, link.
                /// Updated on every `DragMove`. The target's `can-drop` callback can return this
                /// to honor the user's modifier choice, or override with any other allowed action.
                proposed_action: DragAction,
            }

            /// Represents an item in a StandardListView and a StandardTableView.
            #[non_exhaustive]
            pub struct StandardListViewItem {
                /// The text content of the item
                text: SharedString,
            }

            /// This is used to define the column and the column header of a TableView
            #[non_exhaustive]
            pub struct TableColumn {
                /// The title of the column header
                title: SharedString,
                /// The minimum column width (logical length)
                min_width: Coord,
                /// The horizontal column stretch
                horizontal_stretch: f32,
                /// Sorts the column
                sort_order: SortOrder,
                /// the actual width of the column (logical length)
                width: Coord,
            }

            /// A structure to hold metrics of a font for a specified pixel size.
            struct FontMetrics {
                /// The distance between the baseline and the top of the tallest glyph in the font.
                ascent: Coord,
                /// The distance between the baseline and the bottom of the tallest glyph in the font.
                /// This is usually negative.
                descent: Coord,
                /// The distance between the baseline and the horizontal midpoint of the tallest glyph in the font,
                /// or zero if not specified by the font.
                x_height: Coord,
                /// The distance between the baseline and the top of a regular upper-case glyph in the font,
                /// or zero if not specified by the font.
                cap_height: Coord,
            }

            /// An item in the menu of a menu bar or context menu
            struct MenuEntry {
                /// The text of the menu entry
                title: SharedString,
                /// the icon associated with the menu entry
                icon: Image,
                /// an opaque id that can be used to identify the menu entry
                id: SharedString,
                // keys: KeySequence,
                /// whether the menu entry is enabled
                enabled: bool,
                /// whether the menu entry is checkable
                checkable: bool,
                /// whether the menu entry is checked
                checked: bool,
                /// Sub menu
                has_sub_menu: bool,
                /// The menu entry is a separator
                is_separator: bool,
                /// The shortcut keys
                shortcut: Keys,
            }

            /// A structure representing the four edges of an axis-aligned rectangle
            struct Edges {
                /// The left edge value
                left: Coord,
                /// The top edge value
                top: Coord,
                /// The right edge value
                right: Coord,
                /// The bottom edge value
                bottom: Coord,
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn parse_field_default() {
        // stringify! of raw tokens inserts spaces
        assert_eq!(
            parse_builtin_struct_field_default("- 1.5"),
            BuiltinStructFieldDefault::Number { value: -1.5, text: "-1.5".to_string() }
        );
        assert_eq!(
            parse_builtin_struct_field_default("false"),
            BuiltinStructFieldDefault::Bool(false)
        );
        assert_eq!(
            parse_builtin_struct_field_default("SortOrder :: r#Unsorted"),
            BuiltinStructFieldDefault::EnumValue {
                enum_name: "SortOrder".to_string(),
                variant: "Unsorted".to_string()
            }
        );
    }

    #[test]
    #[should_panic(expected = "unsupported builtin struct field default")]
    fn parse_field_default_rejects_other_expressions() {
        parse_builtin_struct_field_default("1 + 2");
    }
}
