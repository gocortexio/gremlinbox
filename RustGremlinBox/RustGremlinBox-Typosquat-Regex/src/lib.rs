// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 GremlinBox Maintainer <simon@sigre.xyz>

//! GremlinBox typosquatting package: targets the crates.io crate `regex`.
//!
//! Published under the deliberate misspelling `reqex-match` to test
//! name-proximity detection. An install-time beacon is fired from build.rs.
//! See https://github.com/gocortexio/gremlinbox.

use std::collections::BTreeMap;

pub const LICENCE_IDENTIFIER: &str = "AGPL-3.0-or-later";
pub const PACKAGE_NAME: &str = "reqex-match";
pub const VERSION: &str = "2.0.1";
pub const INSTALL_BEACON: &str = "https://gremlinbox.sigre.xyz/install/typosquat-reqex-match";
pub const TYPOSQUAT_TARGET: &str = "regex";
pub const TYPOSQUAT_VARIANT: &str = "reqex-match";

/// Return the SPDX licence identifier for this package.
pub fn get_licence_identifier() -> &'static str {
    LICENCE_IDENTIFIER
}

/// Return the full text of the licence.
pub fn retrieve_licence_content() -> &'static str {
    include_str!("../LICENCE")
}

/// Return the name of the legitimate crate being targeted.
pub fn get_typosquat_target() -> &'static str {
    TYPOSQUAT_TARGET
}

/// Return the typosquat name under which this crate is published.
pub fn get_typosquat_variant() -> &'static str {
    TYPOSQUAT_VARIANT
}

/// Return package metadata (name, version, licence, target, variant).
pub fn get_package_metadata() -> BTreeMap<&'static str, String> {
    let mut m = BTreeMap::new();
    m.insert("package_name", PACKAGE_NAME.to_string());
    m.insert("version", VERSION.to_string());
    m.insert("licence", LICENCE_IDENTIFIER.to_string());
    m.insert("typosquat_target", TYPOSQUAT_TARGET.to_string());
    m.insert("typosquat_variant", TYPOSQUAT_VARIANT.to_string());
    m.insert("install_beacon", INSTALL_BEACON.to_string());
    m.insert("install_hook", "build.rs".to_string());
    m
}
