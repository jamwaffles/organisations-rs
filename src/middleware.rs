use actix_web::http::header::AUTHORIZATION;
use actix_web::middleware::{Middleware, Started};
use actix_web::{HttpRequest, Result};
use events::{MembershipRole, MembershipStatus, OrganisationType};
use jsonwebtoken::{decode, Validation};
use uuid::Uuid;

/// JWT struct

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthMembership {
    pub organisation_id: Uuid,
    pub organisation_type: OrganisationType,
    pub membership_status: MembershipStatus,
    pub membership_role: MembershipRole,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentAuth {
    user_id: Uuid,
    name: String,
    email: String,
    pub memberships: Vec<AuthMembership>,
}

pub struct InjectJwt;

impl<S> Middleware<S> for InjectJwt {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        let authorisation = req.headers().get(AUTHORIZATION);

        if let Some(auth) = authorisation {
            // TODO: Validate token
            // TODO: Secret as env var
            let token = decode::<CurrentAuth>(
                &auth.to_str().unwrap().split_whitespace().nth(1).unwrap(),
                "super_secret_jam".as_ref(),
                &Validation::default(),
            );

            match token {
                Ok(token) => {
                    req.extensions_mut().insert(token);
                }
                Err(err) => {
                    println!("Token decode error {}", err.description());
                }
            }
        }

        Ok(Started::Done)
    }
}
