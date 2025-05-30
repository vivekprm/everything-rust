Rust is fast, memory-efficient and  safe. It offers low level control without sacrificing high-level abstractions. Rust code is reliable and easy to maintain.

# Intruduction to cloud-based development environments
## Using Github Codespaces Ecosystem
Lets first create a new repo ```rust-github-demo``` using ```rust-new-project-template```

Create dotfiles project.
Go to settings > Codespaces and select dotfiles project that we created and tick "Automatically install  dotfiles".

Now open ```rust-github-demo``` project in  Codespace. Choose the default one or click on "New with options" select appropriate options and click create. It will take  some  time.
We can also configure Prebuild Configuration for our repository settings/codespaces and it  will install all the  software that we  configured.

https://github.com/vivekprm/rust-github-demo/settings/codespaces

Everything inside .devcontainer will be setup in codespaces.  E.g. in dockerfile we  can  define what needs to be setup. In .devcontainer/devcontainer.json we can setup extensions  for vscode etc.

Let's look at [Github copilot CLI](https://www.npmjs.com/package/@githubnext/github-copilot-cli).

Install using below command:
```sh
npm i @githubnext/github-copilot-cli
```

And type ```?? show me directory structure```. And we can ask natural language problems

# Prompt Engineering with GCP BigQuery SQL
Go to https://platform.openai.com/docs/examples and search SQL translate.

Copy  google bigquery query and  in  chatgpt type:
```
I need  you to explain this  Google  BigQuery  query  for me
<Your  Query>
```

And we  can ask to tweak our queries. So BigQuery with prompt engineering is a very  powerful tool.

# Introduction to AWS CodeWhisperer For Rust
[AWS CodeWhisperer](https://docs.aws.amazon.com/codewhisperer/latest/userguide/what-is-cwspr.html) is AI Pair programming tool that helps you build code programmatically.

https://aws.amazon.com/q/developer/

[AWS Cloud9](https://aws.amazon.com/pm/cloud9/) has easy to setup disposable cloud  environments.

Install AWS toolkit and start getting suggestions.

#  Using Google Gemini to enhance Productivity
https://gemini.google.com/?hl=en-IN
https://gemini.google.com/u/1/app

# CI with Rust & Github Actions
The above template that we used has Github Actions setup under .github directory.