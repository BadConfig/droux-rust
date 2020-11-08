let buttons = document.getElementsByClassName('filters__sector-header');
for (let i = 0; i < buttons.length; i++) {
    buttons[i].addEventListener('click', OpenClose);
}
function OpenClose() {
    let button = event.currentTarget;
    let sector = button.parentNode;
    let checkboxes = sector.getElementsByClassName('filters__must-be-hidden')[0];
    let arrow = button.getElementsByTagName('img')[0];
    checkboxes.classList.toggle('filters__must-be-hidden_hidden');
    arrow.classList.toggle('filter__arrow_turned');
}