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
  let response = await fetch("/__stub/endpoint", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      method,
      path,
      responseCode: responseCode,
    }),
  });

  if (!response.ok) {
    throw new Error(`Failed to delete endpoint: ${response.status}`);
  }
}

export async function addEndpointWithBody(
  method: string,
  path: string,
  responseCode: number,
  body: string,
): Promise<void> {
  let response = await fetch("/__stub/endpoint", {
    method: "PUT",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      method,
      path,
      responseCode: responseCode,
      body,
    }),
  });

  if (!response.ok) {
    throw new Error(`Failed to delete endpoint: ${response.status}`);
  }
}

export async function addEndpointHeader(
  method: string,
  path: string,
  key: string,
  value: string,
): Promise<void> {
  let response = await fetch("/__stub/header", {
    method: "PUT",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      method,
      path,
      key: key,
      value: value
    }),
  });

  if (!response.ok) {
    throw new Error(`Failed to delete endpoint: ${response.status}`);
  }
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

export async function deleteEndpointHeader(
  method: string,
  path: string,
  key: string 
): Promise<void> {
  const response = await fetch("/__stub/header", {
    method: "DELETE",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      method,
      path,
      key
    }),
  });

  if (!response.ok) {
    throw new Error(`Failed to delete endpoint: ${response.status}`);
  }
}
