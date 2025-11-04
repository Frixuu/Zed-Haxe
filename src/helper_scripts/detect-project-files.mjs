// SPDX-License-Identifier: MIT
import { readdir } from "node:fs/promises";
import process, { argv, chdir, stderr, stdout } from "node:process";

const workspacePath = argv[1]; // Node strips everything before "--"
chdir(workspacePath);

const SCORES = [
    { sub: "include", delta: -50 },
    { sub: "test", delta: -10 },
    { sub: "debug", delta: -10 },
    { sub: "sample", delta: -5 },
    { sub: "build", delta: 30 },
    { sub: "compile", delta: 30 },
    { sub: "main", delta: 50 },
    { sub: "lsp", delta: 100 },
    { sub: "devenv", delta: 100 },
];

const entries = await readdir(workspacePath, { withFileTypes: true });
const files = entries.filter((e) => e.isFile()).map((e) => e.name);

const hxmlPaths = [];
for (const relativePath of files.filter((p) =>
    p.toLowerCase().endsWith(".hxml")
)) {
    let score = 0;
    const pathLowerCase = relativePath.toLowerCase();
    if (pathLowerCase === "extraparams.hxml") {
        continue;
    }
    for (const { sub, delta } of SCORES) {
        if (pathLowerCase.includes(sub)) {
            score += delta;
        }
    }
    hxmlPaths.push({ relativePath, score });
}

// Calling `process.exit` is discouraged as it may not flush stdout writes
process.exitCode = (() => {
    hxmlPaths.sort((a, b) => b.score - a.score);
    if (hxmlPaths.length >= 1) {
        stdout.write(hxmlPaths[0].relativePath, "utf8");
        return 101;
    }

    const projectXml = files.find((p) => p.toLowerCase() === "project.xml");
    if (projectXml) {
        stdout.write(projectXml, "utf8");
        return 102;
    }

    const ceramicYml = files.find((p) => p.toLowerCase() === "ceramic.yml");
    if (ceramicYml) {
        stdout.write(ceramicYml, "utf8");
        return 103;
    }

    const nmml = files.find((p) => p.toLowerCase().endsWith(".nmml"));
    if (nmml) {
        stdout.write(nmml, "utf8");
        return 104;
    }

    return 0;
})();

stderr.end();
stdout.end();
