import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import type { Commit } from "../src-tauri/bindings/Commit";
import { promiseWithResolvers } from "./promiseWithResolvers";

type CommitArgs = {
	pattern: string;
	source: string;
	target: string;
	dryRun: boolean;
};
export const commit = (args: CommitArgs) => {
	return invoke<Commit>("commit", args);
};

export const selectDirectory = async () => {
	const { promise, resolve, reject } = promiseWithResolvers<
		string | undefined
	>();
	const unlisten = await listen<string | undefined>(
		"directory-select",
		(ev) => {
			unlisten();
			resolve(ev.payload);
		},
	);
	await invoke("select_directory").catch(reject);
	return promise;
};
