let favButtons = document.getElementsByClassName('ad__favourite-icon');
for (let i=0; i < favButtons.length; i++) {
    favButtons[i].addEventListener('click', addDelToFav);
}
function addDelToFav(evt) {
    let wasInFav = 0;
    let icons = evt.currentTarget.querySelectorAll('img');
    console.log(icons);
    let adId = icons[0].parentNode.parentNode.parentNode.getElementsByClassName('prod_id')[0].value;
    console.log(adId);
    let same = document.querySelectorAll('input[value=' + adId+ ']');
    console.log(same);
    for (let i = 0; i < same.length; i++) {
        let sameIcons = same[i].querySelectorAll('img');
        sameIcons[0].classList.toggle('fav-icon_active');
        sameIcons[1].classList.toggle('fav-icon_active');
    }
    if (icons[0].classList.contains('fav-icon_active')) {
        wasInFav = 1;
    }
    let addToFav = new XMLHttpRequest();
    if (wasInFav === 0) {
        addToFav.open('POST', '/product/favourites/add', true);
    } else {
        addToFav.open('POST', '/product/favourites/delete', true);
    }
    let body = 'prod_id=' + adId;
    console.log(body);
    addToFav.setRequestHeader('Content-Type', 'application/x-www-form-urlencoded');
    addToFav.send(body);
}