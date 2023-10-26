// Copyright 2023 the Druid Authors.
// SPDX-License-Identifier: Apache-2.0

//! An experimental library for making reactive SVG graphics.

mod app;
mod class;
mod clicked;
mod context;
mod group;
mod kurbo_shape;
mod pointer;
mod view;
mod view_ext;

pub use kurbo;

pub use app::App;
pub use context::Cx;
pub use group::group;
pub use kurbo_shape::KurboShape;
pub use pointer::{PointerDetails, PointerMsg};
pub use view::{AnyView, Memoize, View, ViewMarker, ViewSequence};

pub use context::ChangeFlags;

xilem_core::message!(Send);
