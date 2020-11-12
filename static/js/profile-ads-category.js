let activeButton = document.getElementById('active');
let soldButton = document.getElementById('sold');
let deletedButton = document.getElementById('deleted');

let buttons = [activeButton, soldButton, deletedButton];

let display = document.getElementsByClassName('content__ads');

for (let i = 0; i < buttons.length; i++) {
    buttons[i].addEventListener('change', ChooseDisplay);
}

function ChooseDisplay(evt) {
    for (let i = 0; i < buttons.length; i++) {
        if (buttons[i] === evt.currentTarget) {
            display[i].classList.add('content__ads_visible');
        } else {
            display[i].classList.remove('content__ads_visible');
        }
    }
}