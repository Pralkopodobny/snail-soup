use utoipa::{
    openapi::security::{Http, HttpAuthScheme, SecurityScheme},
    Modify,
};

pub struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "Bearer token",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        )
    }
}
