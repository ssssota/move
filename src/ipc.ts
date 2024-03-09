import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import type { PreviewResult } from "../src-tauri/bindings/PreviewResult";
import { promiseWithResolvers } from "./promiseWithResolvers";

type CommitArgs = {
	pattern: string;
	source: string;
	target: string;
};
export const preview = (args: CommitArgs) => {
	return invoke<PreviewResult>("preview", args);
};

export const commit = (args: CommitArgs) => {
	return invoke("commit", args);
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
