
#[derive(Debug)]
pub enum Activity {
    Exit,
    MailSend { to: String, message: String },
    Say { message: String },
}
