// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

import * as vscode from 'vscode';

export function activate(context: vscode.ExtensionContext) {
	console.log('Congratulations, your extension "fennec" is now active!');

	let disposable = vscode.commands.registerCommand('fennec.helloWorld', () => {
		vscode.window.showInformationMessage('Hello World from Fennec!');
	});

	context.subscriptions.push(disposable);
}

export function deactivate() {}
