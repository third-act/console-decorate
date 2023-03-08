pub mod prelude {
    pub use super::{
        decorate, BG_BLUE, BG_GREEN, BG_GREY, BG_NONE, BG_PINK, BG_RED, BG_TEAL, BG_WHITE,
        BG_YELLOW, BLUE, BOLD, GREEN, GREY, ITALICS, PINK, RED, STRIKETHROUGH, TEAL, UNDERLINE,
        WHITE, YELLOW,
    };
}

pub const BOLD: &str = "1";
pub const UNDERLINE: &str = "4";
pub const ITALICS: &str = "3";
pub const STRIKETHROUGH: &str = "9";

pub const GREY: &str = "90";
pub const RED: &str = "91";
pub const GREEN: &str = "92";
pub const YELLOW: &str = "93";
pub const BLUE: &str = "94";
pub const PINK: &str = "95";
pub const TEAL: &str = "96";
pub const WHITE: &str = "97";

pub const BG_NONE: &str = "99";
pub const BG_GREY: &str = "100";
pub const BG_RED: &str = "101";
pub const BG_GREEN: &str = "102";
pub const BG_YELLOW: &str = "103";
pub const BG_BLUE: &str = "104";
pub const BG_PINK: &str = "105";
pub const BG_TEAL: &str = "106";
pub const BG_WHITE: &str = "107";

#[macro_export]
macro_rules! decorate {
    ($str:expr, $($constant:path),*) => {{
        let mut s = String::from("");
        $(
            s = format!("{}{}", s, format!("\x1b[{}m", $constant));
        )*
        format!("{}{}{}", s, $str, "\x1b[0m")
    }};
}
