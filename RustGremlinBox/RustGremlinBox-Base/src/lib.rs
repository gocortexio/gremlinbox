// SPDX-License-Identifier: GPL-3.0
// SPDX-FileCopyrightText: 2025 GremlinBox Maintainer <simon@sigre.xyz>

//! GremlinBox base squatter crate. Occupies the `gremlinbox` name on
//! crates.io and points to the full multi-ecosystem project.

use std::collections::BTreeMap;

pub const VERSION: &str = "2.0.1";
pub const PROJECT_NAME: &str = "GremlinBox";
pub const REPOSITORY: &str = "https://github.com/gocortexio/gremlinbox";
pub const DESCRIPTION: &str = "GremlinBox: multi-language supply chain security testing package collection. See https://github.com/gocortexio/gremlinbox for the full project.";

/// Return project metadata for this base squatter crate.
pub fn get_package_metadata() -> BTreeMap<&'static str, String> {
    let mut m = BTreeMap::new();
    m.insert("package_name", "gremlinbox".to_string());
    m.insert("version", VERSION.to_string());
    m.insert("licence", "GPL-3.0".to_string());
    m.insert("description", DESCRIPTION.to_string());
    m.insert("repository", REPOSITORY.to_string());
    m
}
