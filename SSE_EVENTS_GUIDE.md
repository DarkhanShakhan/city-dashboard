# Server-Sent Events (SSE) Integration Guide

## Overview

The City Dashboard now supports **dual-mode control**:
1. **Keyboard controls** - Manual testing and fallback mode
2. **Server-Sent Events (SSE)** - Real-time event streaming from a server

Both modes work simultaneously. The system will log connection status and automatically reconnect if the server becomes unavailable.

---

## Configuration

### Server URL

Set the SSE server URL via environment variable:

```bash
export SSE_URL="http://localhost:3000/events"
cargo run
```

**Default:** `http://localhost:3000/events` if not specified

---

## Event Format

Events are sent as Server-Sent Events (SSE) with JSON payloads:

```
data: {"type": "barrier_broken", "team": "Red Team", "message": "Gate compromised"}

data: {"type": "led_display_broken", "team": "Blue Team"}
```

### Supported Events

#### 1. Barrier Gate Events

**Barrier Broken**
```json
{
  "type": "barrier_broken",
  "team": "Red Team",
  "message": "Physical breach at north gate"
}
```
- **Effect**: Opens barrier gate
- **Logged as**: `BARRIER BROKEN by Red Team - Physical breach at north gate`

**Barrier Repaired**
```json
{
  "type": "barrier_repaired",
  "team": "Blue Team"
}
```
- **Effect**: Closes barrier gate
- **Logged as**: `Barrier repaired by Blue Team`

---

#### 2. LED Display Events

**LED Display Broken**
```json
{
  "type": "led_display_broken",
  "team": "Red Team",
  "message": "Display hacked - showing fake warnings"
}
```
- **Effect**: Activates danger mode (red flashing "DANGER")
- **Logged as**: `LED DISPLAY BROKEN by Red Team - Display hacked - showing fake warnings`

**LED Display Repaired**
```json
{
  "type": "led_display_repaired"
}
```
- **Effect**: Restores normal LED display
- **Logged as**: `LED display repaired`

---

#### 3. SCADA System Events

**SCADA Compromised**
```json
{
  "type": "scada_compromised",
  "building_id": 5,
  "team": "Red Team",
  "message": "Building automation system hacked"
}
```
- **Effect**: Toggles SCADA state for all buildings (shows red flashing)
- **Logged as**: `SCADA COMPROMISED (Building 5) by Red Team - Building automation system hacked`
- `building_id` is optional (null = all buildings)

**SCADA Restored**
```json
{
  "type": "scada_restored",
  "building_id": null
}
```
- **Effect**: Resets all SCADA systems to working state
- **Logged as**: `SCADA systems restored`

---

#### 4. Emergency Stop Events

**Emergency Stop**
```json
{
  "type": "emergency_stop",
  "reason": "Security breach - all traffic halted"
}
```
- **Effect**: Forces all traffic lights to RED
- **Logged as**: `EMERGENCY STOP - Security breach - all traffic halted`

**Emergency Stop Deactivated**
```json
{
  "type": "emergency_stop_deactivated"
}
```
- **Effect**: Resumes normal traffic light operation
- **Logged as**: `Emergency stop deactivated`

---

#### 5. Danger Mode Events

**Danger Mode Activated**
```json
{
  "type": "danger_mode_activated",
  "reason": "Hazardous materials detected"
}
```
- **Effect**: Shows red flashing "DANGER" on LED display
- **Logged as**: `DANGER MODE - Hazardous materials detected`

**Danger Mode Deactivated**
```json
{
  "type": "danger_mode_deactivated"
}
```
- **Effect**: Returns LED display to normal
- **Logged as**: `Danger mode deactivated`

---

#### 6. Generic Log Messages

**Custom Log**
```json
{
  "type": "log_message",
  "level": "critical",
  "message": "Custom event from monitoring system"
}
```
- **Effect**: Adds message to log window
- **Levels**: `info`, `warning`, `error`, `critical` (all displayed as critical in current implementation)

---

## Simple Test Server (Python)

Save as `test_sse_server.py`:

