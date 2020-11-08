let arrowRight = document.getElementsByClassName('product__arrow-div_right')[0];
let arrowLeft = document.getElementsByClassName('product__arrow-div_left')[0];
let photos = arrowRight.parentNode.getElementsByClassName('product__all-photos')[0];
let productPhotosCount = photos.getElementsByTagName('img').length;
let productCurrentPhoto = 0;

arrowLeft.addEventListener('click', swipeProduct);
arrowRight.addEventListener('click', swipeProduct);

function swipeProduct() {
    let arrow = event.currentTarget;
    let photoWidth = arrow.parentNode.clientWidth;
    if ((productCurrentPhoto + 1 < productPhotosCount) && arrow.classList.contains('product__arrow-div_right')) {
        productCurrentPhoto +=1
    } else if ((productCurrentPhoto > 0) &&(arrow.classList.contains('product__arrow-div_left'))) {
        productCurrentPhoto -=1;
    }
    photos.style = "transform: translateX(-" + String(photoWidth * productCurrentPhoto) + "px);";
}