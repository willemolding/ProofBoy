const path = require("path");
module.exports = {
  contracts_build_directory: path.join(__dirname, "client/src/contracts"),
  networks: {
    development: {
      host: "127.0.0.1",
      port: 8545,
      network_id: "5777",
      websockets: true,
    },
    linea_testnet: {
      network_id: "59140",
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
  },
  plugins: [
    'truffle-plugin-stdjsonin'
  ]
};
