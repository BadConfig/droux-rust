let dropdownButtons = document.getElementsByClassName('dropdown__button');
let dropdown = document.getElementsByClassName('dropdown')[0];
let dropdownLayout = document.getElementsByClassName('dropdown__layout')[0];
let dropdownCategories = document.getElementsByClassName('dropdown__border');

const isTouchable = is_touch_device();

function is_touch_device() {
    try {
        document.createEvent("TouchEvent");
        return true;
    } catch (e) {
        return false;
    }
}

for (let i = 0; i < dropdownButtons.length; i++) {
    if (isTouchable) {
        dropdownButtons[i].addEventListener('touch', (e) => {
            showDD();
            e.preventDefault();
        });
        dropdownButtons[i].addEventListener('click', (e) => {
            e.preventDefault();
        })
    }
    dropdownButtons[i].addEventListener('mouseover', showDD);
}
function showDD(evt) {
    dropdown.classList.add('dropdown_visible');
    dropdownLayout.classList.add('dropdown__layout_visible');
    for (let i = 0; i < dropdownCategories.length; i++) {
        dropdownCategories[i].classList.remove('dropdown__border_visible');
    }
    let index;
    for (let i = 0; i < dropdownButtons.length; i++) {
        if (dropdownButtons[i] === evt.target) {
            index = i;
            break;
        }
    }
    dropdownCategories[index].classList.add('dropdown__border_visible');
}
if (isTouchable) {
    dropdownLayout.addEventListener('touch', hideDD);
    document.querySelector('.services').addEventListener('touch', hideDD);
    document.querySelector('.header-base__line').addEventListener('touch', hideDD);
    document.getElementsByClassName('header__base')[0].addEventListener('touch', hideDD);
}
dropdownLayout.addEventListener('mouseover', hideDD);
document.querySelector('.services').addEventListener('mouseover', hideDD);
document.querySelector('.header-base__line').addEventListener('mouseover', hideDD);
document.getElementsByClassName('header__base')[0].addEventListener('mouseover', hideDD);

function hideDD(evt) {
    dropdown.classList.remove('dropdown_visible');
    dropdownLayout.classList.remove('dropdown__layout_visible');
}