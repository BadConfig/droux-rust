var favButtons = document.getElementsByClassName('ad__favourite-icon');
for (let i=0; i < favButtons.length; i++) {
    favButtons[i].addEventListener('click', addDelToFav);
}
function addDelToFav(evt) {
    evt.stopImmediatePropagation();
    let wasInFav = 0;
    let icons = evt.currentTarget.querySelectorAll('img');
    let adId = icons[0].parentNode.parentNode.parentNode.getElementsByClassName('prod_id')[0].value;
    let same = document.querySelectorAll('input[value=\"' + adId + '\"]:not([type=\"checkbox\"]):not([type=\"radio\"])');
    for (let i = 0; i < same.length; i++) {
        let sameIcons = same[i].parentNode.getElementsByClassName('ad__favourite-icon')[0].querySelectorAll('img');
        sameIcons[0].classList.toggle('fav-icon_active');
        console.log(sameIcons[0].classList);
        sameIcons[1].classList.toggle('fav-icon_active');
        console.log(sameIcons[0].classList);
    }
    if (evt.currentTarget.querySelector('.ad__favourite-icon-img_empty').classList.contains('fav-icon_active')) {
        wasInFav = 1;
    }
    console.log(wasInFav);
    let addToFav = new XMLHttpRequest();
    if (wasInFav === 0) {
        addToFav.open('POST', '/product/favourites/add', true);
    } else {
        addToFav.open('POST', '/product/favourites/delete', true);
    }
    let body = 'prod_id=' + adId;
    addToFav.setRequestHeader('Content-Type', 'application/x-www-form-urlencoded');
    addToFav.send(body);
}