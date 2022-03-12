const path = require('path')
const { defineConfig } = require('vite')

module.exports = defineConfig({
    // TODO: This is a workaround to enable minification for ES builds.
    // See:
    // - <https://github.com/vitejs/vite/issues/6555>
    // - <https://github.com/vitejs/vite/pull/6670>
    esbuild: {
        minify: true,
    },
    build: {
        // TODO: One of the proposed solutions to minifying ES builds is to
        // default `minify`to false, then allow us to opt-in.
        minify: true,
        lib: {
            entry: path.resolve(__dirname, 'main.js'),
            name: 'UI5',
            fileName: (format) => `ui5.${format}.js`,
            formats: ['esm'],
        },
    }
})
