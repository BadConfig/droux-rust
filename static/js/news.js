const article = document.getElementsByClassName("article__text")[0];
article.innerHTML = el.innerHTML.replaceAll("&lt;", "<").replace("&gt;", ">");