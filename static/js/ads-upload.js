let searchRes = document.getElementsByClassName('search-results')[0];

setInterval(checkAndAdd,1000)

function checkAndAdd() {
    let currentBottom = document.documentElement.getBoundingClientRect().bottom;
    if (currentBottom < document.documentElement.clientHeight + 450) {
        console.log('hui');
        let newFrame = document.createElement('div');
        newFrame.innerText = 'feeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee';
        searchRes.append(newFrame);
    }
}