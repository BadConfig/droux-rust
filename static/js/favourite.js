let favButtons = document.getElementsByClassName('ad__favourite-icon');
for (let i=0; i < favButtons.length; i++) {
    favButtons[i].addEventListener('click', addDelToFav);
}
function addDelToFav(evt) {
    let wasInFav = 0;
    let icons = evt.currentTarget.querySelectorAll('img');
    let adId = icons[0].parentNode.parentNode.parentNode.getElementsByClassName('prod_id')[0].value;
    let selector = 'input[value=' + CSS.escape(adId) + ']'; 
    let same = document.querySelectorAll('input[value=' + CSS.escape(adId) + ']');
    console.log(same);
    for (let i = 0; i < same.length; i++) {
        let sameIcons = same[i].parentNode.getElementsByClassName('ad__favourite-icon')[0].querySelectorAll('img');
        console.log(sameIcons);
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
    addToFav.setRequestHeader('Content-Type', 'application/x-www-form-urlencoded');
    addToFav.send(body);
}