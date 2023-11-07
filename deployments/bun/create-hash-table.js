const fs = require("fs");
const path = require("path");
const cp = require("child_process");

function writeIfNotChanged(file, contents) {
  if (Array.isArray(contents)) contents = contents.join("");

  if (fs.existsSync(file)) {
    const oldContents = fs.readFileSync(file, "utf8");
    if (oldContents === contents) {
      return;
    }
  }

  try {
    fs.writeFileSync(file, contents);
  } catch (error) {
    fs.mkdirSync(path.dirname(file), { recursive: true });
    fs.writeFileSync(file, contents);
  }
}

const input = process.argv[2];
const output = process.argv[3];

const create_hash_table = path.join(__dirname, "./create_hash_table");

const main = async () => {
    const { stdout } = cp.spawnSync(create_hash_table, [input], {
        stdio: "pipe",
    });

    let str = await new Response(stdout).text();

    str = str.replaceAll(/^\/\/.*$/gm, "");
    str = str.replaceAll(/^#include.*$/gm, "");
    str = str.replaceAll(`namespace JSC {`, "");
    str = str.replaceAll(`} // namespace JSC`, "");
    str = "// File generated via `static-hash-table.ts`\n" + str.trim() + "\n";

    writeIfNotChanged(output, str);
};

main();
