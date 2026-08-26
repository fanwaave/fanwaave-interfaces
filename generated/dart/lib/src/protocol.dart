import 'errors.dart';
import 'models.dart';

const protocolVersion = '1';
const schemaRevision = 'fanwaave-0001';

NotificationDispatch parseNotificationDispatch(String id, String revision, Map<String, Object?> payload) {
  if (id.trim().isEmpty) {
    throw const InterfaceException('empty_id');
  }
  if (revision.trim().isEmpty) {
    throw const InterfaceException('empty_revision');
  }
  return NotificationDispatch(id: id, revision: revision, payload: payload);
}

