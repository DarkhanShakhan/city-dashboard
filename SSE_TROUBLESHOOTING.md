# SSE Connection Troubleshooting Guide

## Common Connection Issues

### Issue: "Connection error: timed out"

**Symptoms:**
- Dashboard shows "Server: Connection error: timed out" in logs
- Connection works initially but breaks after some time
- Happens more frequently when no events are being sent

**Root Cause:**
SSE connections are long-lived HTTP connections that can timeout if:
1. No data is received for extended periods
2. Network intermediaries (proxies, load balancers) close idle connections
3. Client timeout is too aggressive

**Solutions Applied:**

#### Server-Side (Fixed ✅)
- **Keep-alive heartbeat**: Server now sends `:keepalive` comments every 15 seconds
- This ensures the connection always has data flowing, preventing timeouts
- Location: `backend/src/main.rs` line 93-99

```rust
Sse::new(event_stream).keep_alive(
    KeepAlive::new()
        .interval(std::time::Duration::from_secs(15))
        .text("keepalive")
)
```

#### Client-Side (Fixed ✅)
- **Increased timeout**: Changed from 30 seconds to 300 seconds (5 minutes)
- This gives the connection more time between keep-alive messages
- Auto-reconnect every 5 seconds if connection fails
- Location: `frontend/src/sse_client.rs` line 38

```rust
timeout: 300, // 5 minutes - generous timeout for long-lived SSE connections
```

### How Keep-Alive Works

```
Time: 0s  -> Client connects to /events
Time: 0s  -> Server sends: data: {"type":"connection_status",...}
Time: 15s -> Server sends: :keepalive
Time: 30s -> Server sends: :keepalive
Time: 45s -> Server sends: :keepalive
...
```

The `:keepalive` comments are SSE comments (ignored by client) that keep the TCP connection active.

---

## Other Common Issues

### Connection Refused

**Symptoms:**
- "Connection error: Connection refused"
- "Failed to connect to server"

**Solutions:**
1. Verify backend is running:
   ```bash
   curl http://localhost:3000/
   ```

2. Check the URL is correct:
   ```bash
   echo $SSE_URL  # Should be http://localhost:3000/events
   ```

3. Check firewall/port:
   ```bash
   sudo netstat -tulpn | grep 3000
   ```

### No Events Appearing

**Symptoms:**
- Connection successful ("Server connected" in logs)
- But no events appear when triggering via API

**Solutions:**
1. Check server logs for broadcast messages:
   ```
   INFO backend: Event broadcast to 1 clients: BarrierBroken { ... }
   ```

2. Test SSE stream directly:
   ```bash
   curl -N http://localhost:3000/events
   ```
   Then trigger event in another terminal and watch for output

3. Verify JSON format in API requests:
   ```bash
   # Good
   curl -X POST http://localhost:3000/api/barrier/break \
     -H "Content-Type: application/json" \
     -d '{"team": "Red Team", "message": "Test"}'

   # Bad (missing Content-Type header)
   curl -X POST http://localhost:3000/api/barrier/break \
     -d '{"team": "Red Team"}'
   ```

### Frequent Reconnections

**Symptoms:**
- Log shows repeated "Server: Connection closed" messages
- Dashboard reconnects every few seconds

**Possible Causes:**
1. **Proxy/Load Balancer timeout**: Some proxies close idle SSE connections
   - **Solution**: Configure proxy to allow long-lived connections
   - Nginx example:
     ```nginx
     proxy_read_timeout 300s;
     proxy_send_timeout 300s;
     ```

2. **Network instability**: WiFi/network dropping packets
   - **Solution**: Use wired connection or check network quality

3. **Server restarts**: Backend is crashing or being restarted
   - **Solution**: Check backend logs for errors

---

## Testing Connection Stability

### 1. Long-Running Test

Start the server and dashboard, let them run for 5+ minutes with no activity:

```bash
# Terminal 1
cd backend && cargo run

# Terminal 2
cd frontend && SSE_URL=http://localhost:3000/events cargo run
```

**Expected:** No timeout errors. Keep-alive should maintain connection.

