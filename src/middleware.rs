use crate::handlers::errors::AppError;
use crate::models::app::CurrentUser;

use axum::{extract::Request, middleware::Next, response::{Response}, Extension};
use axum::http::header::CACHE_CONTROL;
use axum::response::{IntoResponse, Redirect};
use tower_sessions::Session;

pub async fn authenticate(
    session: Session,
    mut req: Request,
    next: Next

) -> Result<Response, AppError>{
    let user_id = session.get::<i32>("authenticated_user_id").await?;

    let mut current_user = CurrentUser{
        is_authenticated: false,
        user_id: None
    };

    if let Some(id) = user_id {
         // User is authenticated, set user_id in current_user and pass it on to the next handler.
        current_user.is_authenticated = true;
        current_user.user_id = Some(id);

        req.extensions_mut().insert(current_user);
        Ok(next.run(req).await)
    } else {

        req.extensions_mut().insert(current_user);
        Ok(next.run(req).await)
    }
}

pub async fn required_authentication(
    Extension(current_user): Extension<CurrentUser>,
    req: Request,
    res: Next
) -> Response{
    if !current_user.is_authenticated {
        return Redirect::to("/log-in").into_response();
    }
    let mut res = res.run(req).await;
    res.headers_mut().insert(CACHE_CONTROL, "no-store".parse().unwrap());

    res
}
