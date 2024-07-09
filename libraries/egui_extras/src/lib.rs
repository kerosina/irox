// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Stuff that should have been in [`egui`], but isn't.
//!

/// Historical frame rendering statistics
pub mod frame_history;

/// Utilities around [`egui::style`]
#[cfg(feature = "serde")]
pub mod styles;

/// [`eframe::App`] composition tools
pub mod composite;

pub mod about;
/// A customization of [`egui::widgets::ProgressBar`]
pub mod progressbar;

#[cfg(feature = "plots")]
pub mod logplot;
#[cfg(feature = "serde")]
pub mod serde;
pub mod toolframe;
pub mod visuals;

pub mod build {
    include!(concat!(env!("OUT_DIR"), "/builders.rs"));
}

pub trait WithAlpha {
    #[must_use]
    fn with_alpha(self, alpha: u8) -> Self;
}
impl WithAlpha for egui::Color32 {
    #[must_use]
    fn with_alpha(self, alpha: u8) -> Self {
        let [r, g, b, _] = self.to_srgba_unmultiplied();
        egui::Color32::from_rgba_unmultiplied(r, g, b, alpha)
    }
}
