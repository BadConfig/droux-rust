extern crate data_encoding;
extern crate ring;
extern crate rand;

use diesel::PgConnection;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use data_encoding::HEXUPPER;
use ring::digest::{Context, SHA256};

pub fn send_auth_link(link: String, email: String, username: String) {
    
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{Message, SmtpTransport, Transport};
    use lettre::message::{header, MultiPart, SinglePart};


    let html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Hello from Lettre!</title>
</head>
<body>
    <div style="display: flex; flex-direction: column; align-items: center;">
        <p>Добрый день, {}\nЧтобы обезопасить свой аккаунт Вам необходимо подтвердить адрес электронной почты, указанный при регистрации профиля.
<a href="https://droux.ru{}">cсылкa для подтверждения</a>
Если вы уже подтвердили адрес электронной почты, Вы можете начать использовать 
весь функционал нашей платформы, подтверждать его снова не нужно.</p>
    </div>
</body>
</html>"#,username,link);

    println!("msg {}",html);
    let email = Message::builder()
    .from("noreply@droux.ru".parse().unwrap())
    .to(email.parse().unwrap())
    .subject("Verify your account")
    .multipart(
            MultiPart::alternative() // This is composed of two parts.
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType(
                            "text/plain; charset=utf8".parse().unwrap(),
                        ))
                        .body(format!("VERIFY LINK: {}",link).to_string()), 
                        // Every message should have a plain text fallback.
                )
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType(
                            "text/html; charset=utf8".parse().unwrap(),
                        ))
                        .body(html.to_string()),
                ),
        )
    .unwrap();

    let creds = Credentials::new("drouxgroup@gmail.com".to_string(), 
        "X5GYebjMARCR8".to_string());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp-pulse.com")
        .unwrap()
        .credentials(creds)
        .build();   

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    };
}

pub fn make_password_hash(email: String, nick: String, pass: String) -> String {
   
    let string_to_hash = nick + &email[..] + &pass[..];
    let mut context = Context::new(&SHA256);

    context.update(string_to_hash.as_bytes());
    let hash = context.finish();
    HEXUPPER.encode(hash.as_ref())

}

extern crate hmac;
extern crate sha2;
extern crate hex;
extern crate base64;
pub fn make_jwt_for_user(email: String, nick: String, pass: String) -> String {

    let json_str = format!("{{\"email\": \"{}\",
        \"nick\": \"{}\",
        \"pass\": \"{}\",
        \"created_at\": \"{}\"}}",
    email,
    nick,
    pass,
    chrono::Local::now().naive_local());
    let jwt = base64::encode(json_str.clone());

    use sha2::Sha256;
    use hmac::{Hmac, Mac, NewMac};
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_varkey(b"1259340652:AAFbIZbaGqTRxmPutAZ9ml7-tOQfMQr-CPU")
        .expect("HMAC can take key of any size");
    mac.update(jwt.as_bytes());
    let result = mac.finalize().into_bytes();
    let result = hex::encode(result);
    jwt+"."+&result[..]
}

pub fn get_data_from_jwt(jwt: String) -> Option<(Login,String,String)> {

    use serde_json::Value;
    use sha2::Sha256;
    use hmac::{Hmac, Mac, NewMac};
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_varkey(b"1259340652:AAFbIZbaGqTRxmPutAZ9ml7-tOQfMQr-CPU")
        .expect("HMAC can take key of any size");

    let ind = match jwt.find('.') {
        Some(i) => i,
        None => return None,
    };
    let base_data = jwt[..ind].to_string();
    let ver_token = jwt[ind+1..].to_string();
    let enc_vec = base64::decode(base_data.clone()).unwrap();
    mac.update(base_data.clone().as_bytes());
    let base_data = String::from_utf8(enc_vec).expect("error with encode");
   // let token = hex::decode(ver_token).unwrap();
// `verify` will return `Ok(())` if code is correct, `Err(MacError)` otherwise

    let result = mac.finalize().into_bytes();
// To get underlying array use `into_bytes` method, but be careful, since
// incorrect use of the code value may permit timing attacks which defeat
// the security provided by the `Output`
    let result = hex::encode(result);
     //   if mac.verify(ver_token.as_bytes()) == Result::Ok(()) {
     if result == ver_token {
         let proc_str = |s: &String| -> String {
             s[1..s.len()-1].to_string()
         };
        let jwt_data: Value = serde_json::from_str(&base_data[..]).expect("Error in parsing json");
        Some((Login::Username(
            proc_str(&jwt_data["nick"].to_string())),
            proc_str(&jwt_data["pass"].to_string()),
            proc_str(&jwt_data["created_at"].to_string())))
    } else {
        None
    }
}

pub fn generate_auth_link(conn: &PgConnection) -> String {
  
    use crate::db::auth::link_exists;
    let res =  || -> String {
        let string_to_hash: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();

        let mut context = Context::new(&SHA256);
        context.update(string_to_hash.as_bytes());
        let hash = context.finish();
        HEXUPPER.encode(hash.as_ref())
    };
    let mut var = res();
    while link_exists(var.clone(), conn) {
        var = res();
    }
    var

}

#[derive(Clone)]
pub enum Login {
    Email(String),
    Username(String),
}

extern crate regex;
use regex::Regex;

impl Login {
    pub fn new(login: String) -> Self {
        use Login::*;
        let r = Regex::new(r"[@]").unwrap();
        if r.is_match(&login[..]) {
            Email(login)
        } else {
            Username(login)
        }
    }
}


//auth states
use std::sync::atomic::AtomicUsize;

pub struct IsLogged {
    pub is_logged: AtomicUsize,
}

