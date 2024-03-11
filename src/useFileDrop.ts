import { appWindow } from "@tauri-apps/api/window";
// import { useEffect, useRef } from "react";
import { useEffect, useRef } from "preact/hooks";

type Props = {
	onDrop?: (paths: string[]) => void;
};

export const useFileDrop = <T extends HTMLElement>(props: Props) => {
	const ref = useRef<T>(null);
	const droped = useRef<string[]>();
	const callbacks = useRef(props);
	callbacks.current.onDrop = props.onDrop;
	useEffect(() => {
		if (!ref.current) return;
		const onMouseMove = (event: MouseEvent) => {
			if (!ref.current || !droped.current) return;
			const rect = ref.current.getBoundingClientRect();
			const x = event.clientX - event.movementX;
			const y = event.clientY - event.movementY;
			const mouseIsOver =
				x >= rect.left && x <= rect.right && y >= rect.top && y <= rect.bottom;
			if (mouseIsOver) {
				callbacks.current.onDrop?.(droped.current);
			}
			droped.current = undefined;
		};
		window.addEventListener("mousemove", onMouseMove);
		const unlisten = appWindow.onFileDropEvent((event) => {
			if (event.payload.type === "drop") {
				droped.current = event.payload.paths;
			}
		});
		return () => {
			unlisten.then((fn) => fn());
			window.removeEventListener("mousemove", onMouseMove);
		};
	}, []);

	return { ref };
};
