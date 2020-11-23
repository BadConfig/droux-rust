let searchResults = document.querySelector('.search-results');

let timer = setInterval(checkAndAdd,3000);

let portions = 0;

function checkAndAdd() {
    let currentBottom = document.documentElement.getBoundingClientRect().bottom;
    if (currentBottom < document.documentElement.clientHeight + 450) {
        portions+=1;
        let request = new XMLHttpRequest();
        request.open("POST", '/filters/lots', true);
        request.setRequestHeader('Content-Type', 'application/x-www-form-urlencoded');
        let body = 'search_string=&limit=12&offset=' + String(portions * 12);
        request.send(body)
        request.onreadystatechange = function() {
            jsonToAds(request.response);
        }
    }
}

let filters = document.getElementsByClassName('filters__sector-options');
for (let i = 0; i < filters.length; i++) {
    let options = filters[i].querySelectorAll('input[type=radio]');
    console.log(options);
    for (let j = 0; j < options.length; j++) {
        options[j].addEventListener('change', NewSearch);
    }
}

let headerSearchField = document.getElementById('header-search');
console.log(headerSearchField);
let headerSearchButton = document.querySelector('.search__button');

headerSearchField.addEventListener('change', NewSearch);
headerSearchButton.addEventListener('click', NewSearch);

function NewSearch() {
    let body = 'search_string=' + headerSearchField.value + '&limit=12' + '&offset=0' + '&type_id=';
    if (filters[0].querySelector('input:checked') != null) {
        body += filters[0].querySelector('input:checked').value;
    }
    if (filters[1].querySelector('input:checked') != null) {
        body += '&category_id=' + filters[1].querySelector('input:checked').value;
    } else {
        body += '&category_id=';
    }
    if (filters[2].querySelector('input:checked') != null) {
        body += '&sub_category_id=' + filters[2].querySelector('input:checked').value;
    } else {
        body += '&sub_category_id=';
    }
    if (filters[3].querySelector('input:checked') != null) {
        body += '&prod_brand_id=' + filters[3].querySelector('input:checked').value;
    } else {
        body += '&prod_brand_id=';
    }
    if (filters[4].querySelector('input:checked') != null) {
        body += '&prod_size_id=' + filters[4].querySelector('input:checked').value;
    } else {
        body += '&prod_size_id=';
    }
    if (filters[5].querySelector('input:checked') != null) {
        body += '&product_state_id=' + filters[5].querySelector('input:checked').value;
    } else {
        body += '&product_state_id=';
    }
    console.log(body);
    let request = new XMLHttpRequest();
    request.open("POST", '/filters/lots', true);
    request.setRequestHeader('Content-Type', 'application/x-www-form-urlencoded');
    let res = document.querySelector('.search-results');
    res.parentNode.removeChild(res);
    searchResults = document.createElement('div');
    searchResults.className = 'search-results';
    let main = document.querySelector('main');
    main.append(searchResults);
    request.send(encodeURI(body));
    request.onreadystatechange = function() {
        jsonToAds(request.response);
    }
}











































function jsonToAds(response) {
    let resp = JSON.parse(response);
    if (resp.length === 0) {
        clearInterval(timer);
    }
    for (let i = 0; i < resp.length; i++) {
        let newAd = document.createElement('div');
        let adLink = '/product/' + resp[i].id
        newAd.className = 'ad';
        newAd.innerHTML = "<div class=\"ad__img-container\"> <!--Рамка для одной фотографии-->\n" +
            "                    <div class=\"ad__arrow ad__arrow_left\">\n" +
            "                        <img src=\"../../static/assets/arrow-left.svg\" alt=\"Стрелка просмотра предыдущего фото\">\n" +
            "                    </div>\n" +
            "                    <div class=\"ad__all-images\"> <!--Все фото в линию-->\n" +
            "                    </div>\n" +
            "                    <div class=\"ad__arrow ad__arrow_right\">\n" +
            "                        <img src=\"../../static/assets/arrow-right.svg\" alt=\"Стрелка просмотра слюдующего фото\">\n" +
            "                    </div>\n" +
            "                    <div class=\"ad__favourite-icon\">\n" +
            "                        <img src=\"../../static/assets/favourite.svg\" alt=\"Иконка избранного\" class=\"ad__favourite-icon-img_empty\">\n" +
            "                        <img src=\"../../static/assets/favourite-filled.svg\" class=\"ad__favourite-icon-img_filled\">\n" +
            "                    </div>\n" +
            "                </div>\n" +
            "                <input type=\"hidden\" class=\"prod_id\" value=\"" + resp[i].id + "\">" +
            "                <div class=\"ad__name-and-size\">\n" +
            "                    <a href=\"" + adLink + "\"><h3 class=\"ad__name\">" + resp[i].title + "</h3></a>\n" +
            "                    <span class=\"ad__size\">" + resp[i].size_name + "</span>\n" +
            "                </div>\n" +
            "                <div class=\"ad__category\">" + resp[i].category_name + "</div>\n" +
            "                <div class=\"ad__price\">" + resp[i].price + "</div>"
        let imageDiv = newAd.querySelector('.ad__all-images');
        for (let j = 0; j < resp[i].pictures.length; j++) {
            let productPhoto = document.createElement('a');
            productPhoto.href = adLink;
            productPhoto.innerHTML = "<img src=\"" + resp[i].pictures[j] + "\" class=\"ad__img\" alt=\"\">";
            imageDiv.append(productPhoto);
        }
        if (resp[i].is_in_favourites) {
            newAd.querySelector('.ad__favourite-icon-img_filled').classList.add('fav-icon_active');
        } else {
            newAd.querySelector('.ad__favourite-icon-img_empty').classList.add('fav-icon_active');
        }
        searchResults.append(newAd);
    }
    checkAds();
}