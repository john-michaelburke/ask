# ask
Simple binary for using chatgpt/gemini in terminal with temp history.

## Usage
Go to (OpenAI) https://platform.openai.com/ or (Gemini) https://aistudio.google.com/apikey and sign up for an account. Then in the settings, create an API key.

Specify which engine you would like to use with the environment variable `MODEL_ENGINE` or pass it in with the `-e` flag. Default is `gpt-4o-2024-08-06`. If using Gemini, you could set `MODEL_ENGINE` instead to `gemini-2.0-flash`.

Then either create a new environment variable on your system `ASK_API_KEY` or pass in the key with the `-k` flag. With either the Gemini or OpenAI API key. The API used will be determined by whether the model engine starts with `gpt-` (OpenAI) or `gemini-` (Google).

Then run `ask` with a prompt. The prompt should be a question, and the answer will be the response from chatgpt/gemini.

The query and responses are stored in temp history to allow continuous conversations until your temp folder is cleared typically on reboot.

```
jm@bear:~/dev/ask$ ask 2+2
2 + 2 equals 4.

jm@bear:~/dev/ask$ ask now multiply the first 5 numbers in the fibonacci sequence by that
The first five numbers in the Fibonacci sequence are 1, 1, 2, 3, and 5. Multiplying each of these numbers by 4, we get: 4, 4, 8, 12, and 20.
```

## Future Additions
- [ ] Add the ability to specify which temperature to use, currently the default is `0.5`. This controls how much freedom the model has to hallucinate.
