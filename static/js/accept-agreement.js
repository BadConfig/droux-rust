let acceptCheckbox = document.getElementById('accept-agreement');
acceptCheckbox.addEventListener('change', allowBuying);
orderButton = document.querySelector('.new-order__form input[type=\"submit\"]');
function allowBuying() {
    if (acceptCheckbox.checked) {
        orderButton.disabled = false;
    } else {
        orderButton.disabled = true;
    }
}