//! Server-Sent Events (SSE) client for receiving real-time events
//!
//! This module implements a simple SSE client that runs in a background thread
//! and sends parsed events to the main game loop via channels. It's compatible
//! with macroquad's custom async runtime by using blocking I/O in a separate thread.
//!
//! ## SSE Format
//! Server-Sent Events follow this format:
//! ```text
//! data: {"type": "barrier_broken", "team": "Red Team"}
//!
//! data: {"type": "led_display_broken", "team": "Blue Team"}
//! ```

use crate::events::{EventSender, GameEvent};
use std::io::BufRead;
use std::thread;
use std::time::Duration;

/// Configuration for SSE client
#[derive(Clone)]
pub struct SseConfig {
    /// Server URL endpoint (e.g., "http://localhost:3000/events")
    pub url: String,

    /// Reconnection interval in seconds when connection fails
    pub reconnect_interval: u64,

    /// Request timeout in seconds
    pub timeout: u64,
}

impl Default for SseConfig {
    fn default() -> Self {
        Self {
            url: "http://localhost:3000/events".to_string(),
            reconnect_interval: 5,
            timeout: 30,
        }
    }
}

/// SSE client that runs in a background thread
pub struct SseClient {
    config: SseConfig,
    sender: EventSender,
}

impl SseClient {
    /// Creates a new SSE client with given configuration
    ///
    /// # Arguments
    /// * `config` - SSE configuration including server URL
    /// * `sender` - Event sender to communicate with main game loop
    pub fn new(config: SseConfig, sender: EventSender) -> Self {
        Self { config, sender }
    }

    /// Starts the SSE client in a background thread
    ///
    /// This spawns a background thread that continuously tries to connect
    /// to the SSE endpoint, receive events, and send them to the main loop.
    ///
    /// # Returns
    /// JoinHandle for the background thread (can be used to stop it if needed)
    pub fn start(self) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            self.run_loop();
        })
    }

    /// Main loop that handles connection, reconnection, and event processing
    fn run_loop(&self) {
        loop {
            // Notify about connection attempt
            let _ = self.sender.send(GameEvent::ConnectionStatus {
                connected: false,
                error: Some("Connecting to server...".to_string()),
            });

            match self.connect_and_receive() {
                Ok(_) => {
                    // Connection closed normally
                    let _ = self.sender.send(GameEvent::ConnectionStatus {
                        connected: false,
                        error: Some("Connection closed".to_string()),
                    });
                }
                Err(e) => {
                    // Connection failed
                    let error_msg = format!("Connection error: {}", e);
                    let _ = self.sender.send(GameEvent::ConnectionStatus {
                        connected: false,
                        error: Some(error_msg),
                    });
                }
            }

            // Wait before reconnecting
            thread::sleep(Duration::from_secs(self.config.reconnect_interval));
        }
    }

    /// Connects to SSE endpoint and processes events
    fn connect_and_receive(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Create HTTP request with SSE headers
        let response = ureq::get(&self.config.url)
            .timeout(Duration::from_secs(self.config.timeout))
            .set("Accept", "text/event-stream")
            .set("Cache-Control", "no-cache")
            .call()?;

        // Check if connection successful
        if response.status() != 200 {
            return Err(format!("HTTP error: {}", response.status()).into());
        }

        // Notify successful connection
        let _ = self.sender.send(GameEvent::ConnectionStatus {
            connected: true,
            error: None,
        });

        // Read SSE stream line by line
        let reader = std::io::BufReader::new(response.into_reader());
        for line in reader.lines() {
            let line = line?;

            // SSE format: "data: <json>"
            if let Some(data) = line.strip_prefix("data: ") {
                if !data.trim().is_empty() {
                    self.parse_and_send_event(data);
                }
            }
            // Ignore comment lines (starting with :) and empty lines
        }

        Ok(())
    }

    /// Parses JSON event data and sends to main loop
    fn parse_and_send_event(&self, data: &str) {
        match serde_json::from_str::<GameEvent>(data) {
            Ok(event) => {
                if let Err(e) = self.sender.send(event) {
                    eprintln!("Failed to send event to main loop: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to parse SSE event: {} - Data: {}", e, data);
                // Send as generic log message instead
                let _ = self.sender.send(GameEvent::LogMessage {
                    level: crate::events::LogLevel::Error,
                    message: format!("Invalid event format: {}", data),
                });
            }
        }
    }
}

/// Convenience function to start SSE client with default configuration
///
/// # Arguments
/// * `url` - Server SSE endpoint URL
/// * `sender` - Event sender for communication with main loop
///
/// # Returns
/// JoinHandle for the background thread
///
/// # Example
/// ```
/// let (sender, receiver) = create_event_channel();
/// let handle = start_sse_client("http://localhost:3000/events", sender);
/// ```
pub fn start_sse_client(url: impl Into<String>, sender: EventSender) -> thread::JoinHandle<()> {
    let config = SseConfig {
        url: url.into(),
        ..Default::default()
    };

    let client = SseClient::new(config, sender);
    client.start()
}
