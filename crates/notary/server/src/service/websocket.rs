use tokio::time::Instant;
use tracing::{debug, error, info};
use ws_stream_tungstenite::WsStream;

use crate::{
    service::{axum_websocket::WebSocket, notary_service},
    types::NotaryGlobals,
};

/// Perform notarization using the established websocket connection
pub async fn websocket_notarize(
    socket: WebSocket,
    notary_globals: NotaryGlobals,
    session_id: String,
) {
    let start = Instant::now();
    debug!(?session_id, "Upgraded to websocket connection");
    // Wrap the websocket in WsStream so that we have AsyncRead and AsyncWrite
    // implemented
    let stream = WsStream::new(socket.into_inner());
    match notary_service(stream, notary_globals, &session_id).await {
        Ok(_) => {
            info!(
                ?session_id,
                elapsed_time_millis = start.elapsed().as_millis(),
                "Successful notarization using websocket!"
            );
        }
        Err(err) => {
            error!(
                ?session_id,
                elapsed_time_millis = start.elapsed().as_millis(),
                "Failed notarization using websocket: {err}"
            );
        }
    }
}
