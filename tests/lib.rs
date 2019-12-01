#[cfg(test)]
mod tests {
    use app::client::connect;
    use app::network::server;

    #[test]
    fn it_works() {
        connect()
    }

    #[test]
    fn server_it_works() {
        server::connect()
    }
}
