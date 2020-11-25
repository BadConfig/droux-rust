let uploader = document.getElementsByClassName('uploader')[0];
let arrowRight = uploader.getElementsByClassName('uploader__arrow_right')[0];
let arrowLeft = uploader.getElementsByClassName('uploader__arrow_left')[0];
let frames = uploader.getElementsByClassName('uploader__all-frames')[0];
let frame = frames.getElementsByClassName('uploader__photo-frame');
let uploaderInput = uploader.getElementsByClassName('uploader__input')[0];
let popUp = document.getElementsByClassName('pop-up')[0];
let popUpCross = popUp.getElementsByClassName('pop-up__close')[0];
let place = popUp.getElementsByClassName('pop-up__croppie-place')[0];
let cancel = popUp.getElementsByClassName('pop-up__button_cancel')[0];

let framesCount = frame.length;
let currentFirstFrame = 0;

let currentLastPhoto = frames.getElementsByClassName('uploader__frame-img').length - 1;

let label = document.createElement('label');
label.htmlFor = 'photo-upload-input';
label.className = 'uploader__minor-label';
frame[currentLastPhoto + 1].append(label);

arrowLeft.addEventListener('click', SwipeFrames);
arrowRight.addEventListener('click', SwipeFrames);

function GetWidth() {
    let frame = frames.getElementsByClassName('uploader__photo-frame')[0];
    let frameWidth = Number(getComputedStyle(frame).width.slice(0, -2));
    let frameMargin = Number(getComputedStyle(frame).marginRight.slice(0, -2));
    return (frameWidth + frameMargin);
}

function SwipeFrames() {
    if ((currentFirstFrame + 4 < framesCount) && (event.currentTarget === arrowRight)) {
        currentFirstFrame +=1;
    } else if ((currentFirstFrame > 0) && (event.currentTarget === arrowLeft)){
        currentFirstFrame -=1;
    }
    frames.style = "transform: translateX(-" + String(currentFirstFrame * GetWidth()) + "px);";
}

let crosses = document.getElementsByClassName('uploader__delete-frame');
for (let i = 0; i < crosses.length; i++) {
    crosses[i].addEventListener('click', DeleteFrame);
}
function DeleteFrame(){
    console.log('hui')
    let delta = GetWidth();
    frames.style = "transform: translateX(-" + String(currentFirstFrame * delta) + "px);";
    let delFrame = event.currentTarget.parentNode;
    delFrame.style = "transform: translateY(50px);";

    if (framesCount === 10) {
        let newFrame = document.createElement('div');
        newFrame.className = 'uploader__photo-frame';
        frames.append(newFrame);
        framesCount +=1;

        let label = document.createElement('label');
        label.htmlFor = 'photo-upload-input';
        label.className = 'uploader__minor-label';
        frame[10].append(label);

        uploaderInput.disabled = 0;
    }
    framesCount -= 1;
    currentLastPhoto -= 1;

    setTimeout(DelThis, 200);

    function DelThis(){
        delFrame.remove();
    }
}

popUp.addEventListener('click', ClosePopUp);
popUpCross.addEventListener('click', ClosePopUp);
cancel.addEventListener('click', ClosePopUp);

function ClosePopUp() {
    if ((event.target === popUp)||(event.target === popUpCross)||(event.target === cancel)) {
        popUp.classList.remove('pop-up_visible');
        uploaderInput.value = null;
    }
}

let scissors = new Croppie(place, {
    viewport: {width: document.documentElement.clientWidth * 0.64, height: document.documentElement.clientWidth * 0.57},
    boundary: {width: document.documentElement.clientWidth * 0.8, height: document.documentElement.clientHeight * 0.8},
    showZoomer: true,
    enableOrientation: false
})

uploaderInput.addEventListener('change', UploadPhoto);

function UploadPhoto() {
    if (currentLastPhoto < 9) {
        let files = uploaderInput.files;
        if (files.length === 1) {
            if (files[0].type.match('image.*')) {
                let file = URL.createObjectURL(files[0]);
                scissors.bind({
                    url: file
                })
                popUp.classList.add('pop-up_visible');
            } else {
                alert('Разрешено загружать только фотографии!');
            }
        }
    }
}

popUp.getElementsByClassName('pop-up__button_ok')[0].addEventListener('click', MakeMini);

function MakeMini(){
    scissors.result('blob', 'original').then(function(blob){

        currentLastPhoto += 1;

        let link = URL.createObjectURL(blob);
        let miniature = document.createElement('img');
        miniature.src = link;
        miniature.alt = '';
        miniature.className = 'uploader__frame-img';
        frame[currentLastPhoto].append(miniature);

        let frameCross = document.createElement('img');
        frameCross.src = 'assets/close.svg';
        frameCross.alt = '';
        frameCross.className = 'uploader__delete-frame';
        frame[currentLastPhoto].append(frameCross);
        frameCross.addEventListener('click', DeleteFrame);

        if (framesCount < 10){
            let newFrame = document.createElement('div');
            newFrame.className = 'uploader__photo-frame';
            frames.append(newFrame);
            framesCount +=1;
            currentFirstFrame = currentLastPhoto - 2;
        }

        frame[currentLastPhoto].getElementsByClassName('uploader__minor-label')[0].remove();
        if (currentLastPhoto < 9) {
            let label = document.createElement('label');
            label.htmlFor = 'photo-upload-input';
            label.className = 'uploader__minor-label'
            frame[currentLastPhoto + 1].append(label);
        }

        for (let i = 0; i < currentLastPhoto + 1; i++) {
            frame[i].id = "photo_" + String(i);
        }
        uploaderInput.value = null;
        if (currentLastPhoto === 9) {
            uploaderInput.disabled = 1;
        }
        frames.style = "transform: translateX(-" + String(currentFirstFrame * GetWidth()) + "px);";
    });
    popUp.classList.remove('pop-up_visible');
}


