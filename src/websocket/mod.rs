//!
//! The Binance WebSocket adapter.
//!

pub mod error;

use std::sync::mpsc;
use std::thread;

use websocket::client::ClientBuilder;
use websocket::ws::dataframe::DataFrame;
use websocket::OwnedMessage;

use crate::data::event::depth::Depth;
use crate::data::event::trade::Trade;
use crate::data::event::Event;

use self::error::Error;

///
/// The Binance WebSocket client.
///
pub struct Client {}

impl Client {
    ///
    /// Subscribes to a `symbol`-dedicated trade and depth streams.
    ///
    pub fn subscribe(symbol: &str) -> Result<mpsc::Receiver<Event>, Error> {
        let (tx, rx) = mpsc::channel();

        {
            let address = format!(
                "wss://stream.binance.com:9443/ws/{}@trade",
                symbol.to_ascii_lowercase()
            );
            let mut client = ClientBuilder::new(&address)
                .expect(crate::panic::WEBSOCKET_ADDRESS_VALID)
                .connect_secure(None)
                .map_err(Error::Connection)?;

            let tx = tx.clone();
            thread::spawn(move || loop {
                let message = match client.recv_message() {
                    Ok(message) => {
                        if message.is_ping() {
                            log::debug!("Received ping");
                            match client.send_message(&OwnedMessage::Pong(b"pong frame".to_vec())) {
                                Ok(()) => log::debug!("Sent pong"),
                                Err(error) => log::warn!("Pong sending error: {}", error),
                            }
                            continue;
                        }

                        message.take_payload()
                    }
                    Err(error) => {
                        log::error!("Websocket error: {}", error);
                        return;
                    }
                };

                if message.is_empty() {
                    continue;
                }

                match serde_json::from_slice::<Trade>(&message) {
                    Ok(trade) => match tx.send(Event::Trade(trade)) {
                        Ok(()) => {}
                        Err(_) => break,
                    },
                    Err(error) => log::warn!("Parsing error: {} ({:?})", error, message),
                }
            });
        }

        {
            let address = format!(
                "wss://stream.binance.com:9443/ws/{}@depth@100ms",
                symbol.to_ascii_lowercase()
            );
            let mut client = ClientBuilder::new(&address)
                .expect(crate::panic::WEBSOCKET_ADDRESS_VALID)
                .connect_secure(None)
                .map_err(Error::Connection)?;

            thread::spawn(move || loop {
                let message = match client.recv_message() {
                    Ok(message) => {
                        if message.is_ping() {
                            log::debug!("Received ping");
                            match client.send_message(&OwnedMessage::Pong(b"pong frame".to_vec())) {
                                Ok(()) => log::debug!("Sent pong"),
                                Err(error) => log::warn!("Pong sending error: {}", error),
                            }
                            continue;
                        }

                        message.take_payload()
                    }
                    Err(error) => {
                        log::error!("Websocket error: {}", error);
                        return;
                    }
                };

                if message.is_empty() {
                    continue;
                }

                match serde_json::from_slice::<Depth>(&message) {
                    Ok(depth) => match tx.send(Event::Depth(depth)) {
                        Ok(()) => {}
                        Err(_) => break,
                    },
                    Err(error) => log::warn!("Parsing error: {} ({:?})", error, message),
                }
            });
        }

        Ok(rx)
    }
}
