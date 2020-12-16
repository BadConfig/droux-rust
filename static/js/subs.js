const subCatSwitchers = document.getElementsByClassName('subscribes__category-switcher');
const subCats = document.getElementsByClassName('subscribes__section');

subCatSwitchers[0].addEventListener('change', switchSection);
subCatSwitchers[1].addEventListener('change', switchSection);

function switchSection() {
    subCats[0].classList.toggle('subscribes__section_active');
    subCats[1].classList.toggle('subscribes__section_active');
}