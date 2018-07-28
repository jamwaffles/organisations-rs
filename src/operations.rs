use actix_web::{HttpRequest, HttpMessage, HttpResponse, AsyncResponder, Error};
use events::{AccountInvitedToOrg};
use responses::{HealthcheckSuccess, InviteUserSuccess, SuccessfulResponse};
use serde_json::{from_slice};
use bytes::Bytes;
use futures::future::Future;

pub fn health(_req: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(SuccessfulResponse {
        result: HealthcheckSuccess::new(),
    })
}

pub fn invite_user(req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
    req.body()
        .limit(1024)
        .from_err()
        .and_then(|bytes: Bytes| {
            println!("==== BODY ==== {:?}", body);
            let event: AccountInvitedToOrg = from_slice(&bytes).expect("Event convert");
            Ok(HttpResponse::Ok().json(SuccessfulResponse {
                result: InviteUserSuccess::new(),
            }).into())
        }).responder()
}
