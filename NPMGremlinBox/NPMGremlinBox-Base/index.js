// SPDX-License-Identifier: GPL-3.0
// SPDX-FileCopyrightText: 2025 GremlinBox Maintainer <simon@sigre.xyz>

'use strict';

const VERSION = '2.0.1';
const PROJECT_NAME = 'GremlinBox';
const REPOSITORY = 'https://github.com/gocortexio/gremlinbox';
const DESCRIPTION = 'GremlinBox: multi-language supply chain security testing package collection. '
    + 'See https://github.com/gocortexio/gremlinbox for the full project.';

function getPackageMetadata() {
    return {
        package_name: '@gremlinbox/base',
        version: VERSION,
        licence: 'GPL-3.0',
        description: DESCRIPTION,
        repository: REPOSITORY
    };
}

module.exports = { getPackageMetadata, VERSION, PROJECT_NAME, REPOSITORY, DESCRIPTION };
