use std::net::SocketAddr;

use crate::{requests::Requester, types::InputFile};

/// Options related to setting up webhooks.
pub struct Options {
    /// Local address to listen to.
    pub address: SocketAddr,

    /// Public url that Telegram will send updates to.
    ///
    /// Note:
    /// - At the time of writing only ports 443, 80, 88 and 8443 [are
    ///   supported][set_webhook]
    /// - This url must be forwarded to the [address][addr] in order for webhook
    ///   to work
    /// - This url should be kept private, otherwise malicious actors can
    ///   pretend to be Telegram and send fake updates to your bot
    ///
    /// [set_webhook]: https://core.telegram.org/bots/api#setwebhook
    /// [addr]: (self::Options.address)
    pub url: url::Url,

    /// Upload your public key certificate so that the root certificate in use
    /// can be checked. See Telegram's [self-signed guide] for details.
    ///
    /// [self-signed guide]: https://core.telegram.org/bots/self-signed
    ///
    /// Default - None.
    pub certificate: Option<InputFile>,

    /// Pass `true` to drop all pending updates.
    ///
    /// Default - None.
    pub drop_pending_updates: Option<bool>,
}

impl Options {
    /// Construct a new webhook options, see [`Options.address`] and
    /// [`Options.url`] for details.
    pub fn new(address: SocketAddr, url: url::Url) -> Self {
        Self { address, url, certificate: None, drop_pending_updates: None }
    }

    /// Upload your public key certificate so that the root certificate in use
    /// can be checked. See Telegram's [self-signed guide] for details.
    ///
    /// [self-signed guide]: https://core.telegram.org/bots/self-signed
    pub fn certificate(self, v: InputFile) -> Self {
        Self { certificate: Some(v), ..self }
    }

    /// Drop all pending updates before setting up webhook.
    pub fn drop_pending_updates(self) -> Self {
        Self { drop_pending_updates: Some(true), ..self }
    }
}

#[cfg(feature = "webhooks-axum")]
pub use self::axum::{axum, axum_no_setup, axum_to_router};

#[cfg(feature = "webhooks-axum")]
mod axum;

async fn setup_webhook<R>(bot: R, options: &mut Options) -> Result<(), R::Err>
where
    R: Requester,
{
    use crate::requests::Request;
    use teloxide_core::requests::HasPayload;

    let &mut Options { ref url, ref mut certificate, drop_pending_updates, .. } = options;

    let mut req = bot.set_webhook(url.clone());
    req.payload_mut().certificate = certificate.take();
    req.payload_mut().drop_pending_updates = drop_pending_updates;

    req.send().await?;

    Ok(())
}

fn tuple_first_mut<A, B>(tuple: &mut (A, B)) -> &mut A {
    &mut tuple.0
}
