use crate::model::product::Product;
use crate::model::subscriber::Subscriber;
use crate::model::{notification::Notification, subscriber};
use crate::repository::subscriber::SubscriberRepository;
use bambangshop::{compose_error_response, Result};
use rocket::http::Status;
use std::thread;

pub struct NotificationService;

impl NotificationService {
    pub fn subscribe(product_type: &str, subscriber: Subscriber) -> Result<Subscriber> {
        let product_type_upper = product_type.to_uppercase();
        let product_type_str = product_type_upper.as_str();
        let subscriber_result = SubscriberRepository::add(product_type_str, subscriber);
        return Ok(subscriber_result);
    }

    pub fn unsubscribe(product_type: &str, url: &str) -> Result<Subscriber> {
        let product_type_upper = product_type.to_uppercase();
        let product_type_str = product_type_upper.as_str();
        let result = SubscriberRepository::delete(product_type_str, url);
        if result.is_none() {
            return Err(compose_error_response(
                Status::NotFound,
                "Subscriber not found".to_string(),
            ));
        }
        return Ok(result.unwrap());
    }
}
