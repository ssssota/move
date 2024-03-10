import { signal } from "@preact/signals";
import { useRef } from "preact/hooks";
import { commit, preview, selectDirectory } from "./ipc";
import { useFileDrop } from "./useFileDrop";

const source = signal<string>("");
const target = signal<string>("");
const message = signal<string>("");
const dryrun = signal<string>("");
const uncontrollable = signal<boolean>(false);
const control = async <T,>(promise: Promise<T>): Promise<T> => {
	uncontrollable.value = true;
	try {
		return await promise;
	} finally {
		uncontrollable.value = false;
	}
};

function App() {
	const patternRef = useRef<HTMLInputElement>(null);
	const { ref: sourceRef } = useFileDrop<HTMLParagraphElement>({
		onDrop(paths) {
			source.value = paths[0];
		},
	});
	const { ref: targetRef } = useFileDrop<HTMLParagraphElement>({
		onDrop(paths) {
			target.value = paths[0];
		},
	});

	return (
		<main>
			<h1>move</h1>
			<form
				onSubmit={(e) => {
					e.preventDefault();
					if (!patternRef.current) return;
					const pattern = patternRef.current.value;
					control(
						commit({ pattern, source: source.value, target: target.value })
							.then(() => {
								message.value = "Done!";
							})
							.catch((err) => {
								message.value = `Error: ${err}`;
							}),
					);
				}}
			>
				<p ref={sourceRef}>
					From: {source.value}
					<button
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
						📂
					</button>
				</p>
				<p ref={targetRef}>
					To: {target.value}
					<button
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
						📂
					</button>
				</p>
				<p>
					<label>
						Pattern:
						<input type="text" ref={patternRef} defaultValue="{FILE_NAME}" />
					</label>
				</p>
				<p>
					<button
						type="button"
						disabled={uncontrollable.value}
						onClick={() => {
							if (!patternRef.current) return;
							control(
								preview({
									pattern: patternRef.current.value,
									source: source.value,
									target: target.value,
								})
									.then((res) => {
										dryrun.value = res.entries
											.map((entry) => entry.join(" => "))
											.join("\n");
										message.value = "Done!";
									})
									.catch((err) => {
										message.value = `Error: ${err}`;
									}),
							);
						}}
					>
						Dry run
					</button>
					<button type="submit" disabled={uncontrollable.value}>
						Move!
					</button>
				</p>
				<p>{message.value}</p>
			</form>
			<textarea readOnly value={dryrun.value} />
		</main>
	);
}

export default App;
