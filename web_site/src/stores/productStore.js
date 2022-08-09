import { writable } from "svelte/store";
import { pushToast } from "./toastStore";

export const productStore = writable([]);
export const currentProductStore = writable({});


const fetchProducts = async () => {

    let response = await fetch('http://127.0.0.1:4000/graphql', {
        method: 'POST',
        body: JSON.stringify({
            query: `{
                allProducts {
                    id
                    name
                }
            }`
        }),
        headers: {
            'content-type': 'application/json'
        }
    });

    let data = await response.json();

    data = data.data.allProducts;
    data.reverse();
    // console.log(`Data is: ${JSON.stringify(data)}`);
    productStore.set(data);
};

// fetchProducts();

export const fetchProductsOfCollection = async (collection_id) => {
    let response = await fetch('http://127.0.0.1:4000/graphql', {
        method: 'POST',
        body: JSON.stringify({
            query: `
                query {
                    getCollectionById(collectionId:${collection_id}) {
                        products {
                            id
                            name
                            notification
                        }
                    }
                }
            `
        }),
        headers: {
            'content-type': 'application/json'
        },
        credentials: 'include'
    });

    let responseJson = await response.json();
    // console.log(responseJson);
    // CHECK FOR ERRORS
    if (Object.hasOwn(responseJson, 'errors')) {
        const errorMsg = responseJson.errors.map((i) => i.message).join('\n');
        const newToast = {
            id: 'id' + new Date().getTime(),
            type: 'error',
            title: 'Error while downloading products',
            content: `${errorMsg}`
        };
        pushToast(newToast);
        return false;
    }

    // GET DATA
    let data = responseJson.data.getCollectionById.products;
    productStore.set(data);
    return true;
};
