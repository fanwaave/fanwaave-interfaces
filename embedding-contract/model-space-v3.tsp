/** Fanwaave embedding model-space contract v3. */
namespace Fanwaave.EmbeddingSpaceV3;

const maximumSourceDimensions = 4096;
const storageDimensions = 4100;
const vectorIndexMaximumDimensions = 2000;
const halfVectorIndexMaximumDimensions = 4000;

enum EmbeddingProvider { OpenAi: "openai", Google: "google", Qwen: "qwen", Nvidia: "nvidia", Baai: "baai", Custom: "custom" }
enum GenerationProvider { OpenAi: "openai", Anthropic: "anthropic", Google: "google", Qwen: "qwen", Nvidia: "nvidia", Baai: "baai", Custom: "custom" }
enum InputRole { Symmetric: "symmetric", Query: "query", Document: "document", Classification: "classification", Clustering: "clustering" }
enum DistanceMetric { Cosine: "cosine", InnerProduct: "inner-product", L2: "l2", HammingCandidate: "hamming-candidate" }
enum AnnStrategy { HalfVectorExact: "halfvec-exact", HalfVectorMrl: "halfvec-mrl", BinaryFullRerank: "binary-full-rerank", ExactOnly: "exact-only" }

model EmbeddingSpaceDescriptor {
  embeddingProvider: EmbeddingProvider;
  generationProvider?: GenerationProvider;
  @minLength(1) @maxLength(200) `model`: string;
  @minLength(1) @maxLength(200) modelRevision: string;
  @minValue(1) @maxValue(4096) sourceDimensions: int32;
  storageDimensions: 4100;
  @minLength(1) @maxLength(128) normalization: string;
  @minLength(1) @maxLength(128) taskProfile: string;
  inputRole: InputRole;
  @minLength(64) @maxLength(64) instructionSha256: string;
  @minLength(1) @maxLength(80) instructionFamily: string;
  @minLength(1) @maxLength(80) modality: string;
  distanceMetric: DistanceMetric;
  annStrategy: AnnStrategy;
  @minValue(0) @maxValue(4100) annDimensions: int32;
  @minLength(16) @maxLength(512) spaceFamilyV3: string;
}

/**
 * Cross-artifact invariants: missing embeddings are null; zero padding follows
 * the learned prefix; halfvec-exact never truncates; halfvec-mrl requires an
 * explicit MRL profile; fixed >4,000-dimensional models use full binary
 * candidates plus exact reranking or exact-only search; comparable rows require
 * equal spaceFamilyV3 and a compatible role pair.
 */
