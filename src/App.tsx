import { signal } from "@preact/signals";
import { commit, selectDirectory } from "./ipc";
import { useFileDrop } from "./useFileDrop";

const source = signal<string>("");
const target = signal<string>("");
const message = signal<string>("Select source and target directories.");
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

function App() {
	const { ref: sourceRef } = useFileDrop<HTMLButtonElement>(sourceProps);
	const { ref: targetRef } = useFileDrop<HTMLButtonElement>(targetProps);

	return (
		<main class="h-full">
			<form
				class="h-full grid grid-areas-[source_arrow_target,control_control_control] grid-rows-[1fr_auto] grid-cols-[1fr_auto_1fr]"
				onSubmit={(e) => {
					e.preventDefault();
					message.value = "Moving...";
					control(
						commit({
							pattern: "{CREATED_YYYY}/{CREATED_MM}{CREATED_DD}/{FILE_NAME}",
							source: source.value,
							target: target.value,
							dryRun: false,
						})
							.then(() => {
								message.value = "Done!";
							})
							.catch((err) => {
								message.value = `Error: ${err}`;
							}),
					);
				}}
			>
				<button
					ref={sourceRef}
					class="area-[source] outline-dashed -outline-offset-10 outline-gray outline-4 rounded hover:bg-gray-100"
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
					class="area-[target] outline-dashed -outline-offset-10 outline-gray outline-4 rounded hover:bg-gray-100"
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
				<div class="area-[control] p-1 flex items-center justify-between">
					<p>{message.value}</p>
					<button
						type="submit"
						disabled={uncontrollable.value}
						class="py-1 px-2 rounded border hover:bg-gray-100"
					>
						Move!
					</button>
				</div>
			</form>
		</main>
	);
}

export default App;
