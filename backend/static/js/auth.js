export function getToken() {
    return localStorage.getItem("token");
}

export async function api(url, options = {}) {
    const token = getToken();

    // Start with any headers passed in
    const headers = {
        ...(options.headers || {}),
        Authorization: `Bearer ${token}`,
    };

    // If a body is provided and no Content-Type is set, default to JSON
    if (options.body && !headers["Content-Type"]) {
        headers["Content-Type"] = "application/json";
    }

    const response = await fetch(url, {
        ...options,
        headers,
    });

    // Automatically parse JSON if possible
    const contentType = response.headers.get("Content-Type") || "";
    if (contentType.includes("application/json")) {
        return response.json();
    }

    return response;
}

