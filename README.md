# octo

REPL for LLM REST APIs

## CLI Usage

```bash
Usage: octo [OPTIONS] [PROVIDER]

Arguments:
  [PROVIDER]  Provider API to use [default: open-ai] [possible values: open-ai, together-ai, mistral-ai]

Options:
  -a, --api-key <API_KEY>  API key, uses <PROVIDER>_API_KEY env var if not provided
  -u, --url <URL>          URL provider endpoint
  -m, --model <MODEL>      Model name
  -s, --stream             Use streaming API for quicker responses
  -h, --help               Print help
  -V, --version            Print version
```

## REPL

### Interact

Simply start writing to add a user message to the current conversation.

### Commands

- `/exit` or `/quit` to exit the program.
- `/system` provide the conversation with a system prompt
- `/context @file1 @./dir/file2` add a list of files to improve context
- `/save ./dir/filename` save conversation to a JSON file

## Providers

You need to have a valid `<PROVIDER>_API_KEY=<you token>` environment variable set.

Alternatively, while developing, create a `.cargo/config.toml` file under the root directory of the project, if you don't have one already, and paste the env table there like so:

### OpenAI

```toml
[env]
OPENAI_API_KEY=<you token>
```

### TogetherAI

```toml
[env]
TOGETHERAI_API_KEY=<you token>
```
