import { trimString } from '../productOps';
import { myCollectionStore } from '../stores/myCollectionsStore';
import { pushToast } from '../stores/toastStore';

export const deleteCollection = async (id) => {
    // TODO: MODAL ON FULL PAGE FOR CONFIRMATION

    // REQUEST
    const body = JSON.stringify({
        query: `
            mutation deleteCollection {
                deleteCollection(collectionId:${id}) {
                    id
                    name
                }
            }
        `
    });

    const response = await fetch('http://127.0.0.1:4000/graphql', {
        headers: { 'content-type': 'application/json' },
        method: 'POST',
        body: body,
        credentials: 'include'
    });

    // HANDLE RESPONSE
    const responseJson = await response.json();

    //// CHECK FOR ERRORS
    if (Object.hasOwn(responseJson, 'errors')) {
        const errorMsg = responseJson.errors.map((i) => i.message).join('\n');
        const newToast = {
            id: 'id' + new Date().getTime(),
            type: 'error',
            title: 'Error while deleting the collection',
            content: `${errorMsg}`
        };
        pushToast(newToast);
        return false;
    }

    //// Get data
    const deletedCollection = responseJson.data.deleteCollection;

    myCollectionStore.update((data) => data.filter((c) => c.id != deletedCollection.id));

    // PUSH TOAST
    const newToast = {
        id: 'id' + new Date().getTime(),
        type: 'success',
        title: `The collection has been successfully deleted`,
        content: `The collection "${trimString(deletedCollection.name, 25)}" has been deleted`
    };
    pushToast(newToast);

    return true;
};

export const renameCollection = async (id, oldName, newName) => {
    newName = newName.trim();

    // VALIDATION
    if (oldName == newName) {
        const newToast = {
            id: 'id' + new Date().getTime(),
            type: 'warning',
            title: 'Name has not been updated',
            content: 'New name cannot be the same as the previous one'
        };
        pushToast(newToast);
        return false;
    }

    if (newName.length == 0) {
        const newToast = {
            id: 'id' + new Date().getTime(),
            type: 'warning',
            title: 'Name has not been updated',
            content: 'New name cannot be zero length'
        };
        pushToast(newToast);
        return false;
    }

    // REQUEST
    const body = JSON.stringify({
        query: `
            mutation renameCollection {
                renameCollection(collectionId:${id}, newName:"${newName}") {
                    id
                    name
                }
            }
        `
    });

    const response = await fetch('http://127.0.0.1:4000/graphql', {
        headers: { 'content-type': 'application/json' },
        method: 'POST',
        body: body,
        credentials: 'include'
    });

    const responseJson = await response.json();

    // CHECK FOR ERRORS
    if (Object.hasOwn(responseJson, 'errors')) {
        const errorMsg = responseJson.errors.map((i) => i.message).join('\n');
        const newToast = {
            id: 'id' + new Date().getTime(),
            type: 'error',
            title: 'Error while renaming the collection',
            content: `${errorMsg}`
        };
        pushToast(newToast);
        return false;
    }

    // GET DATA
    const renamedCollection = responseJson.data.renameCollection;

    if (renamedCollection.name != newName) {
        const newToast = {
            id: 'id' + new Date().getTime(),
            type: 'error',
            title: 'Error while renaming the collection',
            content: 'It was unsuccesful for some unknown reason'
        };
        pushToast(newToast);
        return false;
    }

    // UPDATE STORE
    myCollectionStore.update((data) => {
        data.map((c) => {
            if (c.id == id) {
                c.name = newName;
            }
            return c;
        });
        data.sort((a, b) => a.name.localeCompare(b.name));
        return data;
    });

    // PUSH SUCCESS TOAST
    const newToast = {
        id: 'id' + new Date().getTime(),
        type: 'success',
        title: 'The collection has been successfully renamed',
        content: `Now instead of "${oldName}" it is called "${renamedCollection.name}"`
    };
    pushToast(newToast);

    return true;
};

export const am_i_owner_of_collection = async (id) => {
    // REQUEST
    const body = JSON.stringify({
        query: `
            query {
                amIOwnerOfCollection(collectionId:${id})
            }
        `
    });

    const response = await fetch('http://127.0.0.1:4000/graphql', {
        headers: { 'content-type': 'application/json' },
        method: 'POST',
        body: body,
        credentials: 'include'
    });

    const responseJson = await response.json();

    // CHECK FOR ERRORS
    if (Object.hasOwn(responseJson, 'errors')) {
        const errorMsg = responseJson.errors.map((i) => i.message).join('\n');
        const newToast = {
            id: 'id' + new Date().getTime(),
            type: 'error',
            title: 'Error while checking for the owner of the collection',
            content: `${errorMsg}`
        };
        pushToast(newToast);
        return false;
    }

    // GET DATA
    return responseJson.data.amIOwnerOfCollection;
};
