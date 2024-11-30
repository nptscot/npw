export { default as HelpButton } from "./HelpButton.svelte";
export { default as QualitativeLegend } from "./QualitativeLegend.svelte";
import { constructMatchExpression } from "svelte-utils/map";
import { infraTypes } from "../stores";

export let colorByInfraType = constructMatchExpression(
  ["get", "infra_type"],
  Object.fromEntries(infraTypes.map((x) => [x[0], x[2]])),
  "red",
);
