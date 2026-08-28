import "./style.css";
import { addEndpoint, addEndpointHeader, addEndpointWithBody, deleteEndpoint, getEndpoints } from "./api/endpoints";
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
		class="expand-button button-margin"
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

		
    <div 
    class="endpoint-body" 
    hidden>
      <form class="row"
      data-method="${endpoint.method}"
      data-path="${endpoint.path}"
      data-responsecode="${endpoint.responseCode}"
      id="update-body-form" 
      >
			  <textarea id="body" name="body">${endpoint.body ? endpoint.body.Text : ""}</textarea>
        <button 
        class="row-item"
        type ="submit"
        class="button-margin"
        > 
          Update
        </button>
      </form>
    
    <h2>Headers</h2>
    ${endpoint.headers.map(header => `
    <div
    class="row"
    >
      <input readonly class="row-item" name="key" value=${header.key}> </input>
      <input readonly class="row-item" name="value" value=${header.value}> </input>
      <button 
        type="submit"
        > 
        &#10006
      </button>
    </div>
    `).join("")}

    <form 
    class="row" 
    id="add-header-form"
    data-method="${endpoint.method}"
    data-path="${endpoint.path}"
    > 
      <input class="row-item" name="key"> </input>
      <input class="row-item" name="value"> </input>
      <button 
      type="submit"
      > 
        &#10004;
      </button>
    </form>
		</div>

		</li>
		`
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


  const formUpdateBody = document.querySelector<HTMLFormElement>(
    "#update-body-form",
  )!;

  formUpdateBody.addEventListener("submit", async event => {
    event.preventDefault();

    const formData = new FormData(formUpdateBody);

    const body = formData.get("body") as string;
    await addEndpointWithBody(
      formUpdateBody.dataset.method!,
      formUpdateBody.dataset.path!,
      parseInt(formUpdateBody.dataset.responsecode!),
      body
    );

    formUpdateBody.reset();

    await renderEndpoints();
  });

  const formAddHeader = document.querySelector<HTMLFormElement>(
    "#add-header-form",
  )!;

  formAddHeader.addEventListener("submit", async event => {
    event.preventDefault();

    const formData = new FormData(formAddHeader);

    const key = formData.get("key") as string;
    const value = formData.get("value") as string;

    await addEndpointHeader(
      formAddHeader.dataset.method!,
      formAddHeader.dataset.path!,
      key,
      value,
    );

    formAddHeader.reset();

    await renderEndpoints();
  });
}

async function main() {
  app.innerHTML = `
	<h1>HTTP Stub</h1>

	<form id="add-endpoint-form" class="endpoint-form">
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
