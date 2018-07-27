use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessfulResponse<T: Serialize> {
    pub result: T,
}

#[derive(Serialize)]
pub struct HealthcheckSuccess {
    ok: bool,
}

impl HealthcheckSuccess {
    pub fn new() -> Self {
        Self { ok: true }
    }
}