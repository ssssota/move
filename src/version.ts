/**
 * Given a version string, returns a tuple of the major, minor, and patch versions.
 */
const parseVersion = (version: string): [number, number, number] => {
	const [major, minor, patch] = version
		.replace(/^v/, "")
		.split(".")
		.map((v) => Number(v));
	return [major, minor, patch];
};

/**
 * Compares two version strings.
 * Returns 1 if the first version is greater than the second version.
 * Returns -1 if the first version is less than the second version.
 * Returns 0 if the versions are equal.
 * @param a The first version string.
 * @param b The second version string.
 * @returns The comparison result.
 */
export const compareVersions = (a: string, b: string): number => {
	const [aMajor, aMinor, aPatch] = parseVersion(a);
	const [bMajor, bMinor, bPatch] = parseVersion(b);

	if (aMajor > bMajor) return 1;
	if (aMajor < bMajor) return -1;

	if (aMinor > bMinor) return 1;
	if (aMinor < bMinor) return -1;

	if (aPatch > bPatch) return 1;
	if (aPatch < bPatch) return -1;

	return 0;
};
