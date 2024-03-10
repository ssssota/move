import { signal } from "@preact/signals";
import { commit, selectDirectory } from "./ipc";
import { useFileDrop } from "./useFileDrop";

const source = signal<string>("");
const target = signal<string>("");
const message = signal<string>("");
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
				class="h-full grid grid-areas-[source_target,control_control] grid-rows-[1fr_auto] grid-cols-2"
				onSubmit={(e) => {
					e.preventDefault();
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
					class="area-[source] outline-dashed -outline-offset-10 outline-gray outline-4 rounded"
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
				<button
					ref={targetRef}
					class="area-[target] outline-dashed -outline-offset-10 outline-gray outline-4 rounded"
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
				<p class="area-[control]">
					<button type="submit" disabled={uncontrollable.value}>
						Move! {message.value}
					</button>
				</p>
			</form>
		</main>
	);
}

export default App;
