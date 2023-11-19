use super::*;
use crate::store::Store;
use crate::errors::return_error;
use crate::race::AccessControlEngine;
use crate::rob::user::UserQuery;
use std::sync::Arc;
use warp::{http::Method, Filter, Rejection};

pub fn router(
    store: Arc<dyn Store>,
    race_core: Arc<AccessControlEngine>,
) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    let with_store = warp::any().map(move || store.clone());
    let with_access_control_core_access = warp::any().map(move || race_core.clone());

    macro_rules! route_with_body {
        ($method:ident, $function_name:ident, $path:expr) => {
            warp::$method()
                .and($path)
                .and(warp::path::end())
                .and(with_store.clone())
                .and(warp::body::json())
                .and_then($function_name)
        };
    }

    macro_rules! route_without_body {
        ($method:ident, $function_name:ident, $path:expr) => {
            warp::$method()
                .and($path)
                .and(warp::path::end())
                .and(with_store.clone())
                .and_then($function_name)
        };
    }

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "User-Agent",
            "Content-Type",
            "Authorization",
        ])
        .allow_methods(&[Method::GET, Method::POST, Method::DELETE, Method::PATCH]);

    let healthz_route = warp::get()
        .and(warp::path("healthz"))
        .and(warp::path::end())
        .and_then(healthz);

    let is_allowed_route = warp::post()
        .and(warp::path!("v1" / "isallowed"))
        .and(warp::path::end())
        .and(with_store.clone())
        .and(with_access_control_core_access.clone())
        .and(warp::body::json())
        .and_then(is_allowed);

    // tenants
    let add_tenant_route = route_with_body!(post, add_tenant, warp::path!("v1" / "tenants"));
    let update_tenant_route =
        route_with_body!(patch, update_tenant, warp::path!("v1" / "tenants" / String));
    let subscribe_tenant_to_product_route = route_without_body!(
        patch,
        subscribe_tenant_to_product,
        warp::path!("v1" / "tenants" / String / "subscribe" / String)
    );
    let get_tenant_route =
        route_without_body!(get, get_tenant, warp::path!("v1" / "tenants" / String));
    let get_tenants_route = route_without_body!(get, get_tenants, warp::path!("v1" / "tenants"));
    let delete_tenant_route = route_without_body!(
        delete,
        delete_tenant,
        warp::path!("v1" / "tenants" / String)
    );

    // users
    let add_user_route = route_with_body!(post, add_user, warp::path!("v1" / "users"));
    let update_user_route =
        route_with_body!(patch, update_user, warp::path!("v1" / "users" / String));
    let associate_user_with_tenant_route = route_without_body!(
        patch,
        associate_user_with_tenant,
        warp::path!("v1" / "users" / String / "associate" / String)
    );
    let get_user_route = route_without_body!(get, get_user, warp::path!("v1" / "users" / String));
    let get_users_route = warp::get()
        .and(warp::path!("v1" / "users"))
        .and(warp::path::end())
        .and(with_jwt.clone())
        .and(with_store.clone())
        .and(warp::query::<UserQuery>())
        .and_then(get_users);
    let delete_user_route =
        route_without_body!(delete, delete_user, warp::path!("v1" / "users" / String));
    let get_user_info_route =
        route_without_body!(get, get_user_info, warp::path!("v1" / "userinfo" / String));

    // roles
    let add_role_route = route_with_body!(post, add_role, warp::path!("v1" / "roles"));
    let update_role_route =
        route_with_body!(patch, update_role, warp::path!("v1" / "roles" / String));
    let get_role_route = route_without_body!(get, get_role, warp::path!("v1" / "roles" / String));
    let get_roles_route = route_without_body!(get, get_roles, warp::path!("v1" / "roles"));
    let delete_role_route =
        route_without_body!(delete, delete_role, warp::path!("v1" / "roles" / String));

    healthz_route
        .or(is_allowed_route)
        // tenants
        .or(add_tenant_route)
        .or(update_tenant_route)
        .or(subscribe_tenant_to_product_route)
        .or(get_tenant_route)
        .or(get_tenants_route)
        .or(delete_tenant_route)
        // users
        .or(add_user_route)
        .or(update_user_route)
        .or(associate_user_with_tenant_route)
        .or(get_user_route)
        .or(get_users_route)
        .or(delete_user_route)
        .or(get_user_info_route)
        // roles
        .or(add_role_route)
        .or(update_role_route)
        .or(get_role_route)
        .or(get_roles_route)
        .or(delete_role_route)
        // finish up
        .with(cors)
        .recover(return_error)
}
