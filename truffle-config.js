const path = require("path");
module.exports = {
  contracts_build_directory: path.join(__dirname, "client/src/contracts"),
  networks: {
    development: {
      host: "127.0.0.1",
      port: 8545,
      network_id: "*",
      websockets: true,
    },
    dashboard: {
      port: 24012
    }
  },
  mocha: {},
  compilers: {
    solc: {
      version: "0.8.15"
    }
  },
  db: {
    enabled: false
  }
};
