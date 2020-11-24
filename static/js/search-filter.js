let searchField = document.getElementsByClassName('filters__search-field');
for (let i = 0; i < searchField.length; i++) {
    searchField[i].addEventListener('input', Search);
}


function Search() {
    let searchField = event.currentTarget;
    let container = searchField.parentNode;
    let checkboxes = container.querySelectorAll('.filters__checkbox-div');
    let checkboxesLabels = container.querySelectorAll('.filters__checkbox-div label');
    let value = searchField.value.toLowerCase();
    for (let i = 0; i < checkboxes.length; i++) {
        let checkboxValue = checkboxesLabels[i].innerText.toLowerCase();
        if (checkboxValue.includes(value)) {
            checkboxes[i].style="";
        } else {
            checkboxes[i].style="display: none;"
        }
    }
}