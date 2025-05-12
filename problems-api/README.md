# Problems backend API

NPW includes a tool for users to report problems. Unlike the rest of the app
(where the data a user generates is only kept in browser local storage and
manually exported and shared as files by the user), this tool requires a
central backend and storage. This directory has a simple API running as a
Cloudflare worker that accepts a POST request and stores it in a Cloudflare D1
sqlite database.

## Local development

I followed [this
tutorial](https://developers.cloudflare.com/d1/tutorials/build-a-comments-api)
generally. To develop locally:

```
# Create the local DB
npx wrangler d1 execute npw-problems --file schema.sql

# Launch the worker locally
npm run dev
```

Change the endpoint in `ReportProblemModal.svelte`, then try submitting a
report. Then dump the results to a file, `npx wrangler d1 export npw-problems
--output=db.sql`, and open locally to explore.

## Production

For setting up or dumping the DB, just add `--remote` to the wrangler commands.
`npm run deploy` to deploy the worker.

For now, the plan is to manually and periodically export the prod DB and share
amongst maintainers. Later we can consider something nicer.

## Data validity

The backend doesn't attempt to rigorously validate the request. If an attacker
submits junk requests outside the web app, they will wind up in the sqlite DB.
The chances of spam are low given the expected user base, the [D1 per-row size
limits](https://developers.cloudflare.com/d1/platform/limits/) stop some
problems, and since it's a write-only API, there's little reason to focus an
attack here anyway. What this means is that the maintainers must understand the
data isn't trustworthy and not do anything like use the values in automated
scripts that might execute arbitrary code. The email address collected is also
unverified, but should be clear from context if it belongs to a legitimate
known user or not, if we wish to contact people about a problem.
