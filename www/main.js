import * as monaco from 'monaco-editor';
import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker';
import jsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker';
import cssWorker from 'monaco-editor/esm/vs/language/css/css.worker?worker';
import htmlWorker from 'monaco-editor/esm/vs/language/html/html.worker?worker';
import tsWorker from 'monaco-editor/esm/vs/language/typescript/ts.worker?worker';

import init, { compile_to_jsfuck } from './pkg/jsfuck_compiler.js';

// Monaco Worker Environment Setup
self.MonacoEnvironment = {
    getWorker(_, label) {
        if (label === 'json') {
            return new jsonWorker();
        }
        if (label === 'css' || label === 'scss' || label === 'less') {
            return new cssWorker();
        }
        if (label === 'html' || label === 'handlebars' || label === 'razor') {
            return new htmlWorker();
        }
        if (label === 'typescript' || label === 'javascript') {
            return new tsWorker();
        }
        return new editorWorker();
    }
};

let inputEditor, outputEditor;

async function start() {
    const statusEl = document.getElementById('status');
    const loaderEl = document.getElementById('loader');
    const compileBtn = document.getElementById('compile-btn');
    const runBtn = document.getElementById('run-btn');

    loaderEl.style.display = 'inline-block';

    try {
        // Initialize Wasm
        await init();
        console.log("Wasm loaded");

        // Initialize Editors
        inputEditor = monaco.editor.create(document.getElementById('editor-input'), {
            value: 'console.log("Hello from JSFuck!");\nconst a = 10;\nconst b = 20;\nalert(a + b);',
            language: 'javascript',
            theme: 'vs-dark',
            minimap: { enabled: false },
            automaticLayout: true
        });

        outputEditor = monaco.editor.create(document.getElementById('editor-output'), {
            value: '',
            language: 'javascript',
            theme: 'vs-dark',
            readOnly: true,
            wordWrap: 'on',
            minimap: { enabled: false },
            automaticLayout: true
        });

        // Activate UI
        statusEl.textContent = 'Ready';
        loaderEl.style.display = 'none';
        compileBtn.disabled = false;
        runBtn.disabled = false;

        // Events
        compileBtn.addEventListener('click', compile);
        runBtn.addEventListener('click', runCode);

    } catch (e) {
        console.error(e);
        statusEl.textContent = 'Error: ' + e.message;
        statusEl.style.color = '#ff5555';
    }
}

function compile() {
    const code = inputEditor.getValue();
    const minify = document.getElementById('minify-check').checked;
    const statusEl = document.getElementById('status');
    
    statusEl.textContent = 'Compiling...';
    // Use setTimeout to allow UI update
    setTimeout(() => {
        const start = performance.now();
        try {
            const result = compile_to_jsfuck(code, minify);
            outputEditor.setValue(result);
            const time = (performance.now() - start).toFixed(2);
            statusEl.textContent = `Done in ${time}ms (${result.length} bytes)`;
        } catch (e) {
            console.error(e);
            statusEl.textContent = 'Failed';
            outputEditor.setValue('Error: ' + e);
        }
    }, 10);
}

function runCode() {
    const code = outputEditor.getValue();
    if (!code) return;
    try {
        eval(code);
    } catch (e) {
        alert('Runtime Error: ' + e);
    }
}

start();