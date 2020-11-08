let ads = document.getElementsByClassName('ad');
let photosCount = [];
let currentPhoto = [];
for (let i = 0; i < ads.length; i++) {
    photosCount[i] = ads[i].getElementsByClassName('ad__img').length;
    currentPhoto[i] = 0;
    ads[i].id =  'carousel-id_' + String(i);
    let arrowLeft = ads[i].getElementsByClassName('ad__arrow_left')[0];
    arrowLeft.addEventListener('click', SwipePhotos);
    let arrowRight = ads[i].getElementsByClassName('ad__arrow_right')[0];
    arrowRight.addEventListener('click', SwipePhotos);
}

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