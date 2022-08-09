import { trimString } from '../productOps';
import { currentProductStore, productStore } from '../stores/productStore';
import { pushToast } from '../stores/toastStore';
import { get_site_name_from_url } from '../utils';

// Functions working with the api are very consistent
// And they follow theese steps:
// 1. Build a body for request
// 2. Send a request with credentials included
// 3. Handle a response
//      - Check for errors
//      - If there are some, then push toast and return false as signal
// 4. Get data from response
//      - Update some store if necessary
// 5. Push toast cause of success
// 6. Return true as a signal

export const changeNotification = async (productId, newValue) => {
    // 1. BUILD REQUEST BODY
    const body = JSON.stringify({
        query: `mutation updProd {
                    updateNotificationOfProduct(
                        productId:${productId}, 
                        newValue:${newValue}
                        ) {
                            id
                            name
                    }
                }   
            `
    });

    // 2. SEND REQUEST
    const response = await fetch('http://127.0.0.1:4000/graphql', {
        headers: { 'content-type': 'application/json' },
        method: 'POST',
        body: body,
        credentials: 'include'
    });

    // 3. HANDLE RESPONSE
    const responseJson = await response.json();

    //// - CHECK FOR ERRORS
    if (Object.hasOwn(responseJson, 'errors')) {
        const errorMsg = responseJson.errors.map((i) => i.message).join('\n');
        const newToast = {
            id: 'id' + new Date().getTime(),
            type: 'error',
            title: 'Error while changing notification of product',
            content: `${errorMsg}`
        };
        pushToast(newToast);
        return false;
    }

    //// - UPDATE STORE
    let productName = ''
    productStore.update((data) =>
        data.map((p) => {
            if (p.id == productId) {
                p.notification = newValue;
                productName = p.name;
            }
            return p;
        })
    );

    // 5. SEND SUCCESSFULL TOAST
    let newToast = {
        id: 'id' + new Date().getTime(),
        type: 'success',
        title: 'Notification status has been succesfully changed',
        content: `Now you ${newValue ? 'will' : "won't"} be notified about ${trimString(
            productName
        )}`
    };
    pushToast(newToast);

    // 6. RETURN TRUE
    return true;
};


export const getFullProductById = async (id) => {
    // 1. BUILD REQUEST BODY
    const body = JSON.stringify({
        query: `
            {
                getProductById(id:${id}) {
                    id
                    name
                    description
                    notification
                    collection {
                        id
                    }
                    offers {
                        id
                        url
                        prices {
                            id
                            value
                            createdAt
                            availability
                        }
                    }
                }
            }
        `
    });

    // 2. SEND REQUEST
    let response = await fetch('http://127.0.0.1:4000/graphql', {
        method: 'POST',
        body,
        headers: {
            'content-type': 'application/json'
        },
        credentials: 'include'
    });

    // 3. HANDLE RESPONSE
    let responseJson = await response.json();

    //// - CHECK FOR ERRORS
    if (Object.hasOwn(responseJson, 'errors')) {
        const errorMsg = responseJson.errors.map((i) => i.message).join('\n');
        // const newToast = {
        //     id: 'id' + new Date().getTime(),
        //     type: 'error',
        //     title: 'Error downloading offers of product',
        //     content: `${errorMsg}`
        // };
        // pushToast(newToast);
        return errorMsg;
    }

    // GET DATA
    let data = responseJson.data.getProductById;

    // ENRICH DATA
    data.offers = data.offers.map((offer) => {
        offer.site = get_site_name_from_url(offer.url);
        return offer;
    })

    //// - UPDATE STORE
    currentProductStore.set(data);
};

