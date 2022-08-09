import { writable } from "svelte/store";

export const toastStore = writable([]);

export const pushToast = (toast) => {
    // let newToast = {
    //     id: 'id' + new Date().getTime(),
    //     type: 'success',
    //     title: 'Notification status has been succesfully changed',
    //     content: `Now you ${product.notification ? 'will' : "won't"} be notified about ${trimString(
    //         product.name
    //     )}`
    // };

    toastStore.update((data) => [toast, ...data]);
}

export const pushGraphqlErrorToast = (responseJson) => {
    const errorMsg = responseJson.errors.map((i) => i.message).join('\n');

    const toast = {
        id: 'id' + new Date().getTime(),
        type: 'error',
        title: 'Error while adding the offer',
        content: `${errorMsg}`
    };

    toastStore.update((data) => [toast, ...data]);
}
