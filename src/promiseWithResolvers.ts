type PromiseWithResolvers<T> = {
	promise: Promise<T>;
	resolve: (value: T) => void;
	// biome-ignore lint/suspicious/noExplicitAny:
	reject: (reason?: any) => void;
};

export const promiseWithResolvers = <T>() => {
	const out = {} as PromiseWithResolvers<T>;
	out.promise = new Promise((resolve, reject) => {
		out.resolve = resolve;
		out.reject = reject;
	});
	return out;
};
