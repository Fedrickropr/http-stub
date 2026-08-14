import type { Endpoint } from "../models/endpoint";

export async function getEndpoints(): Promise<Endpoint[]> {
  const response = await fetch("/__stub/endpoint");

  if (!response.ok) {
    throw new Error(`Failed to fetch endpoints: ${response.status}`);
  }

  return response.json();
}

export async function addEndpoint(
	method: string,
	path: string,
	responseCode: number,
): Promise<void> {
	await fetch("/__stub/endpoint", {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
		},
		body: JSON.stringify({
			method,
			path,
			response_code: responseCode,
		}),
	});
}

export async function addEndpointWithBody(
	method: string,
	path: string,
	responseCode: number,
	body: string,
): Promise<void> {
	await fetch("/__stub/endpoint", {
		method: "PUT",
		headers: {
			"Content-Type": "application/json",
		},
		body: JSON.stringify({
			method,
			path,
			response_code: responseCode,
			body,
		}),
	});
}

export async function deleteEndpoint(
  method: string,
  path: string,
): Promise<void> {
  const response = await fetch("/__stub/endpoint", {
    method: "DELETE",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      method,
      path,
    }),
  });

  if (!response.ok) {
    throw new Error(`Failed to delete endpoint: ${response.status}`);
  }
}
