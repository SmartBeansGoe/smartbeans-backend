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

Returns an auth_token for an arbitrary username. Returns 404 if the user is not in the SmartApe databse. This route is disabled when the backend is compiled for production.

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

Please note: There might be tasks without shortname.

### GET /progress

Returns the taskids of all solved tasks.

Example output:
```
[1, 2, 3, 17, 42]
```

### POST /submit/\<taskid\>

Submits a solution for a task.

Input (`Content-Type` header has to be `text/plain`):
```
<content of the submitted file>
```

Output:
```
Currently only an empty string, but we could change that, if more information is needed.
```

### GET /submissions/\<taskid\>

Returns all submitted solutions for a task.

Example output:
```
[
    {
        "result": {
            "compileResult": {
                "cmd": "isolate -b 79 -t 5 --fsize=1000000 --mem=100000 --processes=10 --stderr-to-stdout --dir=/usr/bin --run -- /usr/bin/clang /box/main.c",
                "code": 0,
                "failed": false,
                "killed": false,
                "signal": null,
                "stderr": "OK (0.278 sec real, 0.307 sec wall)",
                "stdout": "/box/main.c:3:2: warning: implicitly declaring library function 'printf' with type 'int (const char *, ...)' [-Wimplicit-function-declaration]\n        printf(\"HelloWorld\\n\");\n        ^\n/box/main.c:3:2: note: include the header <stdio.h> or explicitly provide a declaration for 'printf'\n1 warning generated.",
                "timedOut": false
            },
            "feedback": [
                {
                    "reason": "WRONG_ANSWER",
                    "score": 0,
                    "testCase": {
                        "args": [],
                        "stdin": "",
                        "stdout": "Hallo Welt"
                    },
                    "testResult": {
                        "code": 0,
                        "stderr": "OK (0.005 sec real, 0.051 sec wall)",
                        "stdout": "HelloWorld"
                    }
                }
            ],
            "score": 0,
            "testtype": "simple",
            "type": "WRONG_ANSWER"
        },
        "sourceCode": "int main(int argc, char const *argv[])\n{\n\tprintf(\"HelloWorld\\n\");\n\treturn 0;\n}",
        "timestamp": 1607615806262
    },
    {
        "result": {
            "compileResult": {
                "cmd": "isolate -b 86 -t 5 --fsize=1000000 --mem=100000 --processes=10 --stderr-to-stdout --dir=/usr/bin --run -- /usr/bin/clang /box/main.c",
                "code": 0,
                "failed": false,
                "killed": false,
                "signal": null,
                "stderr": "OK (0.273 sec real, 0.317 sec wall)",
                "stdout": "/box/main.c:3:2: warning: implicitly declaring library function 'printf' with type 'int (const char *, ...)' [-Wimplicit-function-declaration]\n        printf(\"Hallo Welt\\n\");\n        ^\n/box/main.c:3:2: note: include the header <stdio.h> or explicitly provide a declaration for 'printf'\n1 warning generated.",
                "timedOut": false
            },
            "feedback": [
                {
                    "reason": "SUCCESS",
                    "score": 1,
                    "testCase": {
                        "args": [],
                        "stdin": "",
                        "stdout": "Hallo Welt"
                    },
                    "testResult": {
                        "cmd": "isolate -b 79 -t 5 --fsize=1000000 --mem=100000 --processes=10 --stderr-to-stdout --dir=/data=/submissions/ge3bgpa0b --run -- /data/a.out",
                        "code": 0,
                        "failed": false,
                        "killed": false,
                        "signal": null,
                        "stderr": "OK (0.005 sec real, 0.046 sec wall)",
                        "stdout": "Hallo Welt",
                        "timedOut": false
                    }
                }
            ],
            "score": 1,
            "testtype": "simple",
            "type": "SUCCESS"
        },
        "sourceCode": "int main(int argc, char const *argv[])\n{\n\tprintf(\"Hallo Welt\\n\");\n\treturn 0;\n}",
        "timestamp": 1607615973307
    },
    ...
]
```

For more information regarding the content of the "result" item of a submission, see [here](https://pad.gwdg.de/VL4fUT5gSvKQWSIT8w5lag?view#Spezifikation-der-REST-API-Schnittstelle). You probably want to to read the paragraph "Ausgabe für `POST /evaluate`". Good luck!

## user

### /username (GET)

Returns the name of the user.

Output:
```
{
    'username': '...'
}
```

### /achievements (GET)

Returns all achievements.

Output:
```
[
    {
        'id': <Integer>,
        'name': <String>,
        'description': <String>,
        'completed': <Bool>
    },
    ...
]
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
