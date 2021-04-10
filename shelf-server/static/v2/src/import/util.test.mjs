import test from "ava";

import * as util from "./util.mjs";

test("titleToKey", (t) => {
    t.deepEqual(util.titleToKey("Jitsu wa Watashi wa!"), "jitsu-wa-watashi-wa");
    t.deepEqual(util.titleToKey("39 Music!"), "39-music");
    t.deepEqual(util.titleToKey(" white  space  "), "white-space");
});

test("titleToKey handles Unicode", (t) => {
    t.deepEqual(util.titleToKey("実は私は…"), "shi-ha-si-ha");
    t.deepEqual(util.titleToKey("陈心云"), "chenxinyun");
});
