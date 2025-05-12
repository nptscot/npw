import { Hono } from "hono";
import { cors } from "hono/cors";

const app = new Hono();
app.use("/api/*", cors());

app.post("/api/problems", async (c) => {
  const {
    problemType,
    details,
    email,
    network,
    url,
    boundary,
    viewport,
    mode,
    currentStage,
    editsRoadStyle,
    backgroundLayer,
  } = await c.req.json();

  if (
    !url ||
    !boundary ||
    !viewport ||
    !mode ||
    !currentStage ||
    !editsRoadStyle ||
    !backgroundLayer
  ) {
    return c.text("Missing required automaticDetails field");
  }
  // Don't bother attempting more rigorous validations. See the README for the rationale.

  const { success } = await c.env.DB.prepare(
    `
    INSERT INTO problems (status, submissionTime, problemType, details, email, network, url, boundary, viewport, appMode, currentStage, editsRoadStyle, backgroundLayer) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
  `,
  )
    .bind(
      "new",
      Date.now(),
      problemType,
      details,
      email,
      network ? JSON.stringify(network) : null,
      url,
      boundary,
      viewport,
      mode,
      currentStage,
      editsRoadStyle,
      backgroundLayer,
    )
    .run();

  if (success) {
    c.status(201);
    return c.text("Problem reported");
  } else {
    c.status(500);
    return c.text("Something went wrong");
  }
});

export default app;
