# Cloud & Server Architecture

## Overview

Hardware Tool provides **flexible deployment options** from standalone desktop to full cloud infrastructure. Whether you're a solo designer, a small team, or an enterprise — the same architecture scales to meet your needs.

> **"One Hardware Tool That Does It All"** — Desktop, server, or cloud — your choice.

---

## Deployment Models

| Model | Description | Use Case |
|-------|-------------|----------|
| **Desktop** | Standalone application | Individual designers |
| **Team Server** | On-premise server | Small teams, air-gapped |
| **Private Cloud** | Self-hosted cloud | Enterprise, compliance |
| **Managed Cloud** | Hardware Tool Cloud | SaaS, no infrastructure |
| **Hybrid** | Desktop + cloud compute | Heavy simulation offload |

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                        Hardware Tool Cloud                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐             │
│  │   Web UI    │  │  Desktop    │  │   CLI       │             │
│  │  (Browser)  │  │   Client    │  │  Client     │             │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘             │
│         │                │                │                     │
│         └────────────────┼────────────────┘                     │
│                          ▼                                      │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                      API Gateway                            ││
│  │              (Authentication, Rate Limiting)                ││
│  └─────────────────────────────────────────────────────────────┘│
│                          │                                      │
│         ┌────────────────┼────────────────┐                     │
│         ▼                ▼                ▼                     │
│  ┌───────────┐    ┌───────────┐    ┌───────────┐               │
│  │  Project  │    │ Simulation│    │  Export   │               │
│  │  Service  │    │  Service  │    │  Service  │               │
│  └───────────┘    └───────────┘    └───────────┘               │
│         │                │                │                     │
│         └────────────────┼────────────────┘                     │
│                          ▼                                      │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                    Storage Layer                            ││
│  │         (Projects, Libraries, Results, Assets)              ││
│  └─────────────────────────────────────────────────────────────┘│
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Desktop Mode

### Standalone Operation

```rust
DesktopConfig {
    // All processing local
    compute: ComputeMode::Local,
    
    // Local storage
    storage: StorageConfig {
        projects_dir: "~/HardwareTool/Projects",
        libraries_dir: "~/HardwareTool/Libraries",
        cache_dir: "~/HardwareTool/Cache",
    },
    
    // Optional cloud sync
    cloud_sync: CloudSyncConfig {
        enabled: false,
        provider: None,
    },
    
    // Offline capable
    offline_mode: true,
}
```

---

## Team Server

### On-Premise Deployment

```yaml
# docker-compose.yml
version: '3.8'

services:
  hwt-server:
    image: hardwaretool/server:latest
    ports:
      - "8080:8080"
    volumes:
      - ./data:/data
      - ./projects:/projects
    environment:
      - HWT_LICENSE_KEY=${LICENSE_KEY}
      - HWT_ADMIN_EMAIL=admin@company.com
      
  hwt-db:
    image: postgres:15
    volumes:
      - ./db:/var/lib/postgresql/data
    environment:
      - POSTGRES_DB=hardwaretool
      - POSTGRES_USER=hwt
      - POSTGRES_PASSWORD=${DB_PASSWORD}
      
  hwt-compute:
    image: hardwaretool/compute:latest
    deploy:
      replicas: 4
    environment:
      - HWT_SERVER_URL=http://hwt-server:8080
```

### Server Configuration

```toml
# server.toml
[server]
host = "0.0.0.0"
port = 8080
workers = 8

[database]
url = "postgres://hwt:password@hwt-db:5432/hardwaretool"
pool_size = 20

[storage]
type = "filesystem"  # or "s3", "azure", "gcs"
path = "/projects"

[auth]
provider = "ldap"  # or "oauth", "saml", "local"
ldap_url = "ldap://ldap.company.com"
ldap_base_dn = "ou=users,dc=company,dc=com"

[compute]
max_concurrent_jobs = 16
job_timeout = 3600  # seconds
gpu_enabled = true
```

---

## Cloud Services

### Project Service

