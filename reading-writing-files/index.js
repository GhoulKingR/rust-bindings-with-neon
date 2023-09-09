const mod = require(".");
if (mod.writeFile()) {
  console.log(mod.readFile());
} else {
  console.log("couldn't read file");
}
