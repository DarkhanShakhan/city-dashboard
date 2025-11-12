# City Dashboard Backend Server

Real-time event server for the City Dashboard application using Server-Sent Events (SSE).

## Features

- 📡 **SSE Streaming**: Real-time event broadcasting to multiple clients
- 🔌 **REST API**: Trigger events via HTTP POST requests
- 🌐 **CORS Enabled**: Accept requests from any origin
- 🔄 **Auto-broadcasting**: Events sent to API are automatically broadcast to all SSE clients
- 📊 **Multiple Event Types**: Barrier, LED, SCADA, Emergency, Danger mode, and custom logs

## Quick Start

```bash
# Start the server
cd backend
cargo run

# Server starts on http://localhost:3000
```

Then connect the dashboard:

```bash
cd frontend
SSE_URL=http://localhost:3000/events cargo run
```

## Architecture

### SSE Broadcasting

The server uses Tokio's broadcast channel to distribute events:

1. API endpoints receive HTTP POST requests
2. Events are sent to a broadcast channel
3. All connected SSE clients receive the event immediately
4. Frontend dashboard processes events and updates UI

### Connection Flow

```
Client 1 (Dashboard) ──┐
                       │
Client 2 (Dashboard) ──┼──> SSE /events ──> Broadcast Channel <── API /api/*
                       │
Client 3 (Dashboard) ──┘
```

## API Endpoints

### 📡 SSE Endpoint

**GET** `/events`

Server-Sent Events stream. Clients connect to receive real-time events.

```bash
curl -N http://localhost:3000/events
```

### 🚧 Barrier Events

**POST** `/api/barrier/break`

Trigger barrier broken event.

```bash
curl -X POST http://localhost:3000/api/barrier/break \
  -H "Content-Type: application/json" \
  -d '{"team": "Red Team", "message": "Gate destroyed with explosives"}'
```

**POST** `/api/barrier/repair`

Trigger barrier repaired event.

```bash
curl -X POST http://localhost:3000/api/barrier/repair \
  -H "Content-Type: application/json" \
  -d '{"team": "Blue Team"}'
```

### 💡 LED Display Events

**POST** `/api/led/break`

Trigger LED display broken event.

```bash
curl -X POST http://localhost:3000/api/led/break \
  -H "Content-Type: application/json" \
  -d '{"team": "Red Team", "message": "Display hacked via network"}'
```

**POST** `/api/led/repair`

Trigger LED display repaired event.

```bash
curl -X POST http://localhost:3000/api/led/repair
```

### 🏭 SCADA Events

**POST** `/api/scada/compromise`

Trigger SCADA system compromised event.

```bash
curl -X POST http://localhost:3000/api/scada/compromise \
  -H "Content-Type: application/json" \
  -d '{"team": "Red Team", "building_id": 5, "message": "Building automation hijacked"}'
```

**POST** `/api/scada/restore`

Trigger SCADA system restored event.

```bash
curl -X POST http://localhost:3000/api/scada/restore \
  -H "Content-Type: application/json" \
  -d '{"building_id": null}'
```

### 🚨 Emergency Stop Events

**POST** `/api/emergency/start`

Trigger emergency stop (all traffic lights red).

```bash
curl -X POST http://localhost:3000/api/emergency/start \
  -H "Content-Type: application/json" \
  -d '{"reason": "Security breach detected"}'
```

**POST** `/api/emergency/stop`

Deactivate emergency stop.

```bash
curl -X POST http://localhost:3000/api/emergency/stop
```

### ⚠️ Danger Mode Events

**POST** `/api/danger/activate`

Activate danger mode on LED display.

```bash
curl -X POST http://localhost:3000/api/danger/activate \
  -H "Content-Type: application/json" \
  -d '{"reason": "Hazardous materials detected"}'
```

**POST** `/api/danger/deactivate`

Deactivate danger mode.

```bash
curl -X POST http://localhost:3000/api/danger/deactivate
```

### 📝 Custom Log Messages

**POST** `/api/log`

Send custom log message.

```bash
curl -X POST http://localhost:3000/api/log \
  -H "Content-Type: application/json" \
  -d '{"level": "critical", "message": "Custom event from external system"}'
```

**Levels**: `info`, `warning`, `error`, `critical`

## Event Format

All events are sent as SSE with JSON payloads:

```
data: {"type": "barrier_broken", "team": "Red Team", "message": "Gate destroyed"}

data: {"type": "led_display_broken", "team": "Blue Team"}
```

