let card = document.querySelector('.card');
let profileButton = document.querySelector('.profile-button');
let profileContent = document.querySelector('.content');

profileButton.addEventListener('click', goToProfile);

function goToProfile() {
    profileContent.classList.add('content_mob_invis');
    card.classList.add('card_mob_vis');
}