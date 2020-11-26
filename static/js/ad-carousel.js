let ads = document.getElementsByClassName('ad');
let photosCount = [];
let currentPhoto = [];
function checkAds() {
    for (let i = 0; i < ads.length; i++) {
        photosCount[i] = ads[i].getElementsByClassName('ad__img').length;
        currentPhoto[i] = 0;
        ads[i].id = 'carousel-id_' + String(i);
        let arrowLeft = ads[i].getElementsByClassName('ad__arrow_left')[0];
        arrowLeft.addEventListener('click', SwipePhotos);
        let arrowRight = ads[i].getElementsByClassName('ad__arrow_right')[0];
        arrowRight.addEventListener('click', SwipePhotos);
    }
}
checkAds();
function SwipePhotos() {
    let arrow = event.currentTarget;
    let ad = arrow.parentNode.parentNode;
    let adID = Number(ad.id.slice(12)); //12 - кол-во символов в строчке carousel-id_
    let photos = ad.getElementsByClassName('ad__all-images')[0];
    let photoWidth = photos.getElementsByTagName('img')[0].clientWidth;
    if (arrow.classList.contains('ad__arrow_right')) {
        if (currentPhoto[adID] + 1 < photosCount[adID]) {
            currentPhoto[adID] += 1;
        }
    } else {
        if (currentPhoto[adID] > 0) {
            currentPhoto[adID] -= 1;
        }
    }
    photos.style="transform: translateX(-" + String(photoWidth * currentPhoto[adID]) + "px);";

}

let photoWidth = String(document.querySelector('.ad__img-container').clientWidth) + 'px';
let borderHeight = Number(window.getComputedStyle(document.querySelector('.ad__img-container')).height.slice(0, -2));
let adPhotos = document.getElementsByClassName('ad__img');
document.onload = function() {
    changeSize();
}
function changeSize() {
    for (let i = 0; i < adPhotos.length; i++) {
        console.log(i);
        adPhotos[i].onload = function () {
            if (document.documentElement.clientWidth < 1200) {
                adPhotos[i].style.width = photoWidth;
                let photoHeight = Number(window.getComputedStyle(adPhotos[i]).height.slice(0, -2));
                adPhotos[i].style.top = String(-(photoHeight - borderHeight) / 2) + 'px';
            } else {
                adPhotos[i].style.height = String(borderHeight) + 'px';
                console.log( adPhotos[i].style.height);
                adPhotos[i].style.left = '-' + String((adPhotos[i].clientWidth - 255)/2) + 'px';
                console.log( adPhotos[i].style.left);
            }
        }
    }
}