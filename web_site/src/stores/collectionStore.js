import { writable } from "svelte/store";
import { pushToast } from './toastStore'

export const publicCollectionsStore = writable([]);

export const fetchPublicCollections = async () => {

    let response = await fetch('http://127.0.0.1:4000/graphql', {
        method: 'POST',
        body: JSON.stringify({
            query: `{
                allPublicCollections {
                    id
                    name
                    description
                    public
                    owner {
                        id
                        name
                    }
                    products {
                        id
                    }
                }
            }`
        }),
        headers: {
            'content-type': 'application/json'
        }
    });

    let responseJson = await response.json();

    //// - CHECK FOR ERRORS
    if (Object.hasOwn(responseJson, 'errors')) {
        const errorMsg = responseJson.errors.map((i) => i.message).join('\n');
        const newToast = {
            id: 'id' + new Date().getTime(),
            type: 'error',
            title: 'Error while downloading public collections',
            content: `${errorMsg}`
        };
        pushToast(newToast);
        return errorMsg;
    }

    let data = responseJson.data.allPublicCollections;
    data = data.map(c => { c.show = true; return c; });
    // console.log(`Data is: ${JSON.stringify(data)}`);
    publicCollectionsStore.set(data);
};

