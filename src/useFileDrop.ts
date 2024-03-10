import { appWindow } from "@tauri-apps/api/window";
// import { useEffect, useRef } from "react";
import { useEffect, useRef } from "preact/hooks";

type Props = {
	onDragEnter?: () => void;
	onDragLeave?: () => void;
	onDrop?: (paths: string[]) => void;
};

export const useFileDrop = <T extends HTMLElement>(props: Props) => {
	const ref = useRef<T>(null);
	const dragging = useRef(false);
	const hovered = useRef(false);
	const callbacks = useRef(props);
	callbacks.current.onDragEnter = props.onDragEnter;
	callbacks.current.onDragLeave = props.onDragLeave;
	callbacks.current.onDrop = props.onDrop;
	useEffect(() => {
		if (!ref.current) return;
		const onMouseMove = (event: MouseEvent) => {
			if (!dragging.current || !ref.current) return;
			const rect = ref.current.getBoundingClientRect();
			const x = event.clientX;
			const y = event.clientY;
			const mouseIsNotOver =
				x < rect.left || x > rect.right || y < rect.top || y > rect.bottom;
			if (mouseIsNotOver) {
				if (hovered.current) {
					hovered.current = false;
					callbacks.current.onDragLeave?.();
				}
			} else {
				if (!hovered.current) {
					hovered.current = true;
					callbacks.current.onDragEnter?.();
				}
			}
		};
		window.addEventListener("mousemove", onMouseMove);
		const unlisten = appWindow.onFileDropEvent((event) => {
			switch (event.payload.type) {
				case "hover":
					dragging.current = true;
					break;
				case "cancel":
					if (hovered.current) callbacks.current.onDragLeave?.();
					dragging.current = false;
					hovered.current = false;
					break;
				case "drop":
					if (hovered.current) callbacks.current.onDrop?.(event.payload.paths);
					dragging.current = false;
					hovered.current = false;
					break;
			}
		});
		return () => {
			unlisten.then((fn) => fn());
			window.removeEventListener("mousemove", onMouseMove);
		};
	}, []);

	return { ref };
};
