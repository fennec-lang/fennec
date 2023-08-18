// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

import * as vscode from 'vscode';
import { StatusBar } from './statusbar';

class Extension {
    private readonly bar: StatusBar;

    constructor(readonly ctx: vscode.ExtensionContext) {
        this.bar = new StatusBar(ctx);
    }

    dispose() {
        this.bar.dispose();
    }
}

export async function activate(ctx: vscode.ExtensionContext) {
    const ext = new Extension(ctx);
    ctx.subscriptions.push(ext);
}

export async function deactivate() {
}
