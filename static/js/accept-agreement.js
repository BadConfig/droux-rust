let acceptCheckbox = document.getElementById('accept-agreement');
acceptCheckbox.addEventListener('change', allowBuying);
orderButton = document.querySelector('.new-order__button');
function allowBuying() {
    if (acceptCheckbox.checked) {
        orderButton.disabled = false;
    } else {
        orderButton.disabled = true;
    }
}