async function log(msg) {
  Deno.core.opSync("log", msg);

}

async function main() {
  Promise.resolve().then(() => log("hello x"));
  await log("new way new life");
}

main();