### 2. Event Flow Test

Trigger events and verify they appear:

```bash
# Send event every 30 seconds for 5 minutes
for i in {1..10}; do
  curl -X POST http://localhost:3000/api/log \
    -H "Content-Type: application/json" \
    -d "{\"level\":\"info\",\"message\":\"Test $i\"}"
  sleep 30
done
```

**Expected:** All 10 events appear in dashboard logs.

### 3. Network Interruption Test

1. Start server and connect dashboard
2. Temporarily disable network or kill server
3. Re-enable network or restart server

**Expected:** Dashboard shows "Connection error", then reconnects automatically within 5 seconds.

---

## Configuration Options

### Adjusting Keep-Alive Interval

If you're behind an aggressive proxy, reduce the interval:

```rust
// backend/src/main.rs
KeepAlive::new()
    .interval(std::time::Duration::from_secs(10))  // More frequent
    .text("keepalive")
```

### Adjusting Client Timeout

For very unstable networks:

```rust
// frontend/src/sse_client.rs
timeout: 600, // 10 minutes instead of 5
```

### Adjusting Reconnection Interval

For faster reconnection attempts:

```rust
// frontend/src/sse_client.rs
reconnect_interval: 2, // 2 seconds instead of 5
```

---

## Monitoring

### Server-Side Logs

Look for these messages:

```
INFO backend: New SSE client connected
INFO backend: Event broadcast to 2 clients: ...
```

Count of clients should match number of connected dashboards.

### Client-Side Logs

Press 'L' in dashboard to see logs:

```
Server connected                    <- Good
Server: Connection closed           <- Connection dropped
Server: Connection error: ...       <- Error occurred
```

### Network Monitoring

Watch SSE traffic:

```bash
# Monitor server connections
sudo netstat -an | grep 3000

# Capture SSE traffic
sudo tcpdump -i any port 3000 -A
```

---

## Best Practices

1. **Always run backend on stable network**: SSE requires persistent connections
2. **Use wired connection for production**: WiFi can cause intermittent drops
3. **Configure reverse proxies properly**: Ensure they support SSE (no buffering, long timeouts)
4. **Monitor connection count**: Should match expected number of dashboards
5. **Test timeout scenarios**: Verify auto-reconnect works as expected

---

## Environment Variables

```bash
# Client configuration
export SSE_URL="http://your-server:3000/events"

# Future: Server configuration (not yet implemented)
export SSE_KEEPALIVE_INTERVAL="15"  # seconds
export BIND_ADDR="0.0.0.0:3000"
```

---

## Production Deployment Notes

### Load Balancer Configuration

For AWS ALB / nginx / other load balancers:

- **Idle timeout**: Set to > keep-alive interval (e.g., 60 seconds)
- **Connection timeout**: Set to > client timeout (e.g., 360 seconds)
- **Enable keep-alive**: Ensure proxy doesn't buffer SSE responses

### Health Checks

Don't use `/events` for health checks (it's a streaming endpoint). Use `/`:

```bash
# Good health check
curl http://localhost:3000/

# Bad health check (will keep connection open)
curl http://localhost:3000/events
```

### Scaling

Each dashboard connection consumes:
- 1 broadcast channel subscription
- 1 thread (blocked on reading stream)
- Minimal memory (~KB per connection)

Expected capacity: 1000+ concurrent connections per server instance.

---

## Quick Fixes Checklist

- [ ] Backend is running and accessible
- [ ] SSE_URL environment variable is correct
- [ ] Keep-alive is configured (15 second interval)
- [ ] Client timeout is generous (300 seconds)
- [ ] No proxy buffering SSE responses
- [ ] Network is stable
- [ ] Firewall allows port 3000
- [ ] CORS is enabled on server
- [ ] Events are being broadcast (check server logs)

---

## Getting Help

If issues persist:

1. Check server logs for errors
2. Test with curl first (eliminates client issues)
3. Monitor network with tcpdump/wireshark
4. Check for proxy/firewall interference
5. Verify system resources (CPU, memory, file descriptors)

Most SSE issues are network-related, not code bugs!
