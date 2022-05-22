use mqtt::client::{Builder, Session};

fn main() {
    let client = Builder::default().client_id(client_id).build();
    let session = Session::new(client);

    let clean_session = false; // TODO: What does this mean?
    let keep_alive = 60; // TODO: Seconds??
    let auth = None;
    let last_will = None;
    session.connect(client_id, keep_alive, auth, last_will);
}
