---
tags: [infrastructure, protocol-support]
aliases: ["Operational Manual", "System Health"]
---
# Monitoring & Operational Oversight

## 1. System Health
- **Saito Network**: Track node uptime, latency, and consensus participation via the standard dashboard.
- **UPI Gateway**: Monitor transaction success rates, UTR verification latency, and API error rates against NPCI endpoints.

## 2. Financial & Infrastructure
- **Cloud Infrastructure**: Access billing, quotas, and resource utilization in Google Cloud Console.
- **Settlement Audit**: Regularly reconcile UPI transaction logs against on-chain `RoutingProof` events to identify discrepancies.

## 3. Incident Response
- **Alerting**: Automated triggers for high-latency verification or consensus failure.
- **Troubleshooting Steps**:
    - Check node connectivity to NPCI endpoints.
    - Verify `RoutingProof` broadcast status on the Saito explorer.
    - Check for recent network forks if consensus stalls.
- **Recovery**: Refer to [[Documentation/System Architecture|System Architecture]] for protocol-specific failure recovery steps during network partitions.

---
## Related
- [[Documentation/System Architecture|System Architecture]]
