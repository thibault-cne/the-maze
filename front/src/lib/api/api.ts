export function call(
	route: string,
	parameters: any,
	method: string,
	body: any,
	headers: any
): Promise<Response> {
	// Get the url from the environment
	let url: string;

	if (import.meta.env.DEV) {
		url = import.meta.env.VITE_BACKEND_DEV_URL + route;
	} else {
		url = import.meta.env.VITE_BACKEND_PROD_URL + route;
	}

	// Add the parameters to the url
	if (parameters) {
		url += '?';
		for (const key in parameters) {
			url += key + '=' + parameters[key] + '&';
		}
		url = url.slice(0, -1);
	}

	// Create the request
	const request = new Request(url, {
		method: method,
		headers: headers ? headers : new Headers(),
		credentials: 'include',
		body: body,
		redirect: 'follow'
	});

	// Send the request and return the response
	return fetch(request);
}
