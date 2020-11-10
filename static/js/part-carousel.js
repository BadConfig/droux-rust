let parts = document.getElementsByClassName('part');
let adsCount = [];
let currentFirstAd =[];
for (let i = 0; i < parts.length; i++) {
    adsCount[i] = parts[i].getElementsByClassName('part__ad').length;
    parts[i].id = "part-carousel-id_" + String(i);
    currentFirstAd[i] = 0;
    let arrowLeft = parts[i].getElementsByClassName('part__arrow_left')[0];
    arrowLeft.addEventListener('click', SwipePart);
    let arrowRight = parts[i].getElementsByClassName('part__arrow_right')[0];
    arrowRight.addEventListener('click', SwipePart);
}

function SwipePart() {
    let arrow = event.currentTarget;
    let part = arrow.parentNode;
    let ads = part.getElementsByClassName('part__ads')[0];
    let partID = Number(part.id.slice(17)); //17 символов в строке part-carousel-id_
    let ad = ads.getElementsByClassName('part__ad')[0];
    let adMarginRight = Number(getComputedStyle(ad).marginRight.slice(0, -2));
    let adWidth = Number(getComputedStyle(ad).width.slice(0, -2));
    let deltaAds;
    if (arrow.classList.contains('part__arrow_left')) {
        if (currentFirstAd[partID] > 0) {
            deltaAds = (currentFirstAd[partID] >= 4) ? -4 : -(currentFirstAd[partID]);
            currentFirstAd[partID] += deltaAds;
        }
    } else {
        if (currentFirstAd[partID] + 4 < adsCount[partID]) {
            deltaAds = (adsCount[partID] - currentFirstAd[partID] >= 8) ? 4 :  (adsCount[partID] - currentFirstAd[partID] - 4);
            currentFirstAd[partID] += deltaAds;
        }
    }
    ads.style = "transform: translateX(-" + String(currentFirstAd[partID] * (adMarginRight + adWidth)) + "px);";
}