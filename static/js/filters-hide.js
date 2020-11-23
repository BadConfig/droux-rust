let m_sortButtons = document.getElementsByClassName('sort-buttons');
console.log(m_sortButtons);

let m_filtersLayout = document.querySelector('.filters__layout');
console.log(m_filtersLayout);

m_filtersLayout.addEventListener('click', hide_m_filters);
m_sortButtons[0].addEventListener('click', hide_m_filters);
m_sortButtons[1].addEventListener('click', hide_m_filters);

console.log(m_filtersLayout.childNodes[1]);
console.log(m_filtersLayout.childNodes[3]);

function hide_m_filters(evt) {
    console.log(evt.target);
    if (evt.target === m_sortButtons[0]) {
        m_filtersLayout.classList.add('m_active');
        m_filtersLayout.childNodes[1].classList.toggle('m_active');
        console.log(m_filtersLayout.childNodes[1]);
    } else if (evt.target === m_sortButtons[1]) {
        m_filtersLayout.classList.add('m_active');
        m_filtersLayout.childNodes[3].classList.toggle('m_active');
    } else if (evt.target === m_filtersLayout) {
        m_filtersLayout.classList.remove('m_active');
        m_filtersLayout.childNodes[1].classList.remove('m_active');
        m_filtersLayout.childNodes[3].classList.remove('m_active');
    }
}