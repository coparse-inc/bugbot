mod env;
mod github;
mod parse;
mod people;
use anyhow::Result;
use env::config_env_var;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response};
use slack_morphism::prelude::*;
use tracing::*;

use std::net::SocketAddr;
use std::sync::Arc;

async fn oauth_install_handler(
    resp: SlackOAuthV2AccessTokenResponse,
    _client: Arc<SlackHyperClient>,
    _states: SlackClientEventsUserState,
) {
    println!("{:#?}", resp);
}

async fn command_events_handler(
    event: SlackCommandEvent,
    _client: Arc<SlackHyperClient>,
    _states: SlackClientEventsUserState,
) -> Result<SlackCommandEventResponse, Box<dyn std::error::Error + Send + Sync>> {
    let text = event.text.clone().unwrap_or("".into());

    let response_text: String = match parse::Command::from_str(text) {
        Ok(x) => x.into_response_text().await,
        Err(x) => x.render().to_string(),
    };

    Ok(
        SlackCommandEventResponse::new(SlackMessageContent::new().with_text(response_text))
            .with_response_type(SlackMessageResponseType::InChannel),
    )
}

fn error_handler(
    err: Box<dyn std::error::Error + Send + Sync>,
    _client: Arc<SlackHyperClient>,
    _states: SlackClientEventsUserState,
) -> http::StatusCode {
    println!("{:#?}", err);

    // Defines what we return Slack server
    http::StatusCode::BAD_REQUEST
}

#[derive(Debug)]
struct UserStateExample(u64);

fn get_port() -> u16 {
    config_env_var("PORT")
        .and_then(|s| s.parse::<u16>().map_err(anyhow::Error::from))
        .unwrap_or(8080)
}

fn get_addr() -> SocketAddr {
    match config_env_var("RAILWAY_ENVIRONMENT") {
        Ok(_) => std::net::SocketAddr::from(([0, 0, 0, 0], get_port())),
        Err(_) => std::net::SocketAddr::from(([127, 0, 0, 1], 8080)),
    }
}

async fn server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client: Arc<SlackHyperClient> =
        Arc::new(SlackClient::new(SlackClientHyperConnector::new()));

    let addr = get_addr();
    info!("Loading server: {}", addr);

    async fn your_others_routes(
        _req: Request<Body>,
    ) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
        Response::builder()
            .status(404)
            .body("Not Found".into())
            .map_err(|e| e.into())
    }

    let oauth_listener_config = Arc::new(SlackOAuthListenerConfig::new(
        config_env_var("SLACK_CLIENT_ID")?.into(),
        config_env_var("SLACK_CLIENT_SECRET")?.into(),
        config_env_var("SLACK_BOT_SCOPE")?,
        config_env_var("SLACK_REDIRECT_HOST")?,
    ));

    let command_events_config = Arc::new(SlackCommandEventsListenerConfig::new(
        config_env_var("SLACK_SIGNING_SECRET")?.into(),
    ));

    let listener_environment = Arc::new(
        SlackClientEventsListenerEnvironment::new(client.clone())
            .with_error_handler(error_handler)
            .with_user_state(UserStateExample(0)),
    );

    let make_svc = make_service_fn(move |_| {
        let thread_oauth_config = oauth_listener_config.clone();
        let thread_command_events_config = command_events_config.clone();
        let listener = SlackClientEventsHyperListener::new(listener_environment.clone());
        async move {
            let routes = chain_service_routes_fn(
                listener.oauth_service_fn(thread_oauth_config, oauth_install_handler),
                chain_service_routes_fn(
                    listener.command_events_service_fn(
                        thread_command_events_config,
                        command_events_handler,
                    ),
                    your_others_routes,
                ),
            );

            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(service_fn(routes))
        }
    });

    info!("Server is listening on {}", &addr);

    let server = hyper::server::Server::bind(&addr).serve(make_svc);
    server.await.map_err(|e| {
        error!("Server error: {}", e);
        e.into()
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter("events_api_server=debug,slack_morphism=debug")
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    server().await?;

    Ok(())
}
