type Release = {
	html_url: string;
	tag_name: string;
};

export const getLatestRelease = async (
	owner: string,
	repo: string,
): Promise<Release> => {
	const res = await fetch(
		`https://api.github.com/repos/${owner}/${repo}/releases/latest`,
	);
	const json = await res.json();
	return json;
};
