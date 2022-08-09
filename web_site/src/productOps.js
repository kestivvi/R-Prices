import { productStore } from './stores/productStore';
import { pushGraphqlErrorToast, pushToast, toastStore } from './stores/toastStore';
import { get } from 'svelte/store'

export const handleNotification = async (product) => {
    const body = JSON.stringify({
        query: `mutation updProd {
                    updateNotificationOfProduct(
                        productId:${product.id}, 
                        newValue:${!product.notification}
                        ) {
                            id
                            name
                            notification
                    }
                }   
            `
    });

    const response = await fetch('http://127.0.0.1:4000/graphql', {
        headers: { 'content-type': 'application/json' },
        method: 'POST',
        body: body
    });

    const responseJson = await response.json();
    console.log(`Response is: ${JSON.stringify(responseJson)}`);

    product.notification = !product.notification;
    productStore.update((data) =>
        data.map((p) => {
            if (p.id == product.id) {
                p.notification = product.notification;
            }
            return p;
        })
    );

    let newToast = {
        id: 'id' + new Date().getTime(),
        type: 'success',
        title: 'Notification status has been succesfully changed',
        content: `Now you ${product.notification ? 'will' : "won't"} be notified about ${trimString(
            product.name
        )}`
    };

    pushToast(newToast);
    // toastStore.update((data) => [newToast, ...data]);
    console.log(`Sending back: ${JSON.stringify(product)}`);
    return product;
};

export const trimString = (str, length = 20) => {
    // var length = 20;
    return str.length > length ? str.substring(0, length - 3).trim() + '...' : str;
};


export const deleteOffer = async (id) => {
    const body = JSON.stringify({
        query: `mutation delOff {
                    deleteOffer(id:${id}) {
                        id
                        url
                    }
                }`
    });

    const response = await fetch('http://127.0.0.1:4000/graphql', {
        headers: { 'content-type': 'application/json' },
        method: 'POST',
        body: body
    });

    const responseJson = await response.json();

    if (Object.hasOwn(responseJson, 'errors')) {
        pushGraphqlErrorToast(responseJson);
        return false;
    }

    return true;
};