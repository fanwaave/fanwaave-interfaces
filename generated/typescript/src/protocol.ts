import { InterfaceError } from "./errors";
import type { NotificationDispatch } from "./types";

export function parseNotificationDispatch(
  id: string,
  revision: string,
  payload: Record<string, unknown>,
): NotificationDispatch {
  if (!id.trim()) {
    throw new InterfaceError("empty_id");
  }
  if (!revision.trim()) {
    throw new InterfaceError("empty_revision");
  }
  return { id, revision, payload };
}

