import { batch, effect, signal } from "@preact/signals";
import { getVersion } from "@tauri-apps/api/app";
import { listen } from "@tauri-apps/api/event";
import type { ComponentChild } from "preact";
import { useEffect, useRef } from "preact/hooks";
import { ProgressBar } from "./ProgressBar";
import { commit, readConfig, saveConfig, selectDirectory } from "./ipc";
import { getLatestRelease } from "./release";
import type { Progress } from "./types";
import { useFileDrop } from "./useFileDrop";
import { compareVersions } from "./version";

const repository = "https://github.com/ssssota/move";

const version = signal<string>("0.0.0");
const source = signal<string>("");
const target = signal<string>("");
const pattern = signal<string>("");
const progress = signal<number>(0);
const message = signal<ComponentChild>("Select source and target directories.");
const uncontrollable = signal<boolean>(false);
const control = <T,>(promise: Promise<T>): Promise<T> => {
	uncontrollable.value = true;
	return promise.finally(() => {
		uncontrollable.value = false;
	});
};
const sourceProps = {
	onDrop: (paths: string[]) => {
		source.value = paths[0];
	},
};
const targetProps = {
	onDrop: (paths: string[]) => {
		target.value = paths[0];
	},
};
getVersion().then((v) => {
	version.value = v;
	getLatestRelease("ssssota", "move").then((release) => {
		const compared = compareVersions(version.value, release.tag_name);
		if (compared === -1) {
			message.value = (
				<span>
					New version available:{" "}
					<a
						href={release.html_url}
						target="_blank"
						rel="noreferrer"
						class="text-blue hover:underline"
					>
						{release.tag_name}
					</a>
				</span>
			);
		}
	});
});
readConfig().then((c) => {
	batch(() => {
		source.value = c.source;
		target.value = c.target;
		pattern.value =
			c.pattern || "{CREATED_YYYY}/{CREATED_MM}{CREATED_DD}/{FILE_NAME}";
	});
});
effect(() => {
	saveConfig({
		version: "V0",
		source: source.value,
		target: target.value,
		pattern: pattern.value,
	});
});

function App() {
	const dialogRef = useRef<HTMLDialogElement>(null);
	const { ref: sourceRef } = useFileDrop<HTMLButtonElement>(sourceProps);
	const { ref: targetRef } = useFileDrop<HTMLButtonElement>(targetProps);

	useEffect(() => {
		const unlisten = listen<Progress>("commit-progress", (ev) => {
			progress.value = Math.floor(ev.payload.complete / ev.payload.total);
		});
		return () => {
			unlisten.then((fn) => fn());
		};
	}, []);

	return (
		<main class="relative h-full">
			<form
				class="h-full grid grid-areas-[source_arrow_target,control_control_control] grid-rows-[1fr_auto] grid-cols-[1fr_auto_1fr]"
				onSubmit={(e) => {
					e.preventDefault();
					message.value = "Moving...";
					control(
						commit({
							pattern: pattern.value,
							source: source.value,
							target: target.value,
							dryRun: false,
						})
							.then((res) => {
								message.value = `Done! (${res.entries.length} files)`;
							})
							.catch((err) => {
								message.value = <span class="text-red-700">Error: {err}</span>;
							}),
					);
				}}
			>
				<button
					ref={sourceRef}
					class="area-[source] break-anywhere p-4 outline-dashed -outline-offset-10 outline-gray outline-4 rounded hover:bg-gray-100 focus-visible:bg-gray-100"
					type="button"
					disabled={uncontrollable.value}
					onClick={() => {
						control(
							selectDirectory().then((dir) => {
								if (dir) source.value = dir;
							}),
						);
					}}
				>
					📂 {source.value}
				</button>
				<div
					title="Move to"
					aria-label="move to"
					class="area-[arrow] grid place-content-center"
				>
					▶
				</div>
				<button
					ref={targetRef}
					class="area-[target] break-anywhere p-4 outline-dashed -outline-offset-10 outline-gray outline-4 rounded hover:bg-gray-100 focus-visible:bg-gray-100"
					type="button"
					disabled={uncontrollable.value}
					onClick={() => {
						control(
							selectDirectory().then((dir) => {
								if (dir) target.value = dir;
							}),
						);
					}}
				>
					📂 {target.value}
				</button>
				<div class="area-[control] px-1 pt-0.5 pb-2 flex items-center justify-between">
					<p class="truncate">{message.value}</p>
					<div class="flex gap-1">
						<button
							type="submit"
							disabled={uncontrollable.value}
							class="py-1 px-2 rounded border hover:bg-gray-100 focus-visible:bg-gray-100"
						>
							Move!
						</button>
						<button
							type="button"
							onClick={() => {
								dialogRef.current?.showModal();
							}}
							disabled={uncontrollable.value}
							class="py-1 px-2 rounded border hover:bg-gray-100 focus-visible:bg-gray-100"
							aria-label="Open settings"
						>
							⚙️
						</button>
					</div>
				</div>
			</form>
			<dialog ref={dialogRef}>
				<div class="fixed inset-0 bg-white p-4 gap-4 flex flex-col overflow-auto">
					<section class="flex flex-col gap-4">
						<h3 class="font-bold">Settings</h3>
						<form
							class="flex flex-col gap-4"
							method="dialog"
							onSubmit={(e) => {
								pattern.value = e.currentTarget.pattern.value;
							}}
						>
							<label class="grid grid-cols-[auto_1fr]">
								<span>Pattern:</span>
								<input
									type="text"
									value={pattern.value}
									name="pattern"
									class="font-mono border rounded px-1"
								/>
							</label>
							<p class="flex justify-end gap-1">
								<button
									type="button"
									class="py-1 px-2 rounded border hover:bg-gray-100 focus-visible:bg-gray-100"
									onClick={() => dialogRef.current?.close()}
								>
									Cancel
								</button>
								<button
									type="submit"
									class="py-1 px-2 rounded border hover:bg-gray-100 focus-visible:bg-gray-100"
								>
									OK
								</button>
							</p>
						</form>
					</section>
					<hr />
					<section>
						<h3 class="font-bold">About</h3>
						<dl class="grid grid-cols-[auto_1fr] gap-x-2">
							{[
								["Author", "ssssota"],
								["Version", version.value],
								[
									"Repository",
									<a href={repository} target="_blank" rel="noreferrer">
										{repository}
									</a>,
								],
							].map(([k, v]) => (
								<div class="contents">
									<dt class="text-gray-500">{k}</dt>
									<dd>{v}</dd>
								</div>
							))}
						</dl>
					</section>
				</div>
			</dialog>

			<div class="fixed bottom-0 w-full text-sky-300">
				{progress.value > 0 && <ProgressBar value={progress.value} />}
			</div>
		</main>
	);
}

export default App;
