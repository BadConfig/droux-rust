let field = document.getElementsByClassName('chat__field')[0];
let messages = document.getElementsByClassName('chat__message-div');

field.scrollTop = field.clientHeight;
let flag = 0;

for (let i = 0; i < messages.length; i++) {
    if (messages[i].classList.contains('chat__message-div_author_companion') && (flag === 0)) {
        messages[i].classList.add('chat__message-div_first');
        flag = 1;
    } else if (messages[i].classList.contains('chat__message-div_author_user')) {
        flag = 0
    }
}
