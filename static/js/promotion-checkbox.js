const promo_checkboxes = document.querySelectorAll('input[type=checkbox][value=true]');
promo_checkboxes.forEach((el) => el.addEventListener('change', (e) => UncheckOthers(e)));

let dont_listen = false

function UncheckOthers(e) {
    if (!dont_listen) {
        dont_listen = true;
        const id = e.currentTarget.id;
        promo_checkboxes.forEach((el) => {
            if (el.id !== id) el.checked = false;
        });
        dont_listen = false;
    }
}