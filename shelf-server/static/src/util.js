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

export function makeKey(kind, name) {
    return kind + "-" + (name
                         .replace(/[\W:]+/g, "-")
                         .replace(/^-+/, "")
                         .replace(/-+$/, "")
                         .toLowerCase());
}

/** Map an item kind to a user-friendly string. */
export function humanKind(kind) {
    switch (kind) {
    case "OVA":
        return "OVA (Original Video Animation)";
    case "ONA":
        return "ONA (Original Net Animation)";
    case "ShortStory":
        return "Short Story";
    case "VisualNovel":
        return "Visual Novel";
    case "Music":
        return "Music Video";
    default:
        return kind;
    }
}
