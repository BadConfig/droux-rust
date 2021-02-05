const hide_categories_div = document.getElementsByClassName('filters__sector_category')[0];
const hide_categories = hide_categories_div.getElementsByTagName('input');

for (let i =0; i < hide_categories.length; i++) {
    hide_categories[i].addEventListener('change', (e) => {
        if (document.getElementById('js_stylesheet') === null) {
            let styleSheet = document.createElement('style');
            styleSheet.id = "js_stylesheet";
            document.body.append(styleSheet);
        }
        document.getElementById('js_stylesheet').innerHTML = `
            .filters__checkbox-div_subcategory:not(div[data-parent="${e.currentTarget.value}"]) {
                display: none;
            }
        `;
    })
}

const sectors = document.getElementsByClassName('filters__sector');

if (document.getElementsByClassName('sort-by__discard')[0] !== null) {
    const discardButton = document.getElementsByClassName('sort-by__discard')[0];
    const discardButtonMobile = document.getElementsByClassName('sort-by__discard')[1];

    function discardFilters() {
        for (let i = 0; i < sectors.length; i++) {
            if (sectors[i].queryS('input:checked').checked !== null) {
                sectors[i].querySelector('input:checked').checked = false;
            }
        }
    }

    discardButton.addEventListener('click', discardFilters);
    discardButtonMobile.addEventListener('click', discardFilters);
}
