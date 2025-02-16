use crate::{configuration, routes, telemetry};
use pavex::blueprint::Blueprint;
use pavex::f;
use pavex::kit::ApiKit;

/// The main blueprint, containing all the routes, middlewares, constructors and error handlers
/// required by our API.
pub fn blueprint() -> Blueprint {
    let mut bp = Blueprint::new();
    ApiKit::new().register(&mut bp);
    telemetry::register(&mut bp);
    configuration::register(&mut bp);
    routes::register(&mut bp);
    bp.singleton(f!(crate::template::compile_templates));
    bp.singleton(f!(crate::asset::CSSAsset::build_css_asset));

    bp
}
