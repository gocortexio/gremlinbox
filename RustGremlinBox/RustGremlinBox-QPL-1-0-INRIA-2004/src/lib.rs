// SPDX-License-Identifier: QPL-1.0-INRIA-2004
// SPDX-FileCopyrightText: 2025 GremlinBox Maintainer <simon@sigre.xyz>

//! GremlinBox licence package: Q Public Licence 1.0 - INRIA 2004 variant
//!
//! Supply chain security testing corpus. This crate implements a single
//! SPDX-compatible licence for testing licence-detection and policy tools.
//! See https://github.com/gocortexio/gremlinbox for the full project.

use std::collections::BTreeMap;

pub const LICENCE_IDENTIFIER: &str = "QPL-1.0-INRIA-2004";
pub const PACKAGE_NAME: &str = "gremlinbox-qpl-1-0-inria-2004";
pub const VERSION: &str = "2.0.1";

/// Return the SPDX licence identifier for this package.
pub fn get_licence_identifier() -> &'static str {
    LICENCE_IDENTIFIER
}

/// Return the full text of the licence.
pub fn retrieve_licence_content() -> &'static str {
    include_str!("../LICENCE")
}

/// Return package metadata (name, version, licence).
pub fn get_package_metadata() -> BTreeMap<&'static str, String> {
    let mut m = BTreeMap::new();
    m.insert("package_name", PACKAGE_NAME.to_string());
    m.insert("version", VERSION.to_string());
    m.insert("licence", LICENCE_IDENTIFIER.to_string());
    m.insert("spdx_licence_id", LICENCE_IDENTIFIER.to_string());
    m
}
