import * as fs from "fs";
import * as path from "path";
import { fileURLToPath } from "url";

export function makeProxy(filename) {
    return function proxy(url) {
        const filepath = path.resolve(
            path.dirname(fileURLToPath(import.meta.url)),
            filename
        );
        return new Promise((resolve, reject) => {
            fs.readFile(filepath, "utf8", (err, data) => {
                if (err) {
                    reject(err);
                } else {
                    resolve(data);
                }
            });
        });
    };
}
