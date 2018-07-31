use actix_web::middleware::{Middleware, Started};
use actix_web::{HttpRequest, HttpResponse, Result};
use middleware::CurrentAuth;

pub struct AdminOnly;

impl<S> Middleware<S> for AdminOnly {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        let exts = req.extensions();
        let token = exts.get::<CurrentAuth>();

        // Auth on presence of token
        // TODO: Validate contents of token
        match token {
            Some(_) => Ok(Started::Done),
            None => Ok(Started::Response(HttpResponse::Unauthorized().finish())),
        }
    }
}
