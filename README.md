# octo

REPL for LLM REST APIs

## CLI Invoke

```bash
Usage: octo [OPTIONS]

Options:
  -u, --url   <URL>    URL service endpoint
  -m, --model <MODEL>  Model name
  -s, --stream         Whether using streaming for a better UX FIXME: This should exist or not, not being a boolean
  -h, --help           Print help
  -V, --version        Print version
```

## REPL Commands

- `/exit` or `/quit` to exit the program.
- `/user` add a user prompt to the conversation
- `/system` provide the conversation with a system prompt
- `/context @file1 @./dir/file2` add a list of files to improve context
- `/save ./dir/filename` save conversation to a JSON file

## OpenAI

You need to have a valid `OPENAI_API_KEY=<you token>` environment variable set.
Alternativly, you are developing, create a `.cargo/config.toml` file under the root directory of the project, if you don't have one already, and paste the env variable there like so:

```toml
[env]
OPENAI_API_KEY=<you token>
```
