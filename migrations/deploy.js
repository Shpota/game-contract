const anchor = require("@project-serum/anchor");

module.exports = async function (provider) {
  anchor.setProvider(provider);
};