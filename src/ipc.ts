import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { promiseWithResolvers } from "./promiseWithResolvers";
import type { Commit, Config } from "./types";

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

export const readConfig = () => {
	return invoke<Config>("read_config");
};

export const saveConfig = (config: Config) => {
	return invoke("save_config", { config });
};

export const openLicenses = () => {
	return invoke("open_licenses");
};
