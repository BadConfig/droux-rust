let searchResults = document.querySelector('.search-results');

setInterval(checkAndAdd,3000);

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
            for (let i = 0; i < request.response.length; i++) {
                let newAd = document.createElement('div');
                let adLink = '/product/' + request.response[i].id
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
                    "                <input type=\"hidden\" class=\"prod_id\" value=\"" + request.response[i].id + "\">" +
                    "                <div class=\"ad__name-and-size\">\n" +
                    "                    <a href=\"" + adLink + "\"><h3 class=\"ad__name\">" + request.response[i].title + "</h3></a>\n" +
                    "                    <span class=\"ad__size\">" + request.response[i].size_name + "</span>\n" +
                    "                </div>\n" +
                    "                <div class=\"ad__category\">" + request.response[i].category_name + "</div>\n" +
                    "                <div class=\"ad__price\">" + request.response[i].price + "</div>"
                let imageDiv = newAd.querySelector('.ad__all-images');
                for (let j = 0; j < request.response[i].pictures.length; j++) {
                    let productPhoto = document.createElement('a');
                    productPhoto.href = adLink;
                    productPhoto.innerHTML = "<img src=\"" + request.response[i].pictures[j] + "\" class=\"ad__img\" alt=\"\">";
                    imageDiv.append(productPhoto);
                }
                if (request.response[i].is_in_favourites) {
                    newAd.querySelector('.ad__favourite-icon-img_filled').classList.add('fav-icon_active');
                } else {
                    newAd.querySelector('.ad__favourite-icon-img_empty').classList.add('fav-icon_active');
                }
                searchResults.append(newAd);
            }
            checkAds();
        }
    }
}