let searchRes = document.getElementsByClassName('search-results')[0];

setInterval(checkAndAdd,1000)
let portions=1
function checkAndAdd() {
    let currentBottom = document.documentElement.getBoundingClientRect().bottom;
    if (currentBottom < document.documentElement.clientHeight + 450) {
        portions+=1;
        let request = new XMLHttpRequest();
        request.open("POST", '/filters/lots', true);
        request.setRequestHeader('Content-Type', 'application/x-www-form-urlencoded');
        let body ='limit=12&offset=' + String(portions * 12);
        request.send(body)
        console.log('her');
    }
}