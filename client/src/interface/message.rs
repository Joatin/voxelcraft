#[derive(Debug, Clone)]
pub enum Message {
    Navigate { page: String },
    QuitApplication,
    CreateNewGame,
    GameLoadingMessage(String, Option<f32>),
    EscapePressed,
}
