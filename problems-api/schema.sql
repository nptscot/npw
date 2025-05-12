DROP TABLE IF EXISTS problems;

CREATE TABLE IF NOT EXISTS problems (
  id integer PRIMARY KEY AUTOINCREMENT,
  -- new, spam, resolved
  status text NOT NULL,
  -- Unix timestamp
  submissionTime integer NOT NULL,
  -- Fields the user enters
  problemType text,
  details text,
  email text,
  network text,
  -- automaticDetails
  url text NOT NULL,
  boundary text NOT NULL,
  viewport text NOT NULL,
  appMode text NOT NULL,
  currentStage text NOT NULL,
  editsRoadStyle text NOT NULL,
  backgroundLayer text NOT NULL
);
