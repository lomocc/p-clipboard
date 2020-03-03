// const { promisify } = require("util");
const clipboard = require("../lib/index.js");

clipboard.getFileListAsync().then(d => console.log("async", d));
console.log(clipboard.getFileList());
