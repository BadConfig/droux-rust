const cropper_place = document.getElementsByClassName('cropper__main')[0];
const cropper_input = document.getElementsByClassName('cropper__input')[0];
const cropper_layout = document.getElementsByClassName('cropper__layout')[0];


const scissors = new Croppie(cropper_place, {
    viewport: {width: 200, height: 200, type: 'square'},
    boundary: {width: document.documentElement.clientWidth * 0.5, height: document.documentElement.clientHeight * 0.5},
    showZoomer: true,
    enableOrientation: false
});

function upload_photo() {
    let files = cropper_input.files;
    if (files.length === 1) {
        let file = files[0];
        if (file.type.match('image.*')) {
            file = URL.createObjectURL(file);
            scissors.bind({
                url: file
            })
            cropper_layout.classList.add('cropper__layout_visible');
        } else {
            alert('Разрешено загружать только фотографии!');
        }
    }
}

function upload_avatar() {
    let imageSize = {
        width: 300,
        height: 300,
        type: 'square'
    }
    scissors.result({
        type: 'blob',
        size: imageSize,
        format: 'png',
        quality: 0.6}).then((avatar) => {
            let data = new FormData();
            data.append('profile_photo', avatar);
            let avatar_request = new XMLHttpRequest();
            avatar_request.open('POST', '/users/profile_pictures/create');
            avatar_request.responseType = 'text';
            avatar_request.setRequestHeader("Content-Type", "multipart/form-data");
            avatar_request.send(data);
            let link = URL.createObjectURL(avatar);
            document.querySelector('.card__photo > img').src = link;
            document.querySelector('.header__user-photo').src = link;
            close_cropper();
        });
}

function close_cropper() {
    cropper_layout.classList.remove('cropper__layout_visible');
    cropper_input.value = null;
}

cropper_input.addEventListener('change', upload_photo);
cropper_layout.addEventListener('click', (e) => {
    if (e.target === cropper_layout) {
        close_cropper();
    }
} );
document.querySelector('.cropper__cross').addEventListener('click', close_cropper);