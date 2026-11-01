//! The launcher's view of Horizon's design tokens (`crates/horizon-theme`, described in
//! `docs/design-system.md`): the blocky look it shares with the website ("Blocks"), stone-dark or
//! lavender-light with Horizon's violet, Pixelify Sans for headings and controls, Geist and Geist
//! Mono for text (all bundled, SIL Open Font License).
use gpui::{rgb, Hsla, WindowAppearance};
use horizon_launcher::backend::Appearance;
use horizon_launcher::themes::Accent;
use horizon_theme as tokens;
use std::time::Duration;

pub use tokens::{Step, BODY, CAPTION, DISPLAY, HEADING, LABEL, MICRO, TITLE, UI, WORDMARK};
pub const FONT: &str = tokens::FONT;
pub const MONO: &str = tokens::MONO;
pub const PIXEL: &str = tokens::PIXEL;

pub use tokens::{BLOCK_BEVEL as BEVEL, BLOCK_LIP as LIP};
/// Shapes and controls, shared with the game's menus and the website.
pub use tokens::{CONTROL_LARGE, DISABLED, FIELD, PRESSED, RISE, ROW, SEGMENT};

/// Blocks have square corners: these stay for the places that ask for a radius.
pub const RADIUS: f32 = 0.;
pub const RADIUS_PLATE: f32 = 0.;
/// Buttons are a little taller than the quiet look's, for their lip.
pub const CONTROL: f32 = 34.;
/// The top bar, and the sidebar of instances and servers.
pub const TOPBAR: f32 = 56.;
pub const SIDEBAR: f32 = 248.;
pub const PAGE_GUTTER: f32 = 32.;

/// Motion: short and eased out, never bouncy.
pub const QUICK: Duration = tokens::QUICK.duration();
pub const MOVE: Duration = tokens::MOVE.duration();
pub const ENTER: Duration = tokens::ENTER.duration();
pub const STAGGER: Duration = tokens::STAGGER;

#[derive(Clone, Copy)]
pub struct Theme {
    /// The page behind everything (under its stone texture), and its colour as `0xRRGGBB` for
    /// the texture.
    pub page: Hsla,
    pub page_rgb: u32,
    /// Hover plates, fields and the console: the raised panel colour.
    pub plate_soft: Hsla,
    /// Pressed plates, a switch that is off: the button colour.
    pub plate: Hsla,
    /// Dialogs and menus above the page: the panel colour.
    pub surface: Hsla,
    pub rule: Hsla,
    pub ink: Hsla,
    pub ink_soft: Hsla,
    pub muted: Hsla,
    pub accent: Hsla,
    pub on_accent: Hsla,
    pub danger: Hsla,
    pub success: Hsla,
    pub warn: Hsla,
    /// The blocky parts (`horizon_theme::Blocks`).
    pub side: Hsla,
    pub panel: Hsla,
    pub raised: Hsla,
    pub button: Hsla,
    pub button_bevel: Hsla,
    pub well: Hsla,
    pub bevel: Hsla,
    pub lip: Hsla,
    pub outline: Hsla,
    pub face: Hsla,
    pub face_bevel: Hsla,
    pub face_lip: Hsla,
    pub on_face: Hsla,
    pub dark: bool,
}

fn c(hex: u32) -> Hsla {
    rgb(hex).into()
}

/// Whether the launcher is pinned dark (`Some(true)`) or light, or follows the system (`None`):
/// `HORIZON_LAUNCHER_APPEARANCE=light` or `dark` (for screenshots and tests), else the player's
/// choice in Settings.
pub fn pinned_dark(choice: Appearance) -> Option<bool> {
    match std::env::var("HORIZON_LAUNCHER_APPEARANCE").as_deref() {
        Ok("light") => Some(false),
        Ok("dark") => Some(true),
        _ => match choice {
            Appearance::Dark => Some(true),
            Appearance::Light => Some(false),
            Appearance::System => None,
        },
    }
}

impl Theme {
    /// The pinned look (`pinned_dark`: dark unless the player picked another), the system's when
    /// they asked to follow it, in the player's accent.
    pub fn for_appearance(choice: Appearance, accent: Accent, system: WindowAppearance) -> Self {
        let system_dark = !matches!(
            system,
            WindowAppearance::Light | WindowAppearance::VibrantLight
        );
        let dark = pinned_dark(choice).unwrap_or(system_dark);
        Self::from(accent.apply(coconut_blocks(dark)))
    }