export const renameProduct = async (productId, newName) => {
    // 1. BUILD REQUEST BODY
    const body = JSON.stringify({
        query: `
            mutation renameProd {
                renameProduct(id:${productId}, newValue:"${newName}") {
                    id
                    name
                } 
            }   
        `
    });

    // 2. SEND REQUEST
    const response = await fetch('http://127.0.0.1:4000/graphql', {
        headers: { 'content-type': 'application/json' },
        method: 'POST',
        body: body,
        credentials: 'include'
    });

    // 3. HANDLE RESPONSE
    const responseJson = await response.json();

    //// - CHECK FOR ERRORS
    if (Object.hasOwn(responseJson, 'errors')) {
        const errorMsg = responseJson.errors.map((i) => i.message).join('\n');
        const newToast = {
            id: 'id' + new Date().getTime(),
            type: 'error',
            title: 'Error while renaming the product',
            content: `${errorMsg}`
        };
        pushToast(newToast);
        return false;
    }

    //// - UPDATE STORE
    let productName = ''
    currentProductStore.update((data) => {
        data.name = newName;
        return data;
    });

    // 5. SEND SUCCESSFULL TOAST
    let newToast = {
        id: 'id' + new Date().getTime(),
        type: 'success',
        title: 'Name of the product has been succesfully changed',
        content: `Now its name is ${trimString(newName, 25)}`
    };
    pushToast(newToast);

    // 6. RETURN TRUE
    return true;
};

export const deleteProduct = async (productId) => {
    // 1. BUILD REQUEST BODY
    const body = JSON.stringify({
        query: `
            mutation delProd {
                deleteProduct(id:${productId}) {
                    id
                    name
                    notification
                }
            } 
        `
    });

    // 2. SEND REQUEST
    const response = await fetch('http://127.0.0.1:4000/graphql', {
        headers: { 'content-type': 'application/json' },
        method: 'POST',
        body: body,
        credentials: 'include'
    });

    // 3. HANDLE RESPONSE
    const responseJson = await response.json();

    //// - CHECK FOR ERRORS
    if (Object.hasOwn(responseJson, 'errors')) {
        const errorMsg = responseJson.errors.map((i) => i.message).join('\n');
        const newToast = {
            id: 'id' + new Date().getTime(),
            type: 'error',
            title: 'Error while deleting the product',
            content: `${errorMsg}`
        };
        pushToast(newToast);
        return errorMsg;
    }

    const deletedProduct = responseJson.data.deleteProduct;

    //// - UPDATE STORE
    productStore.update((data) => {
        data.filter((p) => p.id != deletedProduct.id)
    });

    // 5. SEND SUCCESSFULL TOAST
    let newToast = {
        id: 'id' + new Date().getTime(),
        type: 'success',
        title: 'The product has been succesfully deleted',
        content: `Now "${trimString(deleteProduct.name, 25)} is no longer available`
    };
    pushToast(newToast);
};

export const addOfferToProduct = async (productId, url) => {
    // 1. BUILD REQUEST BODY
    const body = JSON.stringify({
        query: `
            mutation addOffer($newOffer:AddOfferInput!) {
                addOfferToProduct(input:$newOffer) {
                    id
                    url
                    prices {
                        id
                        value
                        createdAt
                    }
                }
            }
        `,
        variables: {
            newOffer: {
                url: url,
                productId: +productId
            }
        }
    });

    // 2. SEND REQUEST
    const response = await fetch('http://127.0.0.1:4000/graphql', {
        headers: { 'content-type': 'application/json' },
        method: 'POST',
        body,
        credentials: 'include'
    });

    // 3. HANDLE RESPONSE
    const responseJson = await response.json();

    //// - CHECK FOR ERRORS
    if (Object.hasOwn(responseJson, 'errors')) {
        const errorMsg = responseJson.errors.map((i) => i.message).join('\n');
        const newToast = {
            id: 'id' + new Date().getTime(),
            type: 'error',
            title: 'Error while adding a new offer to the product',
            content: `${errorMsg}`
        };
        pushToast(newToast);
        return errorMsg;
    }

    const newOffer = responseJson.data.addOfferToProduct;

    //// - UPDATE STORE
    currentProductStore.update(data => {
        data.offers = [newOffer, ...data.offers];
        return data;
    })

    // 5. SEND SUCCESSFULL TOAST
    let newToast = {
        id: 'id' + new Date().getTime(),
        type: 'success',
        title: 'The offer has been succesfully added',
        content: `Now "${trimString(newOffer.url, 30)} is being tracked`
    };
    pushToast(newToast);
};