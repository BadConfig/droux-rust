extern crate reqwest;
use serde_json::json;
use crate::Error;
use crate::routes::product::BuyForm;
use crate::routes::product::PrivForm;
use serde::{
    Serialize,
    Deserialize,
};


impl BuyForm {

    pub fn send_new_order_to_crm(&self) -> Result<(), Error> {

        let data = json!({
                "firstName": self.name,
                "lastName": self.username,
                "email": self.mail,
                "phone": self.phone,
                "delivery": {
                    "address": {
                        "index": self.post_index,
                        "text": self.ship_addr,
                    },
                    "items": [
                        {
                            "initialPrice": self.pr_price,
                            "productName": "Product",
                            "properties" : [
                                {
                                    "name": "SellerUserName",
                                    "value": self.seller_username,
                                },
                                {
                                    "name": "Phone",
                                    "value": self.seller_phone,
                                },
                                {
                                    "name": "Email",
                                    "value": self.seller_email,
                                },
                                {
                                    "name": "Location",
                                    "value": self.seller_location,
                                },
                                {
                                    "name": "IsPreOrder",
                                    "value": self.pr_is_pre_order,
                                },
                            ]
                        }
                    ]
                }
        });
        print!("my_json:\n{:#?}\n",&data.to_string());
        let params = [
            ("apiKey", "qZPXO3WaF2LLQI3YEZHdpJLoYZxhHdzH"),
            ("site", "droux.ru"),
            ("order", &data.to_string())];

        let client = reqwest::blocking::Client::new();
        let res = client.post("https://droux.retailcrm.ru/api/v5/orders/create")
            .form(&params)
            .send()?;
        print!("{:#?}\n",res);
        Ok(())
    }
}

#[derive(Serialize,Deserialize)]
pub enum TrDescription {
    Priveleges(PrivForm),
    Order(BuyForm),
    Unpayed,
}

impl PrivForm {
    pub fn send_sber_pay_link(&self, summ: i64) -> Result<String,Error> {

        let params = [
            ("userName", "T773007004660-api"),
            ("password", "T773007004660"),
            ("amount", &format!("{}",summ*100)[..]),
            ("currency", "643"),
            ("returnUrl", "http://localhost:8000/product/pay"),
            ("orderNumber", &format!("{}{}",self.product_name,self.product_id)[..]),
            ("description", &serde_json::to_string(&TrDescription::Priveleges(self.clone()))?[..])];

        let client = reqwest::blocking::Client::new();
        let res = client.post("https://3dsec.sberbank.ru/payment/rest/register.do")
            .form(&params)
            .send()?
            .text()?;
        let v: serde_json::Value = serde_json::from_str(&(res)[..])?;
        print!("{:#?}\n",v);
        let r = v["formUrl"].to_string();
        let len = r.len();
        let r = r[1..len-1].to_string();
        Ok(r)
    }
}

impl TrDescription {
    pub fn get_sber_pay_status(orderId: String) -> Result<TrDescription, Error> {

        let params = [
            ("userName", "T773007004660-api"),
            ("password", "T773007004660"),
            ("orderId", &orderId[..])];

        let client = reqwest::blocking::Client::new();
        let res = client.post("https://3dsec.sberbank.ru/payment/rest/getOrderStatusExtended.do")
            .form(&params)
            .send()?
            .text()?;
        print!("{:#?}\n",res);
        let v: serde_json::Value = serde_json::from_str(&(res)[..])?;
        let descr = v["orderDescription"].to_string();
        let descr = descr.replace("\\","");
        let dlen = descr.len()-1;
        let descr = &descr[1..dlen];
        print!("{}",descr);
        let descr: TrDescription = serde_json::from_str(descr)?;
        match descr {
            TrDescription::Priveleges(p) => {
                if v["orderStatus"] == 2 {
                    Ok(TrDescription::Priveleges(p))
                } else {
                    Ok(TrDescription::Unpayed)
                }
            },
            TrDescription::Order(o) => {
                if v["orderStatus"] == 1 {
                    Ok(TrDescription::Order(o))
                } else {
                    Ok(TrDescription::Unpayed)
                }
            },
            TrDescription::Unpayed => Ok(TrDescription::Unpayed),
        }
    }
}

impl BuyForm {
    pub fn send_sber_pre_pay_link(&self) -> Result<String,Error> {

        print!("here");
        let params = [
            ("userName", "T773007004660-api"),
            ("password", "T773007004660"),
            ("amount", &format!("{}",self.pr_price*100)),
            ("currency", &format!("{}",643)[..]),
            ("returnUrl", "http://localhost/product/pay"),
            ("orderNumber", &format!("{}{}order",self.pr_name,self.pr_id)[..]),
            ("description", &serde_json::to_string(&TrDescription::Order(self.clone()))?[..])];

        let client = reqwest::blocking::Client::new();
        let res = client.post("https://3dsec.sberbank.ru/payment/rest/registerPreAuth.do")
            .form(&params)
            .send()?
            .text()?;
            print!("{:#?}\n",res);
            let v: serde_json::Value = serde_json::from_str(&(res)[..])?;
            print!("{:#?}\n",v);
            let r = v["formUrl"].to_string();
            let len = r.len();
            let r = r[1..len-1].to_string();
            Ok(r)
    }
}


