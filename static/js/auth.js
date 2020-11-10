let signInButton = document.getElementById('sign-in');
let signUpButton = document.getElementById('sign-up');
let signInForm = document.getElementsByClassName('authorizer__form_sign-in')[0];
let signUpForm = document.getElementsByClassName('authorizer__form_sign-up')[0];
signInButton.addEventListener('change', ChangeAuthType);
signUpButton.addEventListener('change', ChangeAuthType);

function ChangeAuthType() {
    signInForm.classList.toggle('authorizer__form_visible');
    signUpForm.classList.toggle('authorizer__form_visible');
}

let authLayout = document.getElementsByClassName('authorizer__background')[0];
let authButton = document.getElementById('auth-button');
authButton.addEventListener('click', AuthToggle);
authLayout.addEventListener('click', AuthToggle);

function AuthToggle(evt) {
    if (evt.target === authLayout) {
        authLayout.classList.remove('authorizer_visible');
    } else {
        authLayout.classList.add('authorizer_visible');
    }
}