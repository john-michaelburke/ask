# ask
Simple binary for using chatgpt in terminal with temp history.

## Usage
Go to https://platform.openai.com/ and sign up for an account. Then in the settings, create an API key.

Specify which engine you would like to use with the environment variable `OPENAI_CHATGPT_ENGINE` or pass it in with the `-e` flag. Default is `gpt-4o-2024-08-06`.

Then either create a new environment variable on your system `OPENAI_API_KEY` or pass in the key with the `-k` flag.

Then run `ask` with a prompt. The prompt should be a question, and the answer will be the response from chatgpt.

The query and responses are stored in temp history to allow continuous conversations until your temp folder is cleared typically on reboot.

```
jm@bear:~/dev/ask$ ask 2+2
2 + 2 equals 4.

jm@bear:~/dev/ask$ ask now multiply the first 5 numbers in the fibonacci sequence by that
The first five numbers in the Fibonacci sequence are 1, 1, 2, 3, and 5. Multiplying each of these numbers by 4, we get: 4, 4, 8, 12, and 20.
```

## Future Additions
- [ ] Add the ability to specify which temperature to use, currently the default is `0.5`. This controls how much freedom the model has to hallucinate.
