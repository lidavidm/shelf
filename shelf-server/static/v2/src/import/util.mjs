// Copyright 2020 David Li <li.davidm96@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
//     Unless required by applicable law or agreed to in writing, software
//     distributed under the License is distributed on an "AS IS" BASIS,
//     WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//     See the License for the specific language governing permissions and
//     limitations under the License.

export async function proxy(url, options = {}) {
    const params = {
        url,
    };
    if (options.referrer) {
        params.referrer = options.referrer;
    }
    // Inject cookies
    const cookies = JSON.parse(window.localStorage["cookies"] || "{}");
    const urlComponents = new URL(url);
    params.cookies = cookies[urlComponents.hostname] || {};
    return await window.fetch("/proxy", {
        method: "POST",
        body: JSON.stringify(params),
        headers: {
            "Content-Type": "application/json",
        },
    });
}

export async function defaultProxy(url, options = {}) {
    const req = await proxy(url, options);
    return await req.text();
}

export function langCodeToName(code) {
    switch (code.toLowerCase()) {
        case "en":
        case "en_us":
            return "English";
        case "en_jp":
            return "Japanese (Romaji)";
        case "ja":
        case "ja_jp":
            return "Japanese";
        default:
            return code;
    }
}

export function titleToKey(title) {
    return title
        .toLowerCase()
        .trim()
        .replace(/[^ a-z0-9]/g, "")
        .replace(/ +/g, "-");
}
