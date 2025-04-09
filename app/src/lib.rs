use std::collections::HashSet;

use strum::IntoEnumIterator;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};
use time::OffsetDateTime;
use uuid::Uuid;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Blabla" />

        <Router>
            <main class="d-flex">
                <Transition>
                    <Show when=move || true>
                        aside
                    </Show>
                </Transition>
                <div class="flex-grow-1 position-relative d-flex flex-column">
                    <div>
                        <Routes fallback=move || "not_found".into_view()>
                            <Route path=path!("/") view=UserEdit />
                        </Routes>
                    </div>
                </div>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[component]
pub fn UserEdit() -> impl IntoView {
    let edit_email_disabled = RwSignal::new(true);
    let edit_password_disabled = RwSignal::new(true);

    let user_resource = Resource::new(move || (), async move |_| get_user().await);

    view! {
        <Suspense fallback=|| {
            view! {
                <tbody>
                    <div>Loading...</div>
                </tbody>
            }
        }>
            {move || match user_resource.get() {
                Some(Ok(Some(user))) => {
                    let user = StoredValue::new(user);
                    view! {
                        <div class="mt-3">
                            <h1>
                                <div class="d-flex justify-content-between">image</div>
                            </h1>

                            <form>
                                <ul class="nav nav-tabs" role="tablist">
                                    <li class="nav-item" role="presentation">
                                        <a
                                            href="#tabUserInformation"
                                            class="nav-link active"
                                            data-bs-toggle="tab"
                                            role="tab"
                                        >
                                            User information
                                        </a>
                                    </li>
                                    <li class="nav-item" role="presentation">
                                        <a
                                            href="#tabAccessRoles"
                                            class="nav-link"
                                            data-bs-toggle="tab"
                                            role="tab"
                                        >
                                            Access_roles
                                        </a>
                                    </li>
                                    <li class="nav-item" role="presentation">
                                        <a
                                            href="#tabEmails"
                                            class="nav-link"
                                            data-bs-toggle="tab"
                                            role="tab"
                                        >
                                            Emails
                                        </a>
                                    </li>
                                    <li class="nav-item" role="presentation">
                                        <a
                                            href="#tabUserBrowsers"
                                            class="nav-link"
                                            data-bs-toggle="tab"
                                            role="tab"
                                        >
                                            Web browsers
                                        </a>
                                    </li>
                                </ul>
                                <div class="tab-content">
                                    <div
                                        id="tabUserInformation"
                                        class="tab-pane pt-3 fade active show"
                                        role="tabpanel"
                                    >
                                        <div class="mb-3 row">
                                            <label class="col-sm-2 col-form-label text-sm-end required">
                                                First name
                                            </label>
                                            <div class="col-sm">
                                                <input
                                                    type="text"
                                                    class="form-control pristine"
                                                    value=user.with_value(|user| user.first_name.clone())
                                                    data-pristine-value="Adrian-Mihai"
                                                />
                                            </div>
                                        </div>
                                        <div class="mb-3 row">
                                            <label class="col-sm-2 col-form-label text-sm-end">
                                                Last name
                                            </label>
                                            <div class="col-sm">
                                                <input
                                                    type="text"
                                                    class="form-control pristine"
                                                    value=user.with_value(|user| user.last_name.clone())
                                                />
                                            </div>
                                        </div>
                                        <div class="mb-3 row">
                                            <label class="col-sm-2 col-form-label text-sm-end required disabled">
                                                Login email
                                            </label>
                                            <div class="col-sm">
                                                <div class="input-group">
                                                    <input
                                                        type="password"
                                                        class="password form-control pristine"
                                                        autocomplete="off"
                                                        value=move || ""
                                                        prop:disabled=edit_password_disabled
                                                        id=move || "blabla"
                                                        maxlength=move || 199
                                                        name=move || "name"
                                                        placeholder=move || "placeholder"
                                                        required=move || false
                                                        autofocus=move || true
                                                        class:border-primary=move || !edit_password_disabled.get()
                                                        class:border-2=move || !edit_password_disabled.get()
                                                        minlength=move || 8
                                                        size=move || 20
                                                    />
                                                    <button
                                                        type="button"
                                                        on:click=move |_| {
                                                            edit_email_disabled.set(!edit_email_disabled.get())
                                                        }
                                                        class="btn btn-secondary"
                                                    >
                                                        Edit
                                                    </button>
                                                </div>
                                            </div>
                                        </div>
                                        <div class="mb-3 row">
                                            <label class="col-sm-2 col-form-label text-sm-end">
                                                Registered on
                                            </label>
                                            <div class="col-sm">
                                                <p class="form-control-plaintext">date</p>
                                            </div>
                                        </div>
                                        <div class="row">
                                            <div class="col-md">
                                                <div class="mb-3 row">
                                                    <label class="col-sm-2 col-md-4 col-form-label text-sm-end">
                                                        Last login
                                                    </label>
                                                    <div class="col-sm">
                                                        <p class="form-control-plaintext">date</p>
                                                    </div>
                                                </div>
                                            </div>
                                            <div class="col-md">
                                                <div class="mb-3 row">
                                                    <label class="col-sm-2 col-md-4 col-form-label text-sm-end">
                                                        Last failed login
                                                    </label>
                                                    <div class="col-sm">
                                                        <p class="form-control-plaintext">date</p>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                        <div class="mb-3 row">
                                            <label class="col-sm-2 col-form-label text-sm-end">
                                                Status user
                                            </label>
                                            <div class="col-sm">
                                                <select class="form-select w-auto pristine">
                                                    {move || {
                                                        UserStatus::iter()
                                                            .map(|status| {
                                                                view! {
                                                                    <option
                                                                        prop:selected=move || {
                                                                            user.with_value(|user| user.status == status)
                                                                        }
                                                                        value=status as u8
                                                                    >
                                                                        {format!("{status}")}
                                                                    </option>
                                                                }
                                                            })
                                                            .collect_view()
                                                    }}
                                                </select>
                                            </div>
                                        </div>
                                        <div class="mb-3 row">
                                            <label class="col-sm-2 col-form-label text-sm-end disabled">
                                                New password
                                            </label>
                                            <div class="col-sm">
                                                <div class="input-group">
                                                    <input
                                                        type="password"
                                                        class="password form-control pristine"
                                                        autocomplete="off"
                                                        value=move || ""
                                                        prop:disabled=edit_password_disabled
                                                        id=move || "blabla"
                                                        maxlength=move || 199
                                                        name=move || "name"
                                                        placeholder=move || "placeholder"
                                                        required=move || false
                                                        autofocus=move || true
                                                        class:border-primary=move || !edit_password_disabled.get()
                                                        class:border-2=move || !edit_password_disabled.get()
                                                        minlength=move || 8
                                                        size=move || 20
                                                        pattern=move || "^(?=.*[a-z])(?=.*[A-Z])(?=.*\\d)[a-zA-Z\\d]{8,}$"
                                                        readonly=move || false
                                                        form=move || "login-form"
                                                        aria-label=move || "Password"
                                                        aria-describedby=move || "password-help"
                                                        data-validate=move || true
                                                        tabindex=move || 0
                                                        on:change=move|_| leptos::leptos_dom::logging::console_log("Password changed")
                                                        on:input=move |_| leptos::leptos_dom::logging::console_log("Password input")
                                                    />
                                                    <input
                                                        type="password"
                                                        class="password form-control pristine"
                                                        autocomplete="off"
                                                        value=move || ""
                                                        prop:disabled=edit_password_disabled
                                                        id=move || "blabla"
                                                        maxlength=move || 199
                                                        name=move || "name"
                                                        placeholder=move || "placeholder"
                                                        required=move || false
                                                        autofocus=move || true
                                                        class:border-primary=move || !edit_password_disabled.get()
                                                        class:border-2=move || !edit_password_disabled.get()
                                                    />
                                                    <button
                                                        type="button"
                                                        on:click=move |_| {
                                                            edit_password_disabled.set(!edit_password_disabled.get())
                                                        }
                                                        class="btn btn-secondary"
                                                    >
                                                        <i class="fa-solid fa-edit"></i>
                                                        Edit
                                                    </button>
                                                </div>
                                            </div>
                                        </div>

                                        <Show when=move || !edit_password_disabled.get()>
                                            <div class="mb-3 row">
                                                <label class="col-sm-2 col-form-label text-sm-end disabled">
                                                    Retype new password
                                                </label>
                                                <div class="col-sm">
                                                    <div class="input-group">
                                                        <input
                                                            type="password"
                                                            class="password form-control pristine"
                                                            autocomplete="off"
                                                            value=""
                                                            prop:disabled=edit_password_disabled
                                                            minlength="6"
                                                            data-pristine-value=""
                                                        />
                                                    </div>
                                                </div>
                                            </div>
                                        </Show>

                                        <div class="mb-3 row">
                                            <label class="col-sm-2 col-form-label text-sm-end">
                                                Personal record
                                            </label>
                                            <div class="col-sm">
                                                <ul class="list-unstyled mt-2">
                                                    <li>
                                                        <a href="">Link to</a>
                                                    </li>
                                                </ul>

                                            </div>
                                        </div>
                                    </div>

                                    <div
                                        id="tabAccessRoles"
                                        class="tab-pane fade pt-3 accesses"
                                        role="tabpanel"
                                    >
                                    </div>
                                    <div
                                        id="tabEmails"
                                        class="tab-pane fade pt-3"
                                        role="tabpanel"
                                    >
                                    </div>
                                    <div
                                        id="tabUserBrowsers"
                                        class="tab-pane fade pt-3"
                                        role="tabpanel"
                                    >
                                    </div>
                                </div>
                            </form>
                        </div>
                    }
                        .into_any()
                }
                Some(Ok(None)) => {
                    view! {
                        <div>
                            <p>"Couldn't get user"</p>
                        </div>
                    }
                        .into_any()
                }
                Some(Err(err)) => {
                    view! {
                        <div>
                            <p>"Server Error: " <span>{format!("{err:?}")}</span></p>
                        </div>
                    }
                        .into_any()
                }
                None => view! { <div></div> }.into_any(),
            }}
        </Suspense>
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub unid: Uuid,
    pub created: OffsetDateTime,
    pub first_name: Option<String>,
    pub hash: String,
    pub last_failed_login: Option<OffsetDateTime>,
    pub last_login: Option<OffsetDateTime>,
    pub last_password_change: OffsetDateTime,
    pub last_name: Option<String>,
    pub login: String,
    pub roles: HashSet<String>,
    pub site_schema: Option<String>,
    pub status: UserStatus,
    pub theme: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Display, PartialEq, Eq, Hash, EnumIter)]
pub enum UserStatus {
    Active,
    Banned,
}

impl User {
    pub fn get_user() -> Self {
        User {
            unid: Uuid::new_v4(),
            created: OffsetDateTime::now_utc(),
            first_name: Some("Bob".to_string()),
            hash: "asdf".to_string(),
            last_failed_login: None,
            last_login: None,
            last_password_change: OffsetDateTime::now_utc(),
            last_name: None,
            login: "bob@bob.bob".to_string(),
            roles: HashSet::new(),
            site_schema: None,
            status: UserStatus::Active,
            theme: "dark".to_string(),
        }
    }
}

#[server]
async fn get_user() -> Result<Option<User>, ServerFnError> {
    Ok(Some(User::get_user()))
}
