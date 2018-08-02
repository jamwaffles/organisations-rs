use actix_web::Json;
use actix_web::{AsyncResponder, FutureResponse, HttpResponse, State};
use events::{Event, MembershipEdited, MembershipRole};
use eventstore::SaveEvent;
use futures::future::Future;
use responses::{GenericSuccess, SuccessfulResponse};
use uuid::Uuid;
use AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMembershipCmd {
    organisation_id: Uuid,
    user_id: Uuid,
    membership_role: MembershipRole,
}

pub fn update_membership(
    (payload, state): (Json<UpdateMembershipCmd>, State<AppState>),
) -> FutureResponse<HttpResponse> {
    // Store an event; ¯\_(ツ)_/¯

    println!("/edit-membership");
    let ev = MembershipEdited {
        organisation_id: payload.organisation_id,
        user_id: payload.user_id,
        membership_role: payload.membership_role.clone(),
    };

    println!("UpdateMembership: {:?}", ev);

    let q = SaveEvent::new(Event::MembershipEdited(ev), None);

    state
        .eventstore
        .send::<SaveEvent>(q.into())
        .from_err()
        .and_then(|_| {
            Ok(
                HttpResponse::Ok().json(SuccessfulResponse::<GenericSuccess> {
                    result: GenericSuccess::new(),
                }),
            )
        }).responder()
}
