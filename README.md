# shortlink

A homegrown self-hosted link shortener. Interfaces with a data provider like [cuisine](https://github.com/Cadecraft/cuisine)

## Motivations
I just want a lightweight link shortener. The goal is to use as few resources and be as simple as possible

## Environment & Setup
See the setup instructions for [cuisine](https://github.com/Cadecraft/cuisine), which are similar.

In `.env.example`, `DATA_URL` is a URL that is expected to return text in a JSON format like this one:
```json
{
    "ex": "https://example.com",
    "2": "https://example.com/two"
}
```

## API
- GET `/`: a simple HTML webpage that displays an index of publicly accessible links (coming soon)
- GET `/:short`: redirects to the longer URL pointed to by the shortened name
    - E.g. in the above example, `/ex` responds with `301 Moved Permanently` to the location `https://example.com`
- POST `/refresh`: refreshes the list of redirects given the URL provided by the environment
