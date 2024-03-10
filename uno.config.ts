import type { Preset } from "unocss";
import { defineConfig, presetMini } from "unocss";

export default defineConfig({
	presets: [presetMini(), presetGridAreas()],
});

function presetGridAreas(): Preset {
	return {
		name: "grid-areas",
		rules: [
			[
				/grid-areas-\[(.+)\]/,
				([_, areas]) => ({
					"grid-template-areas": areas
						.split(",")
						.map((row) => JSON.stringify(row.split("_").join(" ")))
						.join(" "),
				}),
			],
			[
				/area-\[(.+)\]/,
				([_, area]) => ({
					"grid-area": area,
				}),
			],
		],
	};
}
