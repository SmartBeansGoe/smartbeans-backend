# API Documentation

All routes except the ones listed under "auth" and "public" require an Authorization header containing a valid auth token (`Authorization: Bearer <auth_token>`). They return 400/Bad request if the header is missing or in the wrong format and 401/Unauthorized if the auth token is invalid or expired.
"user" is the user that corresponds to the auth token sent in the header. "course" is the course the user used to login (if he authenticated via Stud.IP/LTI) or the course the user selected on his first SmartApe login (if he used one of the auth_debug routes).

## public

### /version (GET)

Returns the short git commit hash as a string.

## auth

### /auth_token (POST)

Expects an LTI POST request, validates it, and, if successful, returns an auth token that is valid for 8 hours (the duration can be changed in .env). Returns 401/Unauthorized if the LTI validation fails.

Input:
```
{
    'lis_person_sourcedid': '...',
    'oauth_signature': '...'
}
```

Output:
```
{
    'auth_token': '...'
}
```

### /auth_cookie (POST)

Similar to `auth_token`, but instead of returning a JSON object, it stores the token in a cookie and redirects to the frontend. The frontend URL can be changed in `.env`.

### /auth_debug/\<name> (GET)

Returns an auth_token for an arbitrary username. Disabled when compiled for production.

Output:
```
{
    'auth_token': '...'
}
```

### /auth_debug/\<name>/\<key> (GET)

Similar to `/auth_debug/<name>`, but requires the key specified in `.env`. Also available when compiled for production.

## tasks

### GET /tasks?id=\<id\>&solved=\<true|false\>

Returns all tasks of the course of the user. You can use the query string for filtering. All query paramters are optional.

Example output:
```
[
    {
        name: "A1 Task 1",
        shortname: "A1",
        solved: true,
        task: "Description",
        taskid: 42
    },
    ...
]
```

Please note: There might be tasks without shortname. In this case, "null" is returned.

### GET /progress

Returns the taskids of all solved tasks.

Example output:
```
[1, 2, 3, 17, 42]
```

## misc

### /rand/\<min>/\<max> (GET)

Returns a random integer between <min> and <max>.

Output:
```
{
    'rand': '...'
}
```

### /username (GET)

Returns the name of the user.

Output:
```
{
    'username': '...'
}
```
