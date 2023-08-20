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
    private readonly chan: vscode.OutputChannel;
    private readonly clientChan: vscode.OutputChannel;
    private readonly clientTraceChan: vscode.OutputChannel;
    private readonly bar: StatusBar;
    private readonly path: string;
    private readonly client: lc.LanguageClient;

    constructor(readonly ctx: vscode.ExtensionContext) {
        this.chan = vscode.window.createOutputChannel("Fennec");
        this.clientChan = vscode.window.createOutputChannel("Fennec Language Server");
        this.clientTraceChan = vscode.window.createOutputChannel("Fennec Language Server Trace");
        this.bar = new StatusBar(ctx);
        this.path = fennecPath();
        this.client = createClient(this.path, this.clientChan, this.clientTraceChan);
    }

    async init() {
        await this.client.start().then(() => {
            this.bar.updateStatus('ok');
        }).catch(() => {
            this.bar.updateStatus('broken');
        });
    }

    async dispose() {
        await this.client.dispose();
        this.bar.dispose();
        this.clientTraceChan.dispose();
        this.clientChan.dispose();
        this.chan.dispose();
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
    const fp = process.env['FENNEC_PATH'];
    if (fp !== undefined && fp !== '') {
        return fp;
    }
    return path.join(os.homedir(), '.fennec', 'bin', 'fennec');
}

function createClient(serverPath: string, chan: vscode.OutputChannel, traceChan: vscode.OutputChannel): lc.LanguageClient {
    const serverOpts: lc.ServerOptions = {
        run: {
            command: serverPath,
            args: ['server'],
            transport: lc.TransportKind.stdio,
        },
        debug: {
            command: serverPath,
            args: ['server', '--verbose'],
            transport: lc.TransportKind.stdio,
        },
    };

    const clientOpts: lc.LanguageClientOptions = {
        documentSelector: [{ scheme: "file", language: "fennec" }],
        outputChannel: chan,
        traceOutputChannel: traceChan,
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
