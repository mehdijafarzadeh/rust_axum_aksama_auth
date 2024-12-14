use askama::Template;

#[derive(Template)]
#[template(path="pages/home.html")]
pub struct HomeTemplate;

#[derive(Template)]
#[template(path="pages/create.html")]
pub struct CreateTemplate;

#[derive(Template)]
#[template(path="pages/log-in.html")]
pub struct LoginTemplate;

#[derive(Template)]
#[template(path="pages/notfound.html")]
pub struct NotFoundTemplate;

#[derive(Template)]
#[template(path="pages/server-error.html")]
pub struct ServerErrorTemplate;

#[derive(Template)]
#[template(path="pages/sign-up.html")]
pub struct SignUpTemplate;

#[derive(Template)]
#[template(path="pages/todos.html")]
pub struct TodosTemplate;