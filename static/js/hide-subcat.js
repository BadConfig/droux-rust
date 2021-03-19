const hide_categories_div = document.getElementsByClassName('filters__sector_category')[0];
const hide_categories = hide_categories_div.getElementsByTagName('input');

for (let i =0; i < hide_categories.length; i++) {
    hide_categories[i].addEventListener('change', (e) => {
        if (document.getElementById('js_stylesheet') === null) {
            let styleSheet = document.createElement('style');
            styleSheet.id = "js_stylesheet";
            document.body.append(styleSheet);
        }
        let v = e.currentTarget.value;
        const ss = document.getElementById('js_stylesheet');
        ss.innerHTML = `
            .filters__checkbox-div_subcategory:not(div[data-parent="${ v }"]) {
                display: none;
            }
        `;
        if ((v !== '3') && (v !== '4')) {
            ss.innerHTML += `
                .filters__sector_size .filters__checkbox-div:not(div[data-parent="${ v }"]) {
                    display: none;
                }
            `;
        } else {
            ss.innerHTML += `
                .filters__sector_size .filters__checkbox-div:not(div[data-nosize="true"]) {
                        display: none;
                    }
            `;
        }
    })
}

const sizes_div = document.getElementsByClassName('filters__sector_size')[0];
const sizes_text = sizes_div.getElementsByTagName('label');
const sizes_size_div = sizes_div.getElementsByClassName('filters__checkbox-div');

for (let i=0; i < sizes_text.length; i++) {
    let text = sizes_text[i].innerText;
    if (text !== "no size") {
        if (text.includes('X') || text.includes('L') || text.includes('M') || text.includes('S')) {
            sizes_size_div[i].dataset.parent = "1";
        } else {
            sizes_size_div[i].dataset.parent = "2";
        }
    } else {
        sizes_size_div[i].dataset.nosize = "true";
    }
}

const sectors = document.getElementsByClassName('filters__sector');

if (document.getElementsByClassName('sort-by__discard')[0] !== null) {
    const discardButton = document.getElementsByClassName('sort-by__discard')[0];
    const discardButtonMobile = document.getElementsByClassName('sort-by__discard')[1];

    function discardFilters() {
        for (let i = 0; i < sectors.length; i++) {
            if (sectors[i].querySelector('input:checked') !== null) {
                sectors[i].querySelector('input:checked').checked = false;
            }
        }
    }
    if (discardButton !== null) {
        discardButton.addEventListener('click', discardFilters);
        discardButtonMobile.addEventListener('click', discardFilters);
    }
}
