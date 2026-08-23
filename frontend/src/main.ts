import "./style.css";
import { addEndpoint, addEndpointWithBody, deleteEndpoint, getEndpoints } from "./api/endpoints";
import { exportConfig, importConfig } from "./api/config";

const app = document.querySelector<HTMLDivElement>("#app")!;

async function renderEndpoints() {
	const endpoints = await getEndpoints();

	const list = document.querySelector<HTMLUListElement>("#endpoint-list")!;
	list.innerHTML = endpoints
	.map(
		endpoint => `
		<li class="endpoint">
		<div class="endpoint-header">

		<span class="bold">${endpoint.method}</span>
		<span class="path">${endpoint.path}</span>
		<span class="bold">${endpoint.responseCode}</span>

		<button
		class="button-margin ${endpoint.body ? "" : "expand-button-hidden"}"
		type="button"
		aria-expanded="false"
		>
		▶
		</button>

		<button
		class="delete-button button-margin"
		type="button"
		data-method="${endpoint.method}"
		data-path="${endpoint.path}"
		title="Delete endpoint"
		>
		X
		</button>

		</div>

		${
			endpoint.body
				? `
				<div class="endpoint-body" hidden>
				<textarea readonly>${endpoint.body.Text}</textarea>
				</div>
				`
					: ""
		}

		</li>
		`,
	)
	.join("");

	list.querySelectorAll<HTMLButtonElement>(".expand-button").forEach(button => {
		button.addEventListener("click", () => {
			const endpoint = button.closest(".endpoint")!;
			const body = endpoint.querySelector<HTMLElement>(".endpoint-body");

			if (!body) return;

			const expanded = !body.hidden;

			body.hidden = expanded;
			button.setAttribute("aria-expanded", String(!expanded));
			button.textContent = expanded ? "▶" : "▼";
		});
	});

	list.querySelectorAll<HTMLButtonElement>(".delete-button").forEach(button => {
		button.addEventListener("click", async event => {
			event.preventDefault();
			event.stopPropagation();

			await deleteEndpoint(
				button.dataset.method!,
				button.dataset.path!,
			);

			await renderEndpoints();
		});
	});
}

async function main() {
	app.innerHTML = `
	<h1>HTTP Stub</h1>

	<form id="add-endpoint-form">
		<select id="method" name="method">
		<option value="GET">GET</option>
		<option value="POST">POST</option>
		<option value="PUT">PUT</option>
		<option value="PATCH">PATCH</option>
		<option value="DELETE">DELETE</option>
		<option value="HEAD">HEAD</option>
		<option value="OPTIONS">OPTIONS</option>
	</select>

	<input
		id="path"
		name="path"
		type="text"
		placeholder="/hello"
		pattern="/.*"
		required
	/>

	<input
		id="response-code"
		name="response-code"
		type="number"
		min="100"
		max="599"
		step="1"
		value="200"
		required
	/>

	<textarea
		id="body"
		name="body"
		placeholder="Response body (optional)"
		rows="4"
	></textarea>


	<button type="submit">
		Add endpoint
	</button>
	</form>

	<div class="config-actions">
		<button id="save-config-button" type="button">
			Save config
		</button>

		<button id="load-config-button" type="button">
			Load config
		</button>

		<input
			id="config-file-input"
			type="file"
			accept=".json,application/json"
			hidden
		/>
	</div>

	<h2>Endpoints</h2>

	<ul id="endpoint-list"></ul>
	`;

	const form = document.querySelector<HTMLFormElement>(
		"#add-endpoint-form",
	)!;

	form.addEventListener("submit", async event => {
		event.preventDefault();

		const formData = new FormData(form);

		const method = formData.get("method") as string;
		const path = formData.get("path") as string;
		const responseCode = Number(formData.get("response-code"));
		const body = formData.get("body") as string;

		if (body.trim() === "") {
			await addEndpoint(method, path, responseCode);
		} else {
			await addEndpointWithBody(
				method,
				path,
				responseCode,
				body,
			);
		}

		form.reset();

		await renderEndpoints();
	});

	await renderEndpoints();
	

	const saveButton = document.querySelector<HTMLButtonElement>("#save-config-button")!;

	const loadButton = document.querySelector<HTMLButtonElement>("#load-config-button")!;

	const fileInput = document.querySelector<HTMLInputElement>("#config-file-input")!;

	saveButton.addEventListener("click", async () => {
		const blob = await exportConfig();

		const url = URL.createObjectURL(blob);

		const link = document.createElement("a");
		link.href = url;
		link.download = "http-stub.json";
		link.click();

		URL.revokeObjectURL(url);
	});

	loadButton.addEventListener("click", () => {
		fileInput.click();
	});

	fileInput.addEventListener("change", async () => {
		const file = fileInput.files?.[0];

		if (!file) return;

		await importConfig(file);

		fileInput.value = "";

		await renderEndpoints();
	});
}

main();