async function PostProduct() {
    let sex = document.getElementsByClassName('filters__sector_sex')[0];
    let category = document.getElementsByClassName('filters__sector_type')[0];
    let subcategory = document.getElementsByClassName('filters__sector_sex')[0];
    let brand = document.getElementsByClassName('filters__sector_brand')[0];
    let size = document.getElementsByClassName('filters__sector_size')[0];
    let name = document.getElementsByClassName('ad-form__field-div_name')[0];
    let description = document.getElementsByClassName('ad-form__field-div_description')[0];
    let state = document.getElementsByClassName('ad-form__state')[0];
    let price = document.getElementsByClassName('ad-form__price')[0];
    let number = document.getElementsByClassName('ad-form__num')[0];
    let email = document.getElementsByClassName('ad-form__email')[0];

    console.log("afsdaffas");
    let body = new FormData();
    body.append('type_id', sex.querySelector('input:checked').value);
    body.append('category_id', category.querySelector('input:checked').value);
    body.append('sub_category_id', subcategory.querySelector('input:checked').value);
    body.append('brand_id', brand.querySelector('input:checked').value);
    body.append('size_id', size.querySelector('input:checked').value);
    body.append('state_id', state.querySelector('input:checked').value);
    body.append('title', name.querySelector('input').value);
    body.append('descr', description.querySelector('textarea').value);
    body.append('price', price.querySelector('input').value);
    body.append('phone_number', number.querySelector('input').value);
    body.append('location', email.querySelector('input').value);
    body.append('seller_id', 1);

    let photos = document.getElementsByClassName('uploader__frame-img');
    console.log(photos);
    for (let i = 0; i < 10; i++) {
        let id = 'photo' + String(i + 1);
        if (photos[i] !=null) {
            let blob = await fetch(photos[i].src).then(r => r.blob())
            body.append(id, blob);
            console.log(blob);
        }
    }

    let postAd = new XMLHttpRequest();
    postAd.open('POST', '/product/create', true);
    postAd.send(body);
    postAd.onreadystatechange = function() {
        let redirectRoute = "/product/promotion/create/" + String(postAd.response);
        console.log(redirectRoute);
        window.location.replace(redirectRoute);
    }
    return false;
}

async function EditProduct() {
    let sex = document.getElementsByClassName('filters__sector_sex')[0];
    let category = document.getElementsByClassName('filters__sector_type')[0];
    let subcategory = document.getElementsByClassName('filters__sector_sex')[0];
    let brand = document.getElementsByClassName('filters__sector_brand')[0];
    let size = document.getElementsByClassName('filters__sector_size')[0];
    let name = document.getElementsByClassName('ad-form__field-div_name')[0];
    let description = document.getElementsByClassName('ad-form__field-div_description')[0];
    let state = document.getElementsByClassName('ad-form__state')[0];
    let price = document.getElementsByClassName('ad-form__price')[0];
    let number = document.getElementsByClassName('ad-form__num')[0];
    let email = document.getElementsByClassName('ad-form__email')[0];

    console.log("afsdaffas");
    let body = new FormData();
    body.append('type_id', sex.querySelector('input:checked').value);
    body.append('category_id', category.querySelector('input:checked').value);
    body.append('sub_category_id', subcategory.querySelector('input:checked').value);
    body.append('brand_id', brand.querySelector('input:checked').value);
    body.append('size_id', size.querySelector('input:checked').value);
    body.append('state_id', state.querySelector('input:checked').value);
    body.append('title', name.querySelector('input').value);
    body.append('descr', description.querySelector('textarea').value);
    body.append('price', price.querySelector('input').value);
    body.append('phone_number', number.querySelector('input').value);
    body.append('location', email.querySelector('input').value);
    body.append('seller_id', 1);

    let photos = document.getElementsByClassName('uploader__frame-img');
    console.log(photos);
    for (let i = 0; i < 10; i++) {
        let id = 'photo' + String(i + 1);
        if (photos[i] !=null) {
            let blob = await fetch(photos[i].src).then(r => r.blob())
            body.append(id, blob);
            console.log(blob);
        }
    }
    
    let prodId = document.getElementById('product_id').value;
    let route = "/admin/product/change/" + prodId
    console.log(route);
    let postAd = new XMLHttpRequest();
    let page = document.getElementById('page_num');
    postAd.open('POST', route, true);
    postAd.responseType = 'text';
    postAd.send(body);
    postAd.onreadystatechange = function() {
        let redirectRoute = "/admin/product/1";
        console.log(redirectRoute);
        window.location.replace(redirectRoute);
    }
    return false;
}

