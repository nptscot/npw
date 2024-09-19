import { constructMatchExpression } from "svelte-utils/map";
import { infraTypes } from "../stores";

export let colorByInraType = constructMatchExpression(
  ["get", "infra_type"],
  Object.fromEntries(infraTypes.map((x) => [x[0], x[2]])),
  "red",
);