## Testing

### 1. Start Server

```bash
cd backend
cargo run
```

### 2. Watch SSE Stream

In another terminal:

```bash
curl -N http://localhost:3000/events
```

### 3. Trigger Events

In a third terminal:

```bash
# Break barrier
curl -X POST http://localhost:3000/api/barrier/break \
  -H "Content-Type: application/json" \
  -d '{"team": "Red Team", "message": "Barrier compromised"}'

# You should see the event appear in the SSE stream!
```

### 4. Connect Dashboard

```bash
cd ../frontend
SSE_URL=http://localhost:3000/events cargo run
```

Press 'L' in the dashboard to see the log window with events.

## CTF/Red vs Blue Scenarios

### Red Team Attack Sequence

```bash
# 1. Compromise SCADA
curl -X POST http://localhost:3000/api/scada/compromise \
  -H "Content-Type: application/json" \
  -d '{"team": "Red Team", "message": "Backdoor planted"}'

# 2. Break barrier
curl -X POST http://localhost:3000/api/barrier/break \
  -H "Content-Type: application/json" \
  -d '{"team": "Red Team", "message": "Physical breach"}'

# 3. Hijack LED display
curl -X POST http://localhost:3000/api/led/break \
  -H "Content-Type: application/json" \
  -d '{"team": "Red Team", "message": "Fake evacuation message"}'

# 4. Trigger emergency
curl -X POST http://localhost:3000/api/emergency/start \
  -H "Content-Type: application/json" \
  -d '{"reason": "Red Team full control"}'
```

### Blue Team Defense

```bash
# 1. Restore SCADA
curl -X POST http://localhost:3000/api/scada/restore \
  -H "Content-Type: application/json" \
  -d '{"building_id": null}'

# 2. Repair barrier
curl -X POST http://localhost:3000/api/barrier/repair \
  -H "Content-Type: application/json" \
  -d '{"team": "Blue Team"}'

# 3. Fix LED display
curl -X POST http://localhost:3000/api/led/repair

# 4. Deactivate emergency
curl -X POST http://localhost:3000/api/emergency/stop
```

## Development

### Project Structure

```
backend/
├── Cargo.toml          # Dependencies
├── README.md           # This file
└── src/
    ├── main.rs         # Server implementation
    └── events.rs       # Event type definitions
```

### Dependencies

- **axum**: Web framework
- **tokio**: Async runtime
- **tokio-stream**: SSE streaming
- **serde/serde_json**: JSON serialization
- **tower-http**: CORS and middleware
- **tracing**: Logging

### Adding New Event Types

1. Add event variant to `GameEvent` enum in `src/events.rs`
2. Add corresponding request struct
3. Create API endpoint handler in `src/main.rs`
4. Add route in router configuration

### Logging

The server uses `tracing` for structured logging. All events are logged:

```
INFO backend: Event broadcast to 2 clients: BarrierBroken { team: "Red Team", message: Some("Gate destroyed") }
```

## Production Deployment

### Environment Variables

```bash
# Bind address (default: 0.0.0.0:3000)
export BIND_ADDR="0.0.0.0:8080"
```

### Systemd Service

Create `/etc/systemd/system/city-dashboard-backend.service`:

```ini
[Unit]
Description=City Dashboard SSE Backend
After=network.target

[Service]
Type=simple
User=dashboard
WorkingDirectory=/opt/city-dashboard/backend
ExecStart=/usr/local/bin/cargo run --release
Restart=always

[Install]
WantedBy=multi-user.target
```

### Reverse Proxy (nginx)

```nginx
location /events {
    proxy_pass http://localhost:3000/events;
    proxy_http_version 1.1;
    proxy_set_header Connection '';
    proxy_buffering off;
    proxy_cache off;
}

location /api/ {
    proxy_pass http://localhost:3000/api/;
}
```

## Troubleshooting

### No events received

- Check server is running: `curl http://localhost:3000/`
- Verify SSE connection: `curl -N http://localhost:3000/events`
- Check firewall settings
- Ensure dashboard SSE_URL is correct

### Events not appearing in dashboard

- Verify dashboard is connected (check logs for "Server connected")
- Check JSON format in API requests
- Look for errors in server logs
- Test with curl to isolate issue

### CORS errors

- Server has CORS enabled for all origins
- Check browser console for actual error
- Verify Content-Type header is set correctly

## License

Part of the City Dashboard project.
