namespace Fanwaave.ContactRead;

/** Only read-only contact operation currently authorized by this contract. */
model ContactReadOperation {
  kind: "get_contact_job";
  job_id: string;
}

/**
 * Claims of an authenticated, expiring, job-specific internal read capability.
 * These fields alone are NOT authorization; the caller must verify a
 * trusted issuer's HS256 signature and JWT header kid before using claims.
 */
model ContactReadGrantClaims {
  iss: "fanwaave-contact-auth";
  aud: "fanwaave-contact-reader";
  sub: string;
  scope: "contact_job.read";
  job_id: string;
  iat: int64;
  nbf: int64;
  exp: int64;
  jti: string;
}
