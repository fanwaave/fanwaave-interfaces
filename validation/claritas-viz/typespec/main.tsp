import "@typespec/json-schema";
using TypeSpec.JsonSchema;

namespace Fanwaave.Claritas;

@id("ClaritasProvider")
enum ClaritasProvider { claritas: "claritas-viz" }
@id("VisualizationPurpose")
enum VisualizationPurpose { trendChart: "trend-chart", activityPlot: "activity-plot", distribution: "distribution" }
@id("ClaritasCapability")
enum ClaritasCapability { lineSegments: "line-segments", rollingMean: "rolling-mean", histogramCounts: "histogram-counts" }
@id("VisualizationRuntime")
enum VisualizationRuntime { rust: "rust", typescript: "typescript", dart: "dart" }
@id("ClaritasVisualizationBinding")
model ClaritasVisualizationBinding {
  provider: ClaritasProvider;
  purposes: VisualizationPurpose[];
  capabilities: ClaritasCapability[];
  runtimes: VisualizationRuntime[];
  generatedWitnessOnly: boolean;
}
