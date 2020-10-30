import test from "ava";

import * as util from "./util.mjs";

test("titleToKey", t => {
    t.deepEqual(util.titleToKey("Jitsu wa Watashi wa!"), "jitsu-wa-watashi-wa");
    t.deepEqual(util.titleToKey("39 Music!"), "39-music");
    t.deepEqual(util.titleToKey(" white  space  "), "white-space");
});
