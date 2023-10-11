const { defineConfig } = require('cypress')

module.exports = defineConfig({
  fixturesFolder: false,
  e2e: {
    setupNodeEvents(on, config) {},
    baseUrl: 'http://127.0.0.1:8080',
    defaultCommandTimeout: 60000,
  },
})
