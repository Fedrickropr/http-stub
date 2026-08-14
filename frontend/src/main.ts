import "./style.css";
import { addEndpoint, addEndpointWithBody, deleteEndpoint, getEndpoints } from "./api/endpoints";

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
		class="expand-button ${endpoint.body ? "" : "expand-button-hidden"}"
		type="button"
		aria-expanded="false"
		>
		▶
		</button>

		<button
		class="delete-button expand-button"
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
}

main();
