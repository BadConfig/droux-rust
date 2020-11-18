let reviewLayout = document.getElementsByClassName('review')[0];
let reviewButton = document.getElementsByClassName('card__review-button')[0];

reviewButton.addEventListener('click', toggleRevWindow);
reviewLayout.addEventListener('click', toggleRevWindow);

function toggleRevWindow(evt) {
    if ((evt.target === reviewButton) || (evt.target === reviewLayout)) {
        reviewLayout.classList.toggle('review_visible')
    }
}