namespace Fanwaave.ContactDelivery;

/** Independent TypeSpec authority for Fanwaave outbound contact delivery. */
enum ContactJobStatus {
  queued,
  leased,
  retry,
  sent,
  dead,
  cancelled,
}

enum ContactChannel {
  email,
  sms,
}

enum ContactProvider {
  sendgrid,
  twilio,
}

enum DeliveryDisposition {
  sent,
  retryable,
  permanent_failure,
  suppressed,
}

model EmailPayload {
  to: string;
  subject: string;
  html: string;
  text?: string;
  from?: string;
  reply_to?: string;
  categories?: string[];
  custom_args?: Record<unknown>;
}

model SmsPayload {
  to: string;
  body: string;
  from?: string;
  status_callback?: string;
}

model EmailContact {
  channel: "email";
  payload: EmailPayload;
}

model SmsContact {
  channel: "sms";
  payload: SmsPayload;
}

union ContactRequest {
  email: EmailContact,
  sms: SmsContact,
}

model EnqueueContactJob {
  idempotency_key: string;
  campaign_id?: string;
  priority?: int32;
  max_attempts?: int32;
  metadata?: Record<unknown>;
  contact: ContactRequest;
}

model EnqueueContactReceipt {
  job_id: string;
  idempotency_key: string;
  status: ContactJobStatus;
  duplicate: boolean;
}

model ContactJobView {
  job_id: string;
  idempotency_key: string;
  channel: ContactChannel;
  provider: ContactProvider;
  status: ContactJobStatus;
  attempt_count: int32;
  max_attempts: int32;
  provider_message_id?: string;
  last_error?: string;
}

model DeliveryResult {
  disposition: DeliveryDisposition;
  provider: string;
  provider_message_id?: string;
  upstream_status?: int32;
  error?: string;
  rate_limited: boolean;
  latency_ms: int64;
}

model ContactResultEvent {
  job_id: string;
  idempotency_key: string;
  channel: ContactChannel;
  campaign_id?: string;
  attempt_count: int32;
  result: DeliveryResult;
  occurred_at: string;
}
