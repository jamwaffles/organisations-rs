use actix_web::error::{Error, ErrorUnauthorized};
use actix_web::{http::header::AUTHORIZATION, FromRequest, HttpRequest};
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

impl<S> FromRequest<S> for CurrentAuth {
    type Config = ();
    type Result = Result<Self, Error>;

    #[inline]
    fn from_request(req: &HttpRequest<S>, _: &Self::Config) -> Self::Result {
        let authorisation = req.headers().get(AUTHORIZATION);

        println!("AUTHORISATION: {:?}", authorisation);

        match authorisation {
            Some(_) => {
                if let Some(auth) = authorisation {
                    // TODO: Validate token
                    // TODO: Secret as env var
                    decode::<CurrentAuth>(
                        &auth.to_str().unwrap().split_whitespace().nth(1).unwrap(),
                        "super_secret_jam".as_ref(),
                        &Validation::default(),
                    ).map_err(|_| ErrorUnauthorized("JWT could not be decoded"))
                    .map(|t| t.claims)
                } else {
                    Err(ErrorUnauthorized("JWT not present"))
                }
            }
            _ => {
                println!("Authorisation is none");
                Err(ErrorUnauthorized("No authorisation given"))
            }
        }
    }
}
