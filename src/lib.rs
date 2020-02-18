use hyper::{client::HttpConnector, Client};
use hyper_rustls;
use std::cell::RefCell;
use std::rc::Rc;
use tokio_core::reactor::Core;

type HttpsConnector = hyper_rustls::HttpsConnector<HttpConnector>;

/// Struct used to make calls to the Github API.
pub struct Datadog {
    token: String,
    core: Rc<RefCell<Core>>,
    client: Rc<Client<HttpsConnector>>,
}

impl Clone for Datadog {
    fn clone(&self) -> Self {
        Self {
            token: self.token.clone(),
            core: Rc::clone(&self.core),
            client: Rc::clone(&self.client),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
