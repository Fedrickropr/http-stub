import "./style.css";
import { addEndpoint, deleteEndpoint, getEndpoints } from "./api/endpoints";

const app = document.querySelector<HTMLDivElement>("#app")!;

async function renderEndpoints() {
	const endpoints = await getEndpoints();

	const list = document.querySelector<HTMLUListElement>("#endpoint-list")!;

	list.innerHTML = endpoints
		.map(
			endpoint => `
        <li>
          <span class="method">${endpoint.method}</span>
          <span>${endpoint.path}</span>
          <span>${endpoint.responseCode}</span>

          <button
						class="delete-button"
						type="button"
            data-method="${endpoint.method}"
            data-path="${endpoint.path}"
            title="Delete endpoint"
          >
            X
          </button>
        </li>
      `,
		)
		.join("");

	list.querySelectorAll<HTMLButtonElement>(".delete-button").forEach(button => {
		button.addEventListener("click", async () => {
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

		await addEndpoint(
			formData.get("method") as string,
			formData.get("path") as string,
			Number(formData.get("response-code")),
		);

		form.reset();

		await renderEndpoints();
	});

	await renderEndpoints();
}

main();
