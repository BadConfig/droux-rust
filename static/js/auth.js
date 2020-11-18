// let signInButton = document.getElementById('sign-in');
// let signUpButton = document.getElementById('sign-up');
// let signInForm = document.getElementsByClassName('authorizer__form_sign-in')[0];
// let signUpForm = document.getElementsByClassName('authorizer__form_sign-up')[0];
// signInButton.addEventListener('change', ChangeAuthType);
// signUpButton.addEventListener('change', ChangeAuthType);
//
// function ChangeAuthType() {
//     signInForm.classList.toggle('authorizer__form_visible');
//     signUpForm.classList.toggle('authorizer__form_visible');
// }
//
// let authLayout = document.getElementsByClassName('authorizer__background')[0];
// let authButton = document.getElementById('auth-button');
// if (authButton != null) {
//     authButton.addEventListener('click', AuthToggle);
//     authLayout.addEventListener('click', AuthToggle);
// }
//
// function AuthToggle(evt) {
//     if (evt.target === authLayout) {
//         authLayout.classList.remove('authorizer_visible');
//     } else {
//         authLayout.classList.add('authorizer_visible');
//     }
// }

let layout = document.getElementsByClassName('header__user-actions-layout')[0];
let actions = document.getElementsByClassName('header__user-actions')[0];
let userPhoto = document.getElementsByClassName('header__user-photo')[0];

userPhoto.addEventListener('click', actionsShowHide);
layout.addEventListener('click', actionsShowHide);


function actionsShowHide(evt) {
    console.log(evt.target);
    if (evt.target === userPhoto) {
        layout.classList.add('header__user-actions-layout_visible');
        actions.classList.add('header__user-actions_visible');
    }
    if (evt.target === layout) {
        layout.classList.remove('header__user-actions-layout_visible');
        actions.classList.remove('header__user-actions_visible');
    }
}