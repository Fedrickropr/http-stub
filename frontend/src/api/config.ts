export async function exportConfig(): Promise<Blob> {
	const response = await fetch("/__stub/config");

	if (!response.ok) {
		throw new Error("Failed to export config");
	}

	return await response.blob();
}

export async function importConfig(file: File): Promise<void> {
	const response = await fetch("/__stub/config", {
		method: "PUT",
		headers: {
			"Content-Type": "application/json",
		},
		body: await file.text(),
	});

	if (!response.ok) {
		throw new Error("Failed to import config");
	}
}