```rust
/// Cloud project management
ProjectService {
    // Project storage
    storage: CloudStorage {
        provider: StorageProvider::S3,
        bucket: "hwt-projects",
        encryption: Encryption::AES256,
    },
    
    // Versioning
    versioning: VersioningConfig {
        enabled: true,
        max_versions: 100,
        auto_snapshot: true,
        snapshot_interval: Duration::hours(1),
    },
    
    // Sharing
    sharing: SharingConfig {
        enabled: true,
        public_links: true,
        link_expiry: Duration::days(7),
    },
}
```

### Simulation Service

```rust
/// Cloud simulation compute
SimulationService {
    // Compute resources
    compute: ComputeConfig {
        instance_types: vec![
            InstanceType::CPU_4x16,    // 4 cores, 16GB RAM
            InstanceType::CPU_16x64,   // 16 cores, 64GB RAM
            InstanceType::GPU_V100,    // GPU accelerated
        ],
        auto_scaling: true,
        max_instances: 100,
    },
    
    // Job queue
    queue: QueueConfig {
        provider: QueueProvider::SQS,
        priority_levels: 3,
        max_wait_time: Duration::minutes(5),
    },
    
    // Results
    results: ResultsConfig {
        storage: StorageProvider::S3,
        retention: Duration::days(30),
        compression: true,
    },
}
```

### Export Service

```rust
/// Cloud export processing
ExportService {
    // Parallel export
    parallelism: 8,
    
    // Format support
    formats: vec![
        ExportFormat::Gerber,
        ExportFormat::GDSII,
        ExportFormat::ODB,
        ExportFormat::IPC2581,
    ],
    
    // Delivery
    delivery: DeliveryConfig {
        download_link: true,
        email_notification: true,
        webhook: true,
    },
}
```

---

## License Server

### License Types

| Type | Description | Features |
|------|-------------|----------|
| **Node-Locked** | Single machine | Offline capable |
| **Floating** | Pool of licenses | Shared across team |
| **Subscription** | Cloud-based | Always current |
| **Enterprise** | Unlimited | Custom terms |

### License Server Configuration

```toml
# license-server.toml
[server]
host = "0.0.0.0"
port = 5280

[license]
type = "floating"
total_seats = 25
checkout_timeout = 3600  # seconds
allow_borrow = true
max_borrow_days = 7

[features]
# Feature-based licensing
simulation = { seats = 10 }
ai_assistant = { seats = 5 }
advanced_packaging = { seats = 3 }

[logging]
usage_tracking = true
audit_log = "/var/log/hwt-license.log"
```

### License Check UI

```
┌─────────────────────────────────────────────────────────────────┐
│ License Status                                           [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ License Type: Floating (Enterprise)                            │
│ Server: license.company.com:5280                               │
│ Status: ● Connected                                            │
│                                                                 │
│ Seat Usage:                                                    │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Feature              │ In Use │ Total │ Available          │ │
│ │ ─────────────────────┼────────┼───────┼─────────────────── │ │
│ │ Hardware Tool Core   │ 18     │ 25    │ 7                  │ │
│ │ Simulation           │ 6      │ 10    │ 4                  │ │
│ │ AI Assistant         │ 3      │ 5     │ 2                  │ │
│ │ Advanced Packaging   │ 1      │ 3     │ 2                  │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Your Checkout:                                                 │
│ • Hardware Tool Core (checked out 2h ago)                      │
│ • Simulation (checked out 30m ago)                             │
│                                                                 │
│ [Release Simulation] [Borrow License] [Refresh]                │
└─────────────────────────────────────────────────────────────────┘
```

---

## Team & Organization Management

### Organization Structure

