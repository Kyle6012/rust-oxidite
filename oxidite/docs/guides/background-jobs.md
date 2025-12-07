# Background Jobs Guide

Learn how to process background jobs, schedule cron tasks, and handle async operations with Oxidite.

## Installation

```toml
[dependencies]
oxidite = { version = "1.0", features = ["queue"] }
```

## Quick Start

### Define a Job

```rust
use oxidite::queue::*;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct SendEmailJob {
    to: String,
    subject: String,
    body: String,
}

#[async_trait]
impl Job for SendEmailJob {
    async fn perform(&self) -> JobResult {
        // Your job logic
        println!("Sending email to: {}", self.to);
        
        // Simulate email sending
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        Ok(())
    }
    
    fn max_retries(&self) -> u32 {
        3  // Retry up to 3 times on failure
    }
    
    fn name(&self) -> String {
        "send_email".to_string()
    }
}
```

### Enqueue Jobs

```rust
use oxidite::prelude::*;
use oxidite::queue::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Create queue (Memory or Redis)
    let queue = Queue::memory();
    // let queue = Queue::redis("redis://127.0.0.1")?;
    
    // Enqueue a job
    let job = JobWrapper::new(&SendEmailJob {
        to: "user@example.com".to_string(),
        subject: "Welcome!".to_string(),
        body: "Thanks for signing up!".to_string(),
    })?;
    
    queue.enqueue(job).await?;
    
    Ok(())
}
```

### Start Workers

```rust
use std::sync::Arc;

// Start worker
let worker = Worker::new(Arc::new(queue))
    .worker_count(4);  // 4 concurrent workers

worker.start().await;
```

## Cron Jobs

Schedule recurring jobs with cron expressions:

```rust
#[derive(Serialize, Deserialize)]
struct DailyReportJob;

#[async_trait]
impl Job for DailyReportJob {
    async fn perform(&self) -> JobResult {
        println!("Generating daily report...");
        // Generate and send report
        Ok(())
    }
}

// Schedule to run daily at 9 AM
let job = JobWrapper::new(&DailyReportJob)?
    .with_cron("0 0 9 * * *".to_string());

queue.enqueue(job).await?;
```

### Cron Expression Format

```
┌───────────── second (0-59)
│ ┌───────────── minute (0-59)
│ │ ┌───────────── hour (0-23)
│ │ │ ┌───────────── day of month (1-31)
│ │ │ │ ┌───────────── month (1-12)
│ │ │ │ │ ┌───────────── day of week (0-6)
│ │ │ │ │ │
│ │ │ │ │ │
* * * * * *
```

Examples:
- `0 0 9 * * *` - Daily at 9:00 AM
- `0 */15 * * * *` - Every 15 minutes
- `0 0 0 * * 0` - Weekly on Sunday at midnight
- `0 0 12 1 * *` - Monthly on the 1st at noon

## Retry Logic

Jobs automatically retry on failure with exponential backoff:

```rust
impl Job for MyJob {
    async fn perform(&self) -> JobResult {
        // If this fails, job will retry
        risky_operation().await?;
        Ok(())
    }
    
    fn max_retries(&self) -> u32 {
        5  // Retry up to 5 times
    }
    
    fn backoff_duration(&self) -> Duration {
        Duration::from_secs(60)  // Base backoff: 60s, 120s, 240s, etc.
    }
}
```

## Dead Letter Queue

Jobs that fail after all retries go to the Dead Letter Queue:

```rust
// List failed jobs
let failed_jobs = queue.list_dead_letter().await?;

for job in failed_jobs {
    println!("Failed job: {} - Error: {:?}", job.id, job.error);
}

// Retry a specific job from DLQ
queue.retry_from_dead_letter(&job_id).await?;
```

## Job Statistics

Monitor queue health:

```rust
let  stats = queue.get_stats().await;

println!("Total enqueued: {}", stats.total_enqueued);
println!("Total processed: {}", stats.total_processed);
println!("Total failed: {}", stats.total_failed);
println!("Pending: {}", stats.pending_count);
println!("Running: {}", stats.running_count);
println!("Dead letter: {}", stats.dead_letter_count);
```

## Complete Example

```rust
use oxidite::prelude::*;
use oxidite::queue::*;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
struct ProcessImageJob {
    image_url: String,
    user_id: i64,
}

#[async_trait]
impl Job for ProcessImageJob {
    async fn perform(&self) -> JobResult {
        // Download image
        let image = download_image(&self.image_url).await?;
        
        // Process image
        let thumbnail = create_thumbnail(&image)?;
        
        // Save to storage
        save_to_storage(&thumbnail, self.user_id).await?;
        
        Ok(())
    }
    
    fn max_retries(&self) -> u32 {
        3
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Setup queue
    let queue = Arc::new(Queue::redis("redis://127.0.0.1")?);
    
    // Start workers
    tokio::spawn({
        let queue = queue.clone();
        async move {
            Worker::new(queue)
                .worker_count(4)
                .start()
                .await;
        }
    });
    
    // Your web app
    let mut app = Router::new();
    
    app.post("/upload", {
        let queue = queue.clone();
        move |Json(data): Json<UploadRequest>| {
            let queue = queue.clone();
            async move {
                // Enqueue background job
                let job = JobWrapper::new(&ProcessImageJob {
                    image_url: data.url,
                    user_id: data.user_id,
                })?;
                
                queue.enqueue(job).await?;
                
                Ok(Json(json!({ "status": "processing" })))
            }
        }
    });
    
    Server::new(app).listen("127.0.0.1:3000".parse()?).await
}
```

## CLI Commands

Manage jobs with the CLI:

```bash
# Start worker
oxidite queue:work --workers 4

# View statistics
oxidite queue:list

# View dead letter queue
oxidite queue:dlq

# Clear pending jobs
oxidite queue:clear
```
