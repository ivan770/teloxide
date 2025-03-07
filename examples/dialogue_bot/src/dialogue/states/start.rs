use crate::dialogue::{states::ReceiveFullNameState, Dialogue};
use teloxide::prelude::*;
use teloxide_macros::teloxide;

pub struct StartState;

#[teloxide(subtransition)]
async fn start(_state: StartState, cx: TransitionIn, _ans: String) -> TransitionOut<Dialogue> {
    cx.answer_str("Let's start! What's your full name?").await?;
    next(ReceiveFullNameState)
}
