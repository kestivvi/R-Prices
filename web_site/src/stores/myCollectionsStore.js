import { get, writable } from "svelte/store";
import { loggedInStatus } from '../auth_utils';

export const myCollectionStore = writable([]);

export const fetchMyCollections = async () => {
    // AUTHORIZE ACCESS
    // let logged_in = await am_i_logged_in();
    if (!get(loggedInStatus)) {
        window.location.href = '/auth/login';
        return;
    }

    // FETCH DATA
    let response = await fetch('http://127.0.0.1:4000/graphql', {
        method: 'POST',
        body: JSON.stringify({
            query: `{
					getMyCollections {
						id
						name
						description
                        public
					}
				}`
        }),
        headers: {
            'content-type': 'application/json'
        },
        credentials: 'include'
    });

    let data = await response.json();
    data = data.data.getMyCollections;
    data.sort((a, b) => a.name.localeCompare(b.name));
    myCollectionStore.set(data);
};