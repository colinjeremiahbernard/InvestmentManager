export function getToken() {
    return localStorage.getItem("token");
}

export async function api(url, options = {}) {
    const token = getToken();

    const headers = {
        ...(options.headers || {}),
        Authorization: `Bearer ${token}`,
    };

    return fetch(url, {
        ...options,
        headers,
    });
}