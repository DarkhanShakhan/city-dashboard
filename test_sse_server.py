#!/usr/bin/env python3
"""
Simple SSE test server for City Dashboard
Sends random events every 3-5 seconds for testing

Usage:
    pip install flask
    python test_sse_server.py

Then run the dashboard with:
    SSE_URL=http://localhost:3000/events cargo run
"""

from flask import Flask, Response
import json
import time
import random

app = Flask(__name__)

TEAMS = ["Red Team", "Blue Team", "Green Team", "Alpha Squad", "Omega Force"]

# Example events that will be randomly sent
EVENTS = [
    {
        "type": "barrier_broken",
        "team": "Red Team",
        "message": "Gate compromised with battering ram"
    },
    {
        "type": "barrier_broken",
        "team": "Alpha Squad",
        "message": "Barrier destroyed using explosives"
    },
    {
        "type": "led_display_broken",
        "team": "Red Team",
        "message": "Display hacked via network exploit"
    },
    {
        "type": "led_display_broken",
        "team": "Blue Team",
        "message": "Physical damage to LED matrix"
    },
    {
        "type": "scada_compromised",
        "team": "Alpha Squad",
        "message": "Building automation hijacked"
    },
    {
        "type": "scada_compromised",
        "building_id": 3,
        "team": "Red Team",
        "message": "SCADA protocol exploitation"
    },
    {
        "type": "emergency_stop",
        "reason": "Security breach detected in sector 5"
    },
    {
        "type": "emergency_stop",
        "reason": "Multiple simultaneous intrusions"
    },
    {
        "type": "danger_mode_activated",
        "reason": "Hazardous materials detected"
    },
    {
        "type": "barrier_repaired",
        "team": "Blue Team"
    },
    {
        "type": "led_display_repaired"
    },
    {
        "type": "scada_restored"
    },
    {
        "type": "emergency_stop_deactivated"
    },
    {
        "type": "danger_mode_deactivated"
    },
    {
        "type": "log_message",
        "level": "critical",
        "message": "Unauthorized access attempt on north perimeter"
    }
]

@app.route('/events')
def stream():
    """SSE endpoint that streams random events"""
    def generate():
        # Send initial connection message
        initial = {
            'type': 'log_message',
            'level': 'info',
            'message': 'Connected to test SSE server'
        }
        yield f"data: {json.dumps(initial)}\n\n"
        print(f"Client connected from {request.remote_addr}")

        event_count = 0
        while True:
            # Wait 3-5 seconds between events
            delay = random.uniform(3, 5)
            time.sleep(delay)

            # Pick a random event
            event = random.choice(EVENTS).copy()
            event_count += 1

            # Randomize team for team-based events
            if "team" in event and event["team"] != "Blue Team":
                event["team"] = random.choice(TEAMS)

            # Randomize building_id if present
            if "building_id" in event and event["building_id"] is not None:
                event["building_id"] = random.randint(1, 10)

            # Send event
            data = json.dumps(event)
            yield f"data: {data}\n\n"
            print(f"[{event_count}] Sent: {data}")

    from flask import request
    return Response(generate(), mimetype='text/event-stream')

@app.route('/')
def index():
    """Simple info page"""
    return """
    <!DOCTYPE html>
    <html>
    <head>
        <title>City Dashboard SSE Test Server</title>
        <style>
            body {
                font-family: monospace;
                background: #1e1e1e;
                color: #d4d4d4;
                padding: 20px;
            }
            h1 { color: #4ec9b0; }
            .endpoint { color: #ce9178; }
            .command {
                background: #2d2d30;
                padding: 10px;
                border-left: 3px solid #007acc;
                margin: 10px 0;
            }
            ul { line-height: 1.8; }
        </style>
    </head>
    <body>
        <h1>🏙️ City Dashboard SSE Test Server</h1>
        <p>SSE endpoint: <a class="endpoint" href="/events">/events</a></p>
        <p>Random events sent every 3-5 seconds</p>

        <h2>Usage:</h2>
        <div class="command">
            <code>SSE_URL=http://localhost:3000/events cargo run</code>
        </div>

        <h2>Available Events:</h2>
        <ul>
            <li>barrier_broken / barrier_repaired</li>
            <li>led_display_broken / led_display_repaired</li>
            <li>scada_compromised / scada_restored</li>
            <li>emergency_stop / emergency_stop_deactivated</li>
            <li>danger_mode_activated / danger_mode_deactivated</li>
            <li>log_message (custom messages)</li>
        </ul>

        <h2>Testing:</h2>
        <div class="command">
            <code>curl -N http://localhost:3000/events</code>
        </div>
    </body>
    </html>
    """

if __name__ == '__main__':
    import sys
    PORT = 3000

    print("=" * 60)
    print("  City Dashboard SSE Test Server")
    print("=" * 60)
    print(f"  Server: http://localhost:{PORT}")
    print(f"  Events: http://localhost:{PORT}/events")
    print("=" * 60)
    print("\nTo connect the dashboard:")
    print(f"  SSE_URL=http://localhost:{PORT}/events cargo run\n")
    print("Press Ctrl+C to stop\n")

    try:
        app.run(host='0.0.0.0', port=PORT, threaded=True, debug=False)
    except KeyboardInterrupt:
        print("\n\nServer stopped.")
        sys.exit(0)
