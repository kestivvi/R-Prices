import { writable } from "svelte/store";

export const loggedInStatus = writable([]);

export const fetchStatus = async () => {

    const response = await fetch('http://127.0.0.1:4000/auth/status', {
        headers: { 'content-type': 'application/json' },
        method: 'GET',
        credentials: 'include'
    });

    let response_json = await response.json();
    // console.log(`fetchStatus response: ${JSON.stringify(response_json)}`);

    loggedInStatus.set(!!response_json.loggedIn);
};

fetchStatus();


export const log_me_out = async () => {
    await fetch('http://127.0.0.1:4000/auth/logout', {
        headers: { 'content-type': 'application/json' },
        method: 'POST',
        credentials: 'include'
    });

    await fetchStatus();
}

export const me = async () => {
    const response = await fetch('http://127.0.0.1:4000/auth/me', {
        headers: { 'content-type': 'application/json' },
        method: 'GET',
        credentials: 'include'
    });

    const responseJson = await response.json();

    //// - CHECK FOR ERRORS
    if (Object.hasOwn(responseJson, 'errors')) {
        const errorMsg = responseJson.errors.map((i) => i.message).join('\n');
        const newToast = {
            id: 'id' + new Date().getTime(),
            type: 'error',
            title: 'Cannot get information about you',
            content: `${errorMsg}`
        };
        pushToast(newToast);
        return { errors: errorMsg };
    }

    return responseJson;
}

export const changeName = async (new_name) => {
    // 1. BUILD REQUEST BODY
    const body = JSON.stringify({
        new_name
    });

    // 2. SEND REQUEST
    const response = await fetch('http://127.0.0.1:4000/auth/changeName', {
        headers: { 'content-type': 'application/json' },
        method: 'POST',
        body: body,
        credentials: 'include'
    });

    // 3. HANDLE RESPONSE
    // CHECK FOR ERRORS
    if (response.status != 200) {
        return await response.text();
    }
};

export const changeEmail = async (new_email) => {
    // 1. BUILD REQUEST BODY
    const body = JSON.stringify({
        new_email
    });

    // 2. SEND REQUEST
    const response = await fetch('http://127.0.0.1:4000/auth/changeEmail', {
        headers: { 'content-type': 'application/json' },
        method: 'POST',
        body: body,
        credentials: 'include'
    });

    // 3. HANDLE RESPONSE
    // CHECK FOR ERRORS
    if (response.status != 200) {
        return await response.text();
    }
};

export const changePassword = async (old_password, new_password) => {
    // 1. BUILD REQUEST BODY
    const body = JSON.stringify({
        old_password,
        new_password
    });

    // 2. SEND REQUEST
    const response = await fetch('http://127.0.0.1:4000/auth/changePassword', {
        headers: { 'content-type': 'application/json' },
        method: 'POST',
        body: body,
        credentials: 'include'
    });

    // 3. HANDLE RESPONSE
    // CHECK FOR ERRORS
    if (response.status != 200) {
        return await response.text();
    }
};
