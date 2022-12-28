import { resolve } from 'path';
import { defineConfig } from 'vite';

module.exports = defineConfig({
    build: {
        lib: {
            entry: resolve(__dirname, 'main.js'),
            name: 'UI5',
            formats: ['iife'],
        },
    },
})
