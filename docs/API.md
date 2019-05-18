# API

## `/api/vorto/<vorton>`

Retrieve a specific word.

Example: `/api/vorto/en`

```json
{
    "bibliography": [
        "MT",
        "Far3",
        "PrV"
    ],
    "meanings": [
        {
            "definition": "<p>Prepozicio</p>\n",
            "examples": [ "Mi eniras en via domo." ],
            "usage":"FIG, FIG, FIG"
        },
    ],
    "related": ["al en","ena","ene","ene de"],
    "translations": {
        "ca":[ "a" ],
        "cs":[ "do", "na", "v", "ve" ],
        "de":[ "in", "im" ],
        "en":[ "into" ]
    },
    "word":"en"
}
```

(redacted for more clarity)

## `/api/sercxu?demando=vorto`

Search for a word.

Example: `/api/sercxu?demando=en`

```json
{
    "results": [ "neniu", "en" ]
}
```