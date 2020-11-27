let m_sortButtons = document.getElementsByClassName('sort-buttons');

let m_filtersLayout = document.querySelector('.filters__layout');

m_filtersLayout.addEventListener('click', hide_m_filters);
m_sortButtons[0].addEventListener('click', hide_m_filters);
m_sortButtons[1].addEventListener('click', hide_m_filters);
function hide_m_filters(evt) {
    if (evt.target === m_sortButtons[0]) {
        m_filtersLayout.classList.add('m_active');
        m_filtersLayout.childNodes[1].classList.toggle('m_active');
    } else if (evt.target === m_sortButtons[1]) {
        m_filtersLayout.classList.add('m_active');
        m_filtersLayout.childNodes[3].classList.toggle('m_active');
    } else if (evt.target === m_filtersLayout) {
        m_filtersLayout.classList.remove('m_active');
        m_filtersLayout.childNodes[1].classList.remove('m_active');
        m_filtersLayout.childNodes[3].classList.remove('m_active');
    }
}