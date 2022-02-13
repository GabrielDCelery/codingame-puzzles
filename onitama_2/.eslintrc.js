module.exports = {
    parser: '@typescript-eslint/parser',
    env: {
        node: true,
        es6: true,
        jest: true,
    },
    extends: ['plugin:@typescript-eslint/recommended', 'plugin:prettier/recommended', 'prettier'],
    parserOptions: {
        ecmaVersion: 2018,
        sourceType: 'module',
    },
    plugins: ['@typescript-eslint', 'prettier', 'import'],
    rules: {},
    settings: {
        'import/resolver': {
            typescript: {},
        },
    },
    'prettier/prettier': [
        'error',
        {
            endOfLine: 'auto',
        },
    ],
};
