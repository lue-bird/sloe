import * as vscode from "vscode";
import * as language_client_node from "vscode-languageclient/node";
import * as child_process from "node:child_process";

const languageServerExecutableName: string = "sloe";
let client: language_client_node.LanguageClient | null = null;
export async function activate(context: vscode.ExtensionContext): Promise<void> {
  client = new language_client_node.LanguageClient(
    "sloe",
    "sloe",
    async () => child_process.spawn(languageServerExecutableName),
    {
      diagnosticCollectionName: "sloe",
      documentSelector: [{ scheme: "file", language: "sloe" }],
      synchronize: {
        fileEvents: vscode.workspace.createFileSystemWatcher("**/*.sloe"),
      },
    },
  );
  context.subscriptions.push(
    vscode.commands.registerCommand("sloe.commands.restart", async () => {
      await client?.stop();
      await client?.start();
    }),
  );
  await client.start();
}
export function deactivate(): Thenable<void> | undefined {
  if (client !== null) {
    return client.stop();
  }
  return undefined;
}
