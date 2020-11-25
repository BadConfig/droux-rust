let add_pages = document.getElementsByClassName('mobile__page');
let add_buttons = document.getElementsByClassName('mobile__button');

let firstCheck = document.getElementsByClassName('filters__sector');

for (let i = 0; i < firstCheck.length; i++) {
    firstCheck[i].addEventListener('click', check1);
}


function check1() {
    let flag = 1
    for (let i = 0; i < firstCheck.length; i++) {
        if (!firstCheck[i].querySelector('input:checked')) {
            flag = 0;
            break;
        }
    }
    if (flag) {
        add_buttons[0].disabled = false;
    } else{
        add_buttons[0].disabled = true;
    }
}

function turnPage1() {
    add_pages[0].classList.remove('mobile__page_vis');
    add_pages[1].classList.add('mobile__page_vis');
    return false;
}

let secondCheck = [document.getElementById('ad-name'), document.getElementById('ad-description'),
    document.getElementById('price')];

for (let i = 0; i < secondCheck.length; i++) {
    secondCheck[i].addEventListener('click', check2);
}


function check2() {
    let flag = 1
    for (let i = 0; i < secondCheck.length; i++) {
        if (secondCheck[i].value === '') {
            flag = 0;
            break;
        }
    }
    if (flag && (document.querySelector('.state__options input:checked'))) {
        add_buttons[1].disabled = false;
    } else{
        add_buttons[1].disabled = true;
    }
}

function turnPage2() {
    add_pages[1].classList.remove('mobile__page_vis');
    add_pages[2].classList.add('mobile__page_vis');
    return false;
}



function check3() {
    let flag=1
    if (!document.querySelector('.uploader__frame-img')) {
        flag=0;
    }
    if (flag) {
        add_buttons[2].disabled = false;
    } else{
        add_buttons[2].disabled = true;
    }

}

function turnPage3() {
    add_pages[2].classList.remove('mobile__page_vis');
    add_pages[3].classList.add('mobile__page_vis');
    return false;
}