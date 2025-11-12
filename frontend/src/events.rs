//! Event system for handling server-sent events and game state changes
//!
//! This module defines event types that can be received from the server via SSE
//! or triggered locally via keyboard. Events are passed through channels from
//! the SSE background thread to the main game loop.

use serde::{Deserialize, Serialize};
use std::sync::mpsc;

/// Game events that can be triggered by server or keyboard
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum GameEvent {
    /// Barrier gate broken by a team
    BarrierBroken {
        team: String,
        message: Option<String>,
    },

    /// Barrier gate repaired/reset
    BarrierRepaired {
        team: Option<String>,
    },

    /// LED display broken or damaged
    LedDisplayBroken {
        team: String,
        message: Option<String>,
    },

    /// LED display repaired
    LedDisplayRepaired,

    /// SCADA system compromised
    ScadaCompromised {
        building_id: Option<usize>,
        team: String,
        message: Option<String>,
    },

    /// SCADA system restored
    ScadaRestored {
        building_id: Option<usize>,
    },

    /// Emergency traffic stop activated
    EmergencyStop {
        reason: String,
    },

    /// Emergency stop deactivated
    EmergencyStopDeactivated,

    /// Danger mode activated
    DangerModeActivated {
        reason: String,
    },

    /// Danger mode deactivated
    DangerModeDeactivated,

    /// Custom log message
    LogMessage {
        level: LogLevel,
        message: String,
    },

    /// Server connection status change
    ConnectionStatus {
        connected: bool,
        error: Option<String>,
    },
}

/// Log severity level
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Critical,
}

/// Event receiver that can be polled in the main game loop
pub struct EventReceiver {
    receiver: mpsc::Receiver<GameEvent>,
}

impl EventReceiver {
    /// Creates a new event receiver from a channel receiver
    pub fn new(receiver: mpsc::Receiver<GameEvent>) -> Self {
        Self { receiver }
    }

    /// Polls for new events without blocking
    ///
    /// Returns all available events in the queue. Should be called
    /// every frame in the main game loop.
    pub fn poll(&self) -> Vec<GameEvent> {
        let mut events = Vec::new();
        while let Ok(event) = self.receiver.try_recv() {
            events.push(event);
        }
        events
    }
}

/// Event sender for the SSE background thread
pub struct EventSender {
    sender: mpsc::Sender<GameEvent>,
}

impl EventSender {
    /// Creates a new event sender from a channel sender
    pub fn new(sender: mpsc::Sender<GameEvent>) -> Self {
        Self { sender }
    }

    /// Sends an event to the main game loop
    ///
    /// Returns Ok(()) if successful, Err if the receiver has been dropped
    pub fn send(&self, event: GameEvent) -> Result<(), mpsc::SendError<GameEvent>> {
        self.sender.send(event)
    }
}

/// Creates a new event channel for communicating between SSE thread and main loop
///
/// Returns a tuple of (EventSender, EventReceiver)
pub fn create_event_channel() -> (EventSender, EventReceiver) {
    let (sender, receiver) = mpsc::channel();
    (EventSender::new(sender), EventReceiver::new(receiver))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_channel() {
        let (sender, receiver) = create_event_channel();

        sender
            .send(GameEvent::BarrierBroken {
                team: "Red Team".to_string(),
                message: Some("Barrier destroyed!".to_string()),
            })
            .unwrap();

        let events = receiver.poll();
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_json_parsing() {
        let json = r#"{
            "type": "barrier_broken",
            "team": "Blue Team",
            "message": "Gate compromised"
        }"#;

        let event: GameEvent = serde_json::from_str(json).unwrap();
        match event {
            GameEvent::BarrierBroken { team, message } => {
                assert_eq!(team, "Blue Team");
                assert_eq!(message, Some("Gate compromised".to_string()));
            }
            _ => panic!("Wrong event type"),
        }
    }
}
