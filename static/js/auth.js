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
if (authButton != null) {
    authButton.addEventListener('click', AuthToggle);
    authLayout.addEventListener('click', AuthToggle);
}

function AuthToggle(evt) {
    if (evt.target === authLayout) {
        authLayout.classList.remove('authorizer_visible');
    } else {
        authLayout.classList.add('authorizer_visible');
    }
}

let layout = document.getElementsByClassName('header__user-actions-layout')[0];
let actions = document.getElementsByClassName('header__user-actions')[0];
let userPhoto = document.getElementsByClassName('header__user-photo')[0];

// userPhoto.addEventListener('click', actionsShowHide);
// layout.addEventListener('click', actionsShowHide);
//
//
// function actionsShowHide(evt) {
//     console.log(evt.target);
//     if (evt.target === userPhoto) {
//         layout.classList.add('header__user-actions-layout_visible');
//         actions.classList.add('header__user-actions_visible');
//     }
//     if (evt.target === layout) {
//         layout.classList.remove('header__user-actions-layout_visible');
//         actions.classList.remove('header__user-actions_visible');
//     }
// }

let alphabet = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
let alphabetHigh = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

let numbers = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

let login = document.getElementById('sign-up__login');

login.addEventListener('change', checkLogin);

function checkLogin() {
    login.classList.remove('authorizer__input_failed');
    for (let i = 0; i < login.value.length; i++) {
        if (!alphabet.includes(login.value.charAt(i))) {
            login.classList.add('authorizer__input_failed');
            regAllowed = 0;
            break;
        }
    }
    AllowReg()
}

let pass = document.getElementById('sign-up__password');
let repPass = document.getElementById('sign-up__repeat-password');

pass.addEventListener('change', checkPass);
repPass.addEventListener('change', checkRepPass);

function checkPass() {
    pass.classList.remove('authorizer__input_failed');
    if (pass.value.length < 8) {
        pass.classList.add('authorizer__input_failed');
    } else {
        for (let i = 0; i < pass.value.length; i++) {
            if (!numbers.includes(pass.value.charAt(i)) && !alphabet.includes(pass.value.charAt(i)) && !alphabetHigh.includes(pass.value.charAt(i))) {
                pass.classList.add('authorizer__input_failed');
                break;
            }
        }
    }
    AllowReg()
}

function checkRepPass() {
    repPass.classList.remove('authorizer__input_failed');
    if (repPass.value != pass.value) {
        repPass.classList.add('authorizer__input_failed');
    }
    AllowReg()
}
let email = document.getElementById('sign-up__email');
let accept = document.getElementById('accept-rules');
let checklist = [login, pass, repPass, email];

email.addEventListener('change', AllowReg);
accept.addEventListener('change', AllowReg);

let button = signUpForm.getElementsByClassName('sign-up__button')[0];

function AllowReg() {
    button.disabled = 1;
    let flag = 1;
    if (accept.checked === true) {
        for (let i = 0; i < checklist.length; i++) {
            if ((checklist[i].value === "") || (checklist[i].classList.contains('authorizer__input_failed'))){
                flag = 0;
                console.log('fuck u')
                break;
            }
        }
        if (flag === 1) {
            button.disabled = 0;
            console.log('evthorite');
        }
    }
}