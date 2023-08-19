// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

import * as vscode from 'vscode';
import * as lc from "vscode-languageclient/node";
import * as os from 'os';
import * as path from 'path';
import { StatusBar } from './statusbar';

let _ext: Extension | undefined = undefined;

class Extension {
    private readonly ch: vscode.OutputChannel;
    private readonly bar: StatusBar;
    private readonly path: string;
    private readonly client: lc.LanguageClient;

    constructor(readonly ctx: vscode.ExtensionContext) {
        this.ch = vscode.window.createOutputChannel("Fennec Language Server");
        this.bar = new StatusBar(ctx);
        this.path = fennecPath();
        this.client = createClient(this.path, this.ch);
    }

    async init() {
        await this.client.start();
    }

    async dispose() {
        await this.client.dispose();
        this.bar.dispose();
        this.ch.dispose();
    }
}

export async function activate(ctx: vscode.ExtensionContext) {
    _ext = new Extension(ctx);
    await _ext.init();
}

export async function deactivate() {
    await _ext?.dispose();
}

function fennecPath(): string {
    return path.join(os.homedir(), '.fennec', 'bin', 'fennec');
}

function createClient(serverPath: string, ch: vscode.OutputChannel): lc.LanguageClient {
    const run: lc.Executable = {
        command: serverPath,
        args: ['server'],
        transport: lc.TransportKind.stdio,
    };
    const serverOpts: lc.ServerOptions = {
        run,
        debug: run,
    };

    const clientOpts: lc.LanguageClientOptions = {
        documentSelector: [{ scheme: "file", language: "fennec" }],
        outputChannel: ch,
    };

    const client = new lc.LanguageClient(
        "fennec",
        "Fennec Language Server",
        serverOpts,
        clientOpts,
    );
    client.registerProposedFeatures();

    return client;
}
