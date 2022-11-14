pub trait TwitterClient {
    fn send_reply(reply_to: u64, message: String);
}
