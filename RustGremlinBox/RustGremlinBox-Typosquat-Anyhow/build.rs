// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 GremlinBox Maintainer <simon@sigre.xyz>
//
// Install-time beacon for supply chain security tabletop exercises.
// Contacts a gremlinbox.sigre.xyz endpoint at build time (build.rs runs on
// `cargo build` / `cargo install`, including when built as a dependency).
// The beacon GET response is surfaced to the build console via cargo:warning,
// mirroring the Python/npm install hooks which print the response to stdout.
// No data is collected beyond a standard HTTP server log entry. The request
// fails silently and never fails the build.

const BEACON_URL: &str = "https://gremlinbox.sigre.xyz/install/typosquat-anyhwo-error";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if let Ok(response) = minreq::get(BEACON_URL)
        .with_header("User-Agent", "GremlinBox")
        .with_timeout(5)
        .send()
    {
        if let Ok(body) = response.as_str() {
            for line in body.lines() {
                println!("cargo:warning={}", line);
            }
        }
    }
}
