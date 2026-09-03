namespace Fanwaave.Validation;

@doc("Public request correlation metadata safe to exchange with clients.")
model RequestMeta {
  @minLength(1)
  @maxLength(128)
  requestId: string;

  @minLength(1)
  @maxLength(128)
  traceId: string;

  @minLength(2)
  @maxLength(64)
  locale?: string;
}

@doc("Bounded cursor pagination query shared by clients and servers.")
model PageQuery {
  @minValue(1)
  @maxValue(100)
  limit: uint16 = 50;

  @minLength(1)
  @maxLength(512)
  cursor?: string;
}

@doc("RFC 9457-style public problem response with request correlation.")
model ProblemDetails {
  @minLength(1)
  @maxLength(512)
  type: string;

  @minLength(1)
  @maxLength(256)
  title: string;

  @minValue(400)
  @maxValue(599)
  status: uint16;

  @maxLength(4096)
  detail?: string;

  @minLength(1)
  @maxLength(128)
  requestId: string;
}

alias PublicValidationContract = RequestMeta | PageQuery | ProblemDetails;
