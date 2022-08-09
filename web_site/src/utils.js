export const get_site_name_from_url = (url) => {
    // url = "www.oleole.pl/mikrofony/quadcast.bhtml"
    let match = url.match(/(https?:\/\/)?(www\.)?([A-Za-z0-9\.-]+)/);
    return match[3];
};

export function waitForElm(selector) {
    return new Promise((resolve) => {
        if (document.querySelector(selector)) {
            return resolve(document.querySelector(selector));
        }

        const observer = new MutationObserver((mutations) => {
            if (document.querySelector(selector)) {
                resolve(document.querySelector(selector));
                observer.disconnect();
            }
        });

        observer.observe(document.body, {
            childList: true,
            subtree: true
        });
    });
}
