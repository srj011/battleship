import prettier from 'eslint-config-prettier';
import path from 'node:path';
import { includeIgnoreFile } from '@eslint/compat';
import js from '@eslint/js';
import svelte from 'eslint-plugin-svelte';
import { defineConfig } from 'eslint/config';
import globals from 'globals';
import ts from 'typescript-eslint';
import svelteConfig from './svelte.config.js';

const gitignorePath = path.resolve(import.meta.dirname, '.gitignore');

export default defineConfig(
    includeIgnoreFile(gitignorePath),
    js.configs.recommended,
    ts.configs.recommended,
    svelte.configs.recommended,
    prettier,
    svelte.configs.prettier,
    {
        languageOptions: { globals: { ...globals.browser, ...globals.node } },
        rules: {
            // ❌ TS handles this better
            'no-undef': 'off',

            // -------------------------
            // 🧹 Cleanliness (warnings)
            // -------------------------
            'no-unused-vars': 'off',
            '@typescript-eslint/no-unused-vars': [
                'warn',
                {
                    argsIgnorePattern: '^_',
                    varsIgnorePattern: '^_'
                }
            ],

            'prefer-const': 'warn',

            // -------------------------
            // 🧠 Dev flexibility
            // -------------------------
            'no-console': 'warn',
            'no-debugger': 'warn',

            // -------------------------
            // 🤝 TypeScript ergonomics
            // -------------------------
            '@typescript-eslint/no-explicit-any': 'warn',
            '@typescript-eslint/ban-ts-comment': 'warn',

            // -------------------------
            // 🧯 Turn OFF noisy rules
            // -------------------------
            '@typescript-eslint/explicit-function-return-type': 'off',
            '@typescript-eslint/explicit-module-boundary-types': 'off',

            // Svelte-specific noise reduction
            'svelte/no-at-html-tags': 'off'
        }
    },
    {
        files: ['**/*.svelte', '**/*.svelte.ts', '**/*.svelte.js'],
        languageOptions: {
            parserOptions: {
                projectService: true,
                extraFileExtensions: ['.svelte'],
                parser: ts.parser,
                svelteConfig
            }
        }
    }
);
