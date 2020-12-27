const article = document.getElementsByClassName("article__text")[0];
article.innerHTML = article.innerHTML.replaceAll("&lt;", "<").replaceAll("&gt;", ">");