const vscode = acquireVsCodeApi();

// Initialize Monaco Editor
require.config({ paths: { 'vs': 'https://unpkg.com/monaco-editor@0.30.1/min/vs' }});
require(['vs/editor/editor.main'], () => {
    // Initialize editors when Monaco is loaded
    let originalEditor = monaco.editor.create(document.getElementById('originalCodeContainer'), {
        value: '',
        language: 'rust',
        theme: 'vs-dark',
        readOnly: true
    });
    let newEditor = monaco.editor.create(document.getElementById('newCodeContainer'), {
        value: '',
        language: 'rust',
        theme: 'vs-dark',
        readOnly: true
    });

    // Update editors with initial content
    window.addEventListener('message', event => {
        const message = event.data;
        let newCode;
        switch (message.command) {
            case 'initialise':
                // Set the initial content for the LHS Editor
                originalEditor.setValue(message.data.originalCode);

                // Display a Temporary Message on the RHS Editor
                newCode = 'Press Preview button to view a preview of the Refactored Code';
                newEditor.setValue(newCode);

                // Initialize the Text Fields as well
                document.getElementById('option').textContent = message.data.option;
                document.getElementById('originalName').textContent = message.data.originalName;
                document.getElementById('newName').textContent = message.data.newName;
                break;
            case 'preview':
                // Set the initial content for the LHS Editor
                originalEditor.setValue(message.data.originalCode);

                // Display the Refactored Code on the RHS Editor
                newCode = message.data.newCode;
                newEditor.setValue(newCode);
                break;
            case 'showConfirmButton':
                document.getElementById('confirmButton').style.display = 'inline';
                break;
        }
    });

    document.getElementById('previewButton').addEventListener('click', () => {
        vscode.postMessage({ command: 'preview' });
    });

    document.getElementById('confirmButton').addEventListener('click', () => {
        vscode.postMessage({ command: 'confirm' });
        vscode.postMessage({ command: 'close' });
    });
});
