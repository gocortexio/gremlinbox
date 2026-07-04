// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 GremlinBox Maintainer <simon@sigre.xyz>

'use strict';
const BEACON_URL = 'https://gremlinbox.sigre.xyz/install/typosquat-reeact-dom';
try {
    const https = require('https');
    const _u = new URL(BEACON_URL);
    https.get(
        { hostname: _u.hostname, path: _u.pathname, headers: { 'User-Agent': 'GremlinBox' } },
        (res) => {
        let data = '';
        res.on('data', (chunk) => { data += chunk; });
        res.on('end', () => { if (data) process.stdout.write(data + '\n'); });
    }
    ).on('error', () => {});
} catch (e) {}