    fn from(p: tokens::Blocks) -> Self {
        Self {
            page: c(p.page),
            page_rgb: p.page,
            plate_soft: c(p.raised),
            plate: c(p.button),
            surface: c(p.panel),
            rule: c(p.rule),
            ink: c(p.ink),
            ink_soft: c(p.ink_soft),
            muted: c(p.muted),
            accent: c(p.accent),
            on_accent: c(p.on_face),
            danger: c(p.danger),
            success: c(p.success),
            warn: c(p.warn),
            side: c(p.side),
            panel: c(p.panel),
            raised: c(p.raised),
            button: c(p.button),
            button_bevel: c(p.button_bevel),
            well: c(p.well),
            bevel: c(p.bevel),
            lip: c(p.lip),
            outline: c(p.outline),
            face: c(p.face),
            face_bevel: c(p.face_bevel),
            face_lip: c(p.face_lip),
            on_face: c(p.on_face),
            dark: p.dark,
        }
    }
}

fn coconut_blocks(dark: bool) -> tokens::Blocks {
    let mut p = if dark {
        tokens::BLOCKS_DARK
    } else {
        tokens::BLOCKS_LIGHT
    };
    if dark {
        p.page = 0x17130e;
        p.side = 0x100e0b;
        p.panel = 0x251e17;
        p.raised = 0x34291f;
        p.button = 0x3b2c20;
        p.button_bevel = 0x60432b;
        p.well = 0x110e0a;
        p.bevel = 0x58422d;
        p.lip = 0x090806;
        p.outline = 0x080705;
        p.rule = 0x1b150f;
        p.ink = 0xf8efd9;
        p.ink_soft = 0xd9cbb2;
        p.muted = 0xb6a588;
        p.accent = 0xa8df80;
    } else {
        p.page = 0xf4ead4;
        p.side = 0xe5d5b6;
        p.panel = 0xfff8e8;
        p.raised = 0xefe0bf;
        p.button = 0xe0cda8;
        p.button_bevel = 0xfff4d9;
        p.well = 0xf9efd9;
        p.bevel = 0xffffff;
        p.lip = 0xbba37e;
        p.outline = 0x3b2a1a;
        p.rule = 0xe6d4b3;
        p.ink = 0x291d13;
        p.ink_soft = 0x4b3825;
        p.muted = 0x68553b;
        p.accent = 0x356b21;
    }
    p.face = 0x3d762d;
    p.face_bevel = 0x70a64e;
    p.face_lip = 0x254c1c;
    p
}

/// How strongly a selected suggestion in a text field is tinted with the accent.
pub const SELECTION: f32 = 0.22;

#[cfg(test)]
mod tests {
    use super::{tokens, SELECTION};

    /// `top` at `alpha` over `under`, as the GPU blends them.
    fn blend(top: u32, under: u32, alpha: f32) -> u32 {
        let channel = |shift: u32| {
            let (a, b) = ((top >> shift & 0xff) as f32, (under >> shift & 0xff) as f32);
            ((a * alpha + b * (1. - alpha)).round() as u32) << shift
        };
        channel(16) | channel(8) | channel(0)
    }

    /// What the Servers page draws besides text (whose colours horizon-theme's own test holds to
    /// WCAG AA on every background): the keyboard focus ring, the borders of a focused or refused
    /// field (WCAG 1.4.11 asks 3:1 for those), and text over a selected suggestion.
    #[test]
    fn marks_and_selections_read_clearly() {
        for p in [tokens::BLOCKS_LIGHT, tokens::BLOCKS_DARK] {
            let theme = if p.dark { "dark" } else { "light" };
            for (name, bg) in [
                ("page", p.page),
                ("raised", p.raised),
                ("button", p.button),
                ("panel", p.panel),
                ("well", p.well),
            ] {
                let ring = tokens::contrast(p.accent, bg);
                assert!(ring >= 3.0, "{theme} focus ring on {name}: {ring:.2}");
                let refused = tokens::contrast(p.danger, bg);
                assert!(
                    refused >= 3.0,
                    "{theme} refused field on {name}: {refused:.2}"
                );
            }
            let selection = blend(p.accent, p.panel, SELECTION);
            let ratio = tokens::contrast(p.ink, selection);
            assert!(ratio >= 4.5, "{theme} text on a selection: {ratio:.2}");
        }
    }
}
