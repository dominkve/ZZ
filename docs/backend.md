# Backend Notes

## **1. Queue System (NATS JetStream)**
### **Why NATS?**
- High-throughput, low-latency messaging.
- Supports multiple queues without excessive overhead.
- Handles multiple workers efficiently without job fanning.
- Persistent message storage via JetStream.

### **Implementation Plan**
#### **1.1 Set Up NATS JetStream**
- Install and run `nats-server` locally.
- Configure a `test_results` stream to store messages.

#### **1.2 Python Producer (Sending Messages)**
- Use `nats-py` to push test results to the queue.
- Example:
  ```python
  import nats
  import json
  
  async def send_message():
      nc = await nats.connect("nats://localhost:4222")
      result = {"test_id": 123, "status": "passed"}
      await nc.publish("test_results", json.dumps(result).encode())
      await nc.close()
  ```

#### **1.3 Rust Consumer (Processing Messages & Storing in DB)**
- Use `async-nats` in Rust to listen for messages.
- Store results in MongoDB.
- Example:
  ```rust
  use async_nats::connect;
  
  #[tokio::main]
  async fn main() -> Result<(), Box<dyn std::error::Error>> {
      let nc = connect("nats://localhost:4222").await?;
      let sub = nc.subscribe("test_results").await?;
      
      while let Some(msg) = sub.next().await {
          println!("Received: {:?}", std::str::from_utf8(&msg.data)?);
          // Store in MongoDB here
      }
      Ok(())
  }
  ```

#### **1.4 Handling Failures & Retries**
- Enable message persistence in JetStream.
- Configure retry logic to prevent message loss.

#### **1.5 Rust → Python Communication (Optional)**
- If Rust needs to notify Python, use another NATS queue.

