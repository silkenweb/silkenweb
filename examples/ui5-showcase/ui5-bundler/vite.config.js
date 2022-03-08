const path = require('path')
const { defineConfig } = require('vite')

module.exports = defineConfig({
    build: {
        lib: {
            entry: path.resolve(__dirname, 'main.js'),
            name: 'UI5',
            fileName: (format) => `ui5.${format}.js`,
            formats: ['iife'],
        },
    }
})