```
┌─────────────────────────────────────────────────────────────────┐
│ Organization: Acme Electronics                           [✕]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Teams:                                                         │
│ ├─ 📁 Hardware Team (12 members)                               │
│ │   ├─ 👤 John Smith (Admin)                                   │
│ │   ├─ 👤 Jane Doe (Designer)                                  │
│ │   └─ ...                                                     │
│ ├─ 📁 Firmware Team (8 members)                                │
│ └─ 📁 QA Team (4 members)                                      │
│                                                                 │
│ Projects:                                                      │
│ ├─ 📦 Smart Sensor v2 (Hardware Team)                          │
│ ├─ 📦 Power Board v3 (Hardware Team)                           │
│ └─ 📦 Test Fixture (QA Team)                                   │
│                                                                 │
│ Permissions:                                                   │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Role        │ View │ Edit │ Export │ Admin │ Billing       │ │
│ │ ────────────┼──────┼──────┼────────┼───────┼────────────── │ │
│ │ Owner       │ ✓    │ ✓    │ ✓      │ ✓     │ ✓             │ │
│ │ Admin       │ ✓    │ ✓    │ ✓      │ ✓     │ ✗             │ │
│ │ Designer    │ ✓    │ ✓    │ ✓      │ ✗     │ ✗             │ │
│ │ Viewer      │ ✓    │ ✗    │ ✗      │ ✗     │ ✗             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Invite Member] [Create Team] [Settings]                       │
└─────────────────────────────────────────────────────────────────┘
```

---

## Compute Offload

### Hybrid Mode

```rust
/// Offload heavy computation to cloud
ComputeOffload {
    // Local vs cloud decision
    offload_rules: vec![
        OffloadRule {
            task: TaskType::Simulation,
            condition: Condition::EstimatedTime(Duration::minutes(5)),
            action: Action::OffloadToCloud,
        },
        OffloadRule {
            task: TaskType::DRC,
            condition: Condition::ComponentCount(1000),
            action: Action::OffloadToCloud,
        },
        OffloadRule {
            task: TaskType::Export,
            condition: Condition::Always,
            action: Action::RunLocal,
        },
    ],
    
    // Cloud compute config
    cloud: CloudComputeConfig {
        provider: CloudProvider::AWS,
        region: "us-west-2",
        instance_type: "c6i.4xlarge",
        spot_instances: true,
    },
}
```

---

## API Access

### REST API

```bash
# Authentication
curl -X POST https://api.hardwaretool.dev/v1/auth/token \
  -d '{"api_key": "your_api_key"}'

# List projects
curl -H "Authorization: Bearer $TOKEN" \
  https://api.hardwaretool.dev/v1/projects

# Run DRC
curl -X POST -H "Authorization: Bearer $TOKEN" \
  https://api.hardwaretool.dev/v1/projects/123/drc

# Export Gerber
curl -X POST -H "Authorization: Bearer $TOKEN" \
  https://api.hardwaretool.dev/v1/projects/123/export \
  -d '{"format": "gerber"}'
```

### WebSocket API

```javascript
// Real-time updates
const ws = new WebSocket('wss://api.hardwaretool.dev/v1/ws');

ws.onopen = () => {
  ws.send(JSON.stringify({
    type: 'subscribe',
    project_id: '123',
    events: ['drc', 'simulation', 'export']
  }));
};

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('Event:', data.type, data.payload);
};
```

---

## CLI Commands

```bash
# Server management
hwt server start
hwt server stop
hwt server status
hwt server logs

# Cloud configuration
hwt cloud login
hwt cloud logout
hwt cloud status
hwt cloud sync

# License management
hwt license status
hwt license checkout simulation
hwt license release simulation
hwt license borrow --days 3

# Organization
hwt org list
hwt org switch acme-electronics
hwt org members list
hwt org invite user@example.com --role designer
```

---

## Security

### Data Protection

```rust
SecurityConfig {
    // Encryption
    encryption: EncryptionConfig {
        at_rest: Encryption::AES256,
        in_transit: Encryption::TLS13,
        key_management: KeyManagement::AWS_KMS,
    },
    
    // Access control
    access_control: AccessControlConfig {
        model: AccessModel::RBAC,
        mfa_required: true,
        session_timeout: Duration::hours(8),
    },
    
    // Audit
    audit: AuditConfig {
        enabled: true,
        log_all_access: true,
        retention: Duration::days(365),
    },
    
    // Compliance
    compliance: vec![
        Compliance::SOC2,
        Compliance::GDPR,
        Compliance::ITAR,  // For defense projects
    ],
}
```

---

## Related Topics

- [Real-Time Collaboration](./realtime-collaboration.md)
- [Shared Project Architecture](./shared-project-architecture.md)
- [Plugin Architecture](./plugin-extension-architecture.md)
- [Command-Line Interface](./cli.md)
