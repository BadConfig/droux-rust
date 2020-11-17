let favButtons = document.getElementsByClassName('ad__favourite-icon');
for (let i=0; i < favButtons.length; i++) {
    favButtons[i].addEventListener('click', addDelToFav);
}
function addDelToFav(evt) {
    let wasInFav = 0;
    let icons = evt.currentTarget.querySelectorAll('img');
    console.log(icons);
    for (let i = 0; i < icons.length; i++) {
        icons[i].classList.toggle('fav-icon_active');
    }
        if (icons[0].classList.contains('fav-icon_active')) {
        wasInFav = 1;
    }
    let adId = String(icons[0].parentNode.parentNode.getElementsByClassName('prod_id').value);
    let addToFav = new XMLHttpRequest();
    if (wasInFav === 0) {
        addToFav.open('POST', '/product/favourites/add', true);
    } else {
        addToFav.open('POST', '/product/favourites/add/delete', true);
    }
    let body = 'prod_id=' + encodeURIComponent(adId);
    addToFav.setRequestHeader('Content-Type', 'application/x-www-form-urlencoded');
    addToFav.send(body);
}