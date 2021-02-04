const hide_categories_div = document.getElementsByClassName('filters__sector_category')[0];
const hide_sub_categories_div = document.getElementsByClassName('filters__sector_type')[0];

const hide_categories = hide_categories_div.getElementsByTagName('input');
const hide_sub_categories = hide_sub_categories_div.getElementsByTagName('input');

hide_categories.forEach((category) => {
    category.addEventListener('change', (e) => {
        if (document.getElementById('js_stylesheet') === null) {
            let styleSheet = document.createElement('style');
            styleSheet.id = "js_stylesheet";
            document.body.append(styleSheet);
        }
        document.getElementById('js_stylesheet').innerHTML = `
            input[name="sub_category_id"]:not(input[data-parent="${e.target.value}"]) + label {
                display: none;
            }
        `;

    })
})

