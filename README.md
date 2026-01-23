# shortlink

A homegrown link shortener. Interfaces with a data provider like [cuisine](https://github.com/Cadecraft/cuisine)

## Environment
See `.env.example`. `DATA_URL` is a URL that is expected to return text in this JSON format:
```json
{
    "ex": "https://example.com",
    "ex2": "https://example.com"
}
```

## Setup
See the setup instructions for [cuisine](https://github.com/Cadecraft/cuisine), which are similar.

## API
GET `/`: a simple HTML webpage that displays an index of publicly accessible links (coming soon)
GET `/:short`: redirects to the longer URL pointed to by short
POST `/refresh`: refreshes the list of redirects given the URL provided by the environment
