// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import {
	languages,
	workspace,
	EventEmitter,
	ExtensionContext,
	window,
	InlayHintsProvider,
	TextDocument,
	CancellationToken,
	Range,
	InlayHint,
	TextDocumentChangeEvent,
	ProviderResult,
	commands,
	WorkspaceEdit,
	TextEdit,
	Selection,
	Uri,
} from "vscode";

import {
	Disposable,
	Executable,
	LanguageClient,
	LanguageClientOptions,
	ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient;

// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
export function activate(context: ExtensionContext) {

	// Use the console to output diagnostic information (console.log) and errors (console.error)
	// This line of code will only be executed once when your extension is activated
	console.log('Congratulations, your extension "tyd-language-client" is now active!');

	// The command has been defined in the package.json file
	// Now provide the implementation of the command with registerCommand
	// The commandId parameter must match the command field in package.json
	let disposable = commands.registerCommand('tyd-language-client.helloWorld', () => {
		// The code you place here will be executed every time your command is executed
		// Display a message box to the user
		window.showInformationMessage('Hello World from tyd-language-client!');
	});

	context.subscriptions.push(disposable);

	const traceOutputChannel = window.createOutputChannel("TypeDown Language Server trace");
	const command = process.env.SERVER_PATH || "tyd-language-server";
	const run: Executable = {
		command,
		options: {
			env: {
				...process.env,
				RUST_LOG: "debug",
			},
		},
	};
	const serverOptions: ServerOptions = {
		run,
		debug: run,
	};
	// If the extension is launched in debug mode then the debug server options are used
	// Otherwise the run options are used
	// Options to control the language client
	let clientOptions: LanguageClientOptions = {
		// Register the server for plain text documents
		documentSelector: [{ scheme: "file", language: "tyd" }],
		synchronize: {
			// Notify the server about file changes to '.clientrc files contained in the workspace
			fileEvents: workspace.createFileSystemWatcher("**/.clientrc"),
		},
		traceOutputChannel,
	};

	// Create the language client and start the client.
	client = new LanguageClient("tyd-language-server", "tyd language server", serverOptions, clientOptions);
	// activateInlayHints(context);
	client.start();
}

// This method is called when your extension is deactivated
export function deactivate() {
	if (!client) {
		return undefined;
	}
	return client.stop();
}
