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

let framesCount = 1;
let currentFirstFrame = 0;
let currentLastPhoto = -1;

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

function DeleteFrame(){
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
    viewport: {width: 350, height: 311.5},
    boundary: {width: 350, height: 350},
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







