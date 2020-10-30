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

export async function defaultProxy(url) {
    const req = await window.fetch(`/proxy?url=${window.encodeURIComponent(url)}`);
    const resp = await req.text();
    return resp;
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
    return title.toLowerCase().trim().replace(/[^ a-z0-9]/g, "").replace(/ +/g, "-");
}
