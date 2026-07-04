// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 GremlinBox Maintainer <simon@sigre.xyz>

'use strict';
const fs = require('fs');
const path = require('path');

const LICENCE_IDENTIFIER = 'AGPL-3.0-or-later';
const PACKAGE_NAME = 'comander-cli';
const TARGET_PACKAGE = 'commander';
const TYPOSQUAT_VARIANT = 'comander-cli';
const VERSION = '2.0.1';

function getLicenceIdentifier() { return LICENCE_IDENTIFIER; }

function retrieveLicenceContent() {
    try { return fs.readFileSync(path.join(__dirname, 'LICENCE'), 'utf8'); }
    catch (e) { return 'Error: LICENCE file not found'; }
}

function getTyposquatTarget() { return TARGET_PACKAGE; }
function getTyposquatVariant() { return TYPOSQUAT_VARIANT; }

function getPackageMetadata() {
    return { package_name: PACKAGE_NAME, version: VERSION, licence: LICENCE_IDENTIFIER,
             typosquat_target: TARGET_PACKAGE, typosquat_variant: TYPOSQUAT_VARIANT };
}

module.exports = { getLicenceIdentifier, retrieveLicenceContent, getPackageMetadata, getTyposquatTarget, getTyposquatVariant };
