// SPDX-License-Identifier: CAL-1.0-Combined-Work-Exception
// SPDX-FileCopyrightText: 2025 GremlinBox Maintainer <simon@sigre.xyz>

'use strict';
const fs = require('fs');
const path = require('path');

const LICENCE_IDENTIFIER = 'CAL-1.0-Combined-Work-Exception';
const PACKAGE_NAME = '@gremlinbox/cal-1-0-combined-work-exception';
const VERSION = '2.0.1';

function getLicenceIdentifier() { return LICENCE_IDENTIFIER; }

function retrieveLicenceContent() {
    try { return fs.readFileSync(path.join(__dirname, 'LICENCE'), 'utf8'); }
    catch (e) { return 'Error: LICENCE file not found'; }
}

function getPackageMetadata() {
    return { package_name: PACKAGE_NAME, version: VERSION, licence: LICENCE_IDENTIFIER, spdx_licence_id: LICENCE_IDENTIFIER };
}

module.exports = { getLicenceIdentifier, retrieveLicenceContent, getPackageMetadata };
