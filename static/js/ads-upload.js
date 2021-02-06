let searchResults = document.querySelector('.search-results');

let timer = setInterval(checkAndAdd,3000);

let portions = 1;
let filtersActive = false;
let body;
let stopItFlag = false;
let address = window.location.href;
if (address.includes("order_by=Date")) {
    filtersActive = true;
    document.getElementById('date').checked = true;
    document.getElementById('-m-date').checked = true;
    body = "search_string=&limit=12&order_by=Date";
} else if (address.includes("order_by=Views")) {
    filtersActive = true;
    document.getElementById('popularity').checked = true;
    document.getElementById('-m-popularity').checked = true;
    body = "search_string=&limit=12&order_by=Views";
}

if (address.includes("prod_type_id=1")) {
    filtersActive = true;
    document.getElementById('ad_types1').checked = true;
    body = "limit=12&prod_type_id=1";
} else if (address.includes("prod_type_id=2")) {
    filtersActive = true;
    document.getElementById('ad_types2').checked = true;
    body = "limit=12&prod_type_id=2";
}

let categories = document.querySelectorAll('input[name=\"category_id\"]');
for (let i = 0; i < categories.length; i++) {
    if (address.includes("&category_id=" + (i + 1))) {
        filtersActive = true;
        document.getElementById('ad_cat' + (i + 1)).checked = true;
        body = "limit=12&category_id=" + (i + 1);
    }
}

let subcategories = document.querySelectorAll('input[name=\"sub_category_id\"]');
for (let i = 0; i < subcategories.length; i++) {
    if (address.includes("subcategory_id=" + (i + 1))) {
        filtersActive = true;
        document.getElementById('ad_sub_cat' + (i + 1)).checked = true;
        body = "limit=12&subcategory_id=" + (i + 1);
    }
}


function checkAndAdd() {
    let currentBottom = document.documentElement.getBoundingClientRect().bottom;
    if ((currentBottom < document.documentElement.clientHeight + 450) && (!stopItFlag)){
        stopItFlag = true;
        let request = new XMLHttpRequest();
        request.open("POST", '/filters/lots', true);
        request.setRequestHeader('Content-Type', 'application/x-www-form-urlencoded');
        if (filtersActive) {
            request.send(body + '&offset=' + (12 * portions));
            console.log(body + '&offset=' + (12 * portions));
        } else {
            body = 'search_string=&limit=12' + '&offset=' + (12 * portions);
            console.log(body);
            request.send(body)
        }
        portions+=1;
        request.onload = function() {
            console.log(request.response)
            jsonToAds(request.response);
            changeSize();
        }
    }
}

let filters = document.getElementsByClassName('filters__sector-options');
for (let i = 0; i < filters.length; i++) {
    let options = filters[i].querySelectorAll('input[type="radio"]');
    for (let j = 0; j < options.length; j++) {
        options[j].addEventListener('change', NewSearch);
    }
}

let sort = document.querySelector('div.sort-by');
let mobileSort = document.querySelector('aside.sort-by');

let sortOptions =sort.getElementsByTagName('input');
let mobileSortOptions = mobileSort.getElementsByTagName('input');

for (let i = 0; i < sortOptions.length; i++) {
    sortOptions[i].addEventListener('change', NewSearch);
    mobileSortOptions[i].addEventListener('change', NewSearch);
}

let headerSearchField = document.getElementById('header-search');
let headerSearchButton = document.querySelector('.search__button');

headerSearchField.addEventListener('input', NewSearch);
headerSearchButton.addEventListener('click', NewSearch);
let timeout = 0;
function NewSearch() {
    if (timeout != 0) {
        clearTimeout(timeout);
    }
    timer = setInterval(checkAndAdd,3000);
    timeout = setTimeout(useFilters, 1000);
}
function useFilters() {
    filtersActive = true;
    stopItFlag = true;
    portions = 0;
    body = 'limit=12';
    if (filters[0].querySelector('input:checked') != null) {
        body += '&prod_type_id=' + filters[0].querySelector('input:checked').value;
    }
    if (filters[1].querySelector('input:checked') != null) {
        body += '&category_id=' + filters[1].querySelector('input:checked').value;
    }
    if (filters[2].querySelector('input:checked') != null) {
        body += '&subcategory_id=' + filters[2].querySelector('input:checked').value;
    }
    if (filters[3].querySelector('input:checked') != null) {
        body += '&prod_brand_id=' + filters[3].querySelector('input:checked').value;
    }
    if (filters[4].querySelector('input:checked') != null) {
        body += '&prod_size_id=' + filters[4].querySelector('input:checked').value;
    }
    if (filters[5].querySelector('input:checked') != null) {
        body += '&product_state_id=' + filters[5].querySelector('input:checked').value;
    }
    if (document.documentElement.clientWidth >= 1200) {
        body += '&order_by=' + sort.querySelector('input:checked').value;
    } else {
        body += '&order_by=' + mobileSort.querySelector('input:checked').value;
    }
    if ((headerSearchField.value != "") || (body.length < 30)) { //ПЕРВОЕ, ЧТО МОЖЕТ СЛОМАТЬСЯ
        body += '&search_string=' + headerSearchField.value;
    }
    let res = document.querySelector('.search-results');
    res.parentNode.removeChild(res);
    searchResults = document.createElement('div');
    searchResults.className = 'search-results';
    let main = document.querySelector('main');
    main.append(searchResults);
    let request = new XMLHttpRequest();
    request.open("POST", '/filters/lots', true);
    request.setRequestHeader('Content-Type', 'application/x-www-form-urlencoded');
    request.send(body  + '&offset=' + (12 * portions));
    console.log(body);
    request.onload = function() {
        console.log(request.response)
        jsonToAds(request.response);
    }
    portions += 1;
    timeout = 0;

}


function jsonToAds(response) {
    let resp = JSON.parse(response);
    if (resp.length < 12) {
        clearInterval(timer);
    }
    for (let i = 0; i < resp.length; i++) {
        console.log(i);
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
            "                <div class=\"ad__price\">" + resp[i].price + "₽</div>"
        let imageDiv = newAd.querySelector('.ad__all-images');
        for (let j = 0; j < resp[i].pictures.length; j++) {
            let productPhoto = document.createElement('a');
            productPhoto.href = adLink;
            productPhoto.innerHTML = "<img src=\"" + resp[i].pictures[j] + "\" class=\"ad__img\" alt=\"\">";
            imageDiv.append(productPhoto);
        }
        let favDiv = newAd.querySelector('.ad__favourite-icon');
        if (document.getElementById('auth-button')) {
            favDiv.style.display = 'none';
        }
        if (resp[i].is_in_favourites) {
            newAd.querySelector('.ad__favourite-icon-img_filled').classList.add('fav-icon_active');
        } else {
            newAd.querySelector('.ad__favourite-icon-img_empty').classList.add('fav-icon_active');
        }
        searchResults.append(newAd);
    }
    if (resp.length > 0) {
        console.log(resp.length);
        checkAds();
        listenFav();
        changeSize();
    }
    stopItFlag = false;
}