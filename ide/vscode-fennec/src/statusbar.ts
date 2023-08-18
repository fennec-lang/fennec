// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

import * as vscode from 'vscode';

type Status = 'broken' | 'progress' | 'error' | 'ok' | 'done';

type StatusTheme = {
    // https://code.visualstudio.com/api/references/icons-in-labels
    // https://code.visualstudio.com/api/references/theme-color
    icon: string;
    background: vscode.ThemeColor | undefined;
    foreground: vscode.ThemeColor | undefined;
};

function expandStatus(s: Status): StatusTheme {
    switch (s) {
        case 'broken':
            return {
                icon: '$(circle-slash)',
                background: new vscode.ThemeColor('statusBarItem.errorBackground'),
                foreground: new vscode.ThemeColor('statusBarItem.errorForeground'),
            };
        case 'progress':
            return {
                icon: '$(loading~spin)',
                background: undefined,
                foreground: undefined,
            };
        case 'error':
            return {
                icon: '$(error)',
                background: new vscode.ThemeColor('statusBarItem.warningBackground'),
                foreground: new vscode.ThemeColor('statusBarItem.warningForeground'),
            };
        case 'ok':
            return {
                icon: '$(check)',
                background: new vscode.ThemeColor('statusBarItem.prominentBackground'),
                foreground: new vscode.ThemeColor('statusBarItem.prominentForeground'),
            };
        case 'done':
            return {
                icon: '$(check-all)',
                background: new vscode.ThemeColor('statusBarItem.prominentBackground'),
                foreground: new vscode.ThemeColor('statusBarItem.prominentForeground'),
            };
    }
}

export class StatusBar {
    private readonly version: string;
    private readonly bar: vscode.StatusBarItem;
    private status: Status = 'progress';

    constructor(readonly ctx: vscode.ExtensionContext) {
        this.version = ctx.extension.packageJSON.version;

        this.bar = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left);
        this.bar.name = 'Fennec Status';
        this.updateStatus(this.status);
        this.bar.show();
    }

    private updateStatus(s: Status) {
        this.status = s;
        const t = expandStatus(this.status);
        this.bar.text = `Fennec ${this.version}\u2000${t.icon}`;
        this.bar.color = t.foreground;
        this.bar.backgroundColor = t.background;
    }

    dispose() {
        this.bar.dispose();
    }
}