```python
#!/usr/bin/env python3
"""
Simple SSE test server for City Dashboard
Sends random events every 3-5 seconds
"""

from flask import Flask, Response
import json
import time
import random

app = Flask(__name__)

TEAMS = ["Red Team", "Blue Team", "Green Team", "Alpha Squad"]

EVENTS = [
    {
        "type": "barrier_broken",
        "team": "Red Team",
        "message": "Gate compromised with battering ram"
    },
    {
        "type": "led_display_broken",
        "team": "Blue Team",
        "message": "Display hacked via network exploit"
    },
    {
        "type": "scada_compromised",
        "team": "Alpha Squad",
        "message": "Building automation hijacked"
    },
    {
        "type": "emergency_stop",
        "reason": "Security breach detected"
    },
    {
        "type": "barrier_repaired",
        "team": "Blue Team"
    },
    {
        "type": "scada_restored"
    }
]

@app.route('/events')
def stream():
    """SSE endpoint that streams random events"""
    def generate():
        # Send initial connection message
        yield f"data: {json.dumps({'type': 'log_message', 'level': 'info', 'message': 'Connected to test server'})}\n\n"

        while True:
            # Wait 3-5 seconds between events
            time.sleep(random.uniform(3, 5))

            # Pick a random event
            event = random.choice(EVENTS).copy()

            # Randomize team for team-based events
            if "team" in event and event["team"] != "Blue Team":
                event["team"] = random.choice(TEAMS)

            # Send event
            data = json.dumps(event)
            yield f"data: {data}\n\n"
            print(f"Sent: {data}")

    return Response(generate(), mimetype='text/event-stream')

@app.route('/')
def index():
    return """
    <h1>City Dashboard SSE Test Server</h1>
    <p>SSE endpoint: <a href="/events">/events</a></p>
    <p>Random events sent every 3-5 seconds</p>
    """

if __name__ == '__main__':
    print("Starting SSE test server on http://localhost:3000")
    print("Connect with: SSE_URL=http://localhost:3000/events cargo run")
    app.run(host='0.0.0.0', port=3000, threaded=True)
```

**Run the test server:**

```bash
# Install Flask
pip install flask

# Run server
python test_sse_server.py
```

---

## Simple Test Server (Node.js)

Save as `test_sse_server.js`:

```javascript
#!/usr/bin/env node
/**
 * Simple SSE test server for City Dashboard
 * Sends random events every 3-5 seconds
 */

const http = require('http');

const TEAMS = ["Red Team", "Blue Team", "Green Team", "Alpha Squad"];

const EVENTS = [
    {
        type: "barrier_broken",
        team: "Red Team",
        message: "Gate compromised with battering ram"
    },
    {
        type: "led_display_broken",
        team: "Blue Team",
        message: "Display hacked via network exploit"
    },
    {
        type: "scada_compromised",
        team: "Alpha Squad",
        message: "Building automation hijacked"
    },
    {
        type: "emergency_stop",
        reason: "Security breach detected"
    },
    {
        type: "barrier_repaired",
        team: "Blue Team"
    },
    {
        type: "scada_restored"
    }
];

const server = http.createServer((req, res) => {
    if (req.url === '/events') {
        // Set SSE headers
        res.writeHead(200, {
            'Content-Type': 'text/event-stream',
            'Cache-Control': 'no-cache',
            'Connection': 'keep-alive',
            'Access-Control-Allow-Origin': '*'
        });

        // Send initial message
        res.write(`data: ${JSON.stringify({
            type: 'log_message',
            level: 'info',
            message: 'Connected to test server'
        })}\n\n`);

        // Send random events every 3-5 seconds
        const interval = setInterval(() => {
            const event = {...EVENTS[Math.floor(Math.random() * EVENTS.length)]};

            // Randomize team
            if (event.team && event.team !== "Blue Team") {
                event.team = TEAMS[Math.floor(Math.random() * TEAMS.length)];
            }

            const data = JSON.stringify(event);
            res.write(`data: ${data}\n\n`);
            console.log(`Sent: ${data}`);
        }, 3000 + Math.random() * 2000);

        // Clean up on disconnect
        req.on('close', () => {
            clearInterval(interval);
            console.log('Client disconnected');
        });
    } else {
        res.writeHead(200, {'Content-Type': 'text/html'});
        res.end(`
            <h1>City Dashboard SSE Test Server</h1>
            <p>SSE endpoint: <a href="/events">/events</a></p>
            <p>Random events sent every 3-5 seconds</p>
        `);
    }
});

const PORT = 3000;
server.listen(PORT, () => {
    console.log(`SSE test server running on http://localhost:${PORT}`);
    console.log(`Connect with: SSE_URL=http://localhost:${PORT}/events cargo run`);
});
```

**Run the test server:**

```bash
node test_sse_server.js
```

---

## Testing the Integration

### 1. Start Test Server

```bash
# Python version
python test_sse_server.py

