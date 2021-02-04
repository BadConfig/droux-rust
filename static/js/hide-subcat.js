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
            input[name="sub_category_id"]:not(input[data-parent="${e.currentTarget.value}"]) + label {
                display: none;
            }
        `;
    })
}

