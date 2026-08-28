export interface Endpoint {
	method: string;
	path: string;
	responseCode: number;
	body?: {
		Text: String
	}
	headers: Header[]
}

export interface Header {
	key: string;
	value: string;
}