# OR Node.js version
node test_sse_server.js
```

### 2. Run Dashboard

```bash
cd frontend
SSE_URL=http://localhost:3000/events cargo run
```

### 3. Observe Logs

Press **'L'** to toggle the log window. You should see:
- `City Dashboard initialized`
- `SSE client connecting to: http://localhost:3000/events`
- `Server connected` (when connection succeeds)
- Event messages as they arrive from server

### 4. Test Both Control Modes

**Server events** will automatically trigger:
- Barrier opening/closing
- LED display mode changes
- SCADA system status
- Emergency stop activation

**Keyboard controls** still work for manual testing:
- **B** - Toggle barrier
- **Left Shift** - Toggle danger mode
- **S** - Toggle SCADA
- **Enter** - Emergency stop
- **Escape** - Reset all
- **L** - Toggle log window

---

## Connection Status Monitoring

The dashboard automatically:
1. **Connects** to SSE server on startup
2. **Logs** connection status:
   - `Server connected` - Successfully connected
   - `Server: Connection error: ...` - Failed to connect
   - `Server: Connection closed` - Server disconnected
3. **Reconnects** automatically every 5 seconds if connection fails
4. **Continues operation** using keyboard controls if server unavailable

---

## Production Server Requirements

Your production SSE server should:

1. **Send valid SSE format:**
   ```
   data: <JSON>\n\n
   ```

2. **Set proper headers:**
   ```
   Content-Type: text/event-stream
   Cache-Control: no-cache
   Connection: keep-alive
   ```

3. **Handle multiple clients** (one per dashboard instance)

4. **Implement proper error handling** for JSON serialization

5. **Optional: Send keep-alive comments** every 15-30 seconds:
   ```
   : keepalive\n\n
   ```

---

## Event Examples for CTF/Red vs Blue Scenarios

### Scenario: Barrier Breach

```json
{"type": "barrier_broken", "team": "Red Team", "message": "Exploited weak servo motor"}
```

### Scenario: Display Hijack

```json
{"type": "led_display_broken", "team": "Red Team", "message": "Injected fake evacuation message"}
```

### Scenario: Blue Team Defense

```json
{"type": "scada_restored"}
{"type": "barrier_repaired", "team": "Blue Team"}
{"type": "log_message", "level": "info", "message": "Security patches applied"}
```

### Scenario: Emergency Response

```json
{"type": "emergency_stop", "reason": "Multiple system compromises detected"}
{"type": "danger_mode_activated", "reason": "Unauthorized access in sector 3"}
```

---

## Troubleshooting

### Dashboard doesn't connect

1. Check server is running: `curl http://localhost:3000/events`
2. Verify SSE_URL environment variable
3. Check firewall/network settings
4. Look for connection errors in log window

### Events not appearing

1. Verify JSON format is correct
2. Check `data: ` prefix is present
3. Ensure double newline (`\n\n`) after each event
4. Validate JSON with: `echo '{"type":"barrier_broken","team":"Test"}' | json_pp`

### Server connection but no events

1. Events must be prefixed with `data: `
2. JSON must match event schemas exactly
3. Check server console for errors
4. Test with curl: `curl -N http://localhost:3000/events`

---

## Architecture Notes

- **Thread-based**: SSE client runs in background thread (compatible with macroquad)
- **Channel communication**: `std::sync::mpsc` for thread-safe event passing
- **Non-blocking**: Main game loop polls for events without blocking
- **Auto-reconnect**: 5-second retry interval on connection failure
- **No async runtime conflicts**: Uses `ureq` (blocking HTTP) instead of Tokio

This design ensures compatibility with macroquad's custom async runtime while maintaining responsive event handling.
