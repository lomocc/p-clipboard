const addon = require("../native");

function promisify(fn) {
  return function() {
    return new Promise(resolve => {
      resolve(fn.apply(null, arguments));
    });
  };
}
function promisifyAll(target = {}, suffix = "Async") {
  const result = Object.assign({}, target);
  Object.entries(target).forEach(([name, fn]) => {
    if (typeof fn === "function") {
      result[name + suffix] = promisify(fn);
    }
  });
  return result;
}

module.exports = promisifyAll(addon);
