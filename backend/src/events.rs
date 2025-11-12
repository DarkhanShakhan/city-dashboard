//! Event types for SSE communication
//!
//! These event types match the frontend event expectations defined in
//! frontend/src/events.rs

use serde::{Deserialize, Serialize};

/// Game events that can be triggered by API and sent via SSE
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum GameEvent {
    /// Barrier gate broken by a team
    BarrierBroken {
        team: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        message: Option<String>,
    },

    /// Barrier gate repaired/reset
    BarrierRepaired {
        #[serde(skip_serializing_if = "Option::is_none")]
        team: Option<String>,
    },

    /// LED display broken or damaged
    LedDisplayBroken {
        team: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        message: Option<String>,
    },

    /// LED display repaired
    LedDisplayRepaired,

    /// SCADA system compromised
    ScadaCompromised {
        #[serde(skip_serializing_if = "Option::is_none")]
        building_id: Option<usize>,
        team: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        message: Option<String>,
    },

    /// SCADA system restored
    ScadaRestored {
        #[serde(skip_serializing_if = "Option::is_none")]
        building_id: Option<usize>,
    },

    /// Emergency traffic stop activated
    EmergencyStop { reason: String },

    /// Emergency stop deactivated
    EmergencyStopDeactivated,

    /// Danger mode activated
    DangerModeActivated { reason: String },

    /// Danger mode deactivated
    DangerModeDeactivated,

    /// Custom log message
    LogMessage { level: LogLevel, message: String },

    /// Server connection status change
    ConnectionStatus {
        connected: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
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

/// Request body for triggering barrier broken event
#[derive(Debug, Deserialize)]
pub struct BarrierBrokenRequest {
    pub team: String,
    pub message: Option<String>,
}

/// Request body for triggering barrier repaired event
#[derive(Debug, Deserialize)]
pub struct BarrierRepairedRequest {
    pub team: Option<String>,
}

/// Request body for LED display events
#[derive(Debug, Deserialize)]
pub struct LedDisplayBrokenRequest {
    pub team: String,
    pub message: Option<String>,
}

/// Request body for SCADA events
#[derive(Debug, Deserialize)]
pub struct ScadaCompromisedRequest {
    pub building_id: Option<usize>,
    pub team: String,
    pub message: Option<String>,
}

/// Request body for SCADA restored
#[derive(Debug, Deserialize)]
pub struct ScadaRestoredRequest {
    pub building_id: Option<usize>,
}

/// Request body for emergency stop
#[derive(Debug, Deserialize)]
pub struct EmergencyStopRequest {
    pub reason: String,
}

/// Request body for danger mode
#[derive(Debug, Deserialize)]
pub struct DangerModeRequest {
    pub reason: String,
}

/// Request body for custom log message
#[derive(Debug, Deserialize)]
pub struct LogMessageRequest {
    pub level: LogLevel,
    pub message: String,
}
