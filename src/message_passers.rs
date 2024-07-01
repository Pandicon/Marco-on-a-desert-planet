use std::sync::mpsc;

use crate::data;

pub struct MessagePassers {
    pub main_to_calculator_sender: mpsc::Sender<Message>,
    pub calculator_to_main_sender: mpsc::Sender<Message>,
    pub calculator_to_main_receiver: mpsc::Receiver<Message>,
}

impl Default for MessagePassers {
    fn default() -> Self {
        let (main_to_calculator_sender, _) = mpsc::channel();
        let (calculator_to_main_sender, calculator_to_main_receiver) = mpsc::channel();
        Self {
            main_to_calculator_sender,

            calculator_to_main_sender,
            calculator_to_main_receiver,
        }
    }
}

pub enum Message {
    NewStage(CalculationStage),
    NewPoint(data::Data),
}

#[derive(PartialEq, Eq)]
pub enum CalculationStage {
    Start,
    Points,
    Plots,
    End,
}

impl AsRef<str> for CalculationStage {
    fn as_ref(&self) -> &str {
        match *self {
            Self::Start => "Just started calculation",
            Self::Points => "Generating points",
            Self::Plots => "Generating plots",
            Self::End => "Ended calculations",
        }
    }
}
