const newsArrowLeft = document.getElementsByClassName('carousel__arrow-div_position_left')[0];
const newsArrowRight = document.getElementsByClassName('carousel__arrow-div_position_right')[0];
const newsMarkers = document.getElementsByClassName('carousel__marker');
const slider = document.getElementsByClassName('carousel__all-news')[0];

let currentBanner = 0

newsArrowLeft.addEventListener('click', SwipeNews);
newsArrowRight.addEventListener('click', SwipeNews);

function SwipeNews(e) {
    console.log(currentBanner);
    let sliderWidth = slider.clientWidth;
    console.log(sliderWidth);
    newsMarkers[currentBanner].classList.remove('carousel__marker_active');
    newsMarkers[currentBanner].src = "/static/assets/carousel-marker.svg"
    if ((e.target === newsArrowLeft) && (currentBanner > 0)) {
        currentBanner -= 1;
    } else if ((e.target === newsArrowRight) && (currentBanner < newsMarkers.length - 2)) {
        currentBanner += 1;
    }
    newsMarkers[currentBanner].classList.add('carousel__marker_active');
    newsMarkers[currentBanner].src = "/static/assets/carousel-marker-active.svg"
    slider.style.transform = "translateX(-" + (currentBanner * sliderWidth) + "px)";
}