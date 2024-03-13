import type { ComponentProps, FunctionComponent, JSX } from "preact";

type Props = Omit<ComponentProps<"div">, "min" | "max" | "value"> & {
	max?: JSX.Signalish<number>;
	value: JSX.Signalish<number>;
};
export const ProgressBar: FunctionComponent<Props> = (props) => {
	const { max, value, ...rest } = props;
	const valueMax =
		typeof props.max === "object" ? props.max.value : Number(props.max || 1);
	const valueNow =
		typeof props.value === "object" ? props.value.value : Number(props.value);
	const clampedValue = Math.max(0, Math.min(valueNow, valueMax));
	return (
		<div
			{...rest}
			role="progressbar"
			aria-valuemin={0}
			aria-valuemax={valueMax}
			aria-valuenow={clampedValue}
			style={{ "--percentage": `${(clampedValue / valueMax - 1) * 100}%` }}
			class="relative rounded h-1 w-full overflow-hidden bg-gray-100 before:content-[''] before:absolute before:block before:inset-0 before:bg-current before:translate-x-[var(--percentage)]"
		/>
	);
};
