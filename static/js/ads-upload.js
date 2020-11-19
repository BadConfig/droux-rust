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
        let body = new FormData();
        body.append('search_string', '');
        body.append('limit', 12);
        let offset = portions * 12;
        body.append('offset', offset);
        request.send(body)
        request.onreadystatechange = function() {
            console.log(request.response);
        }
    }
}