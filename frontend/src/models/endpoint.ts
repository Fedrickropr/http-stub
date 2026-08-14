export interface Endpoint {
	method: string;
	path: string;
	responseCode: number;
	body?: {
		Text: String
	}
}
