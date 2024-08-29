Your task is to design a really good aider command for a task. The aider command should include the list of files that should go in to the context window, the prompt specifying what changes should be made to the code base, and URL's for any documentation that would be helpful in completing the task.

Here is the task:

Create a new file src/repositories/mod.rs and define the Repository trait with CRUD methods. Also, define the RepositoryContext struct.

Here's the current file tree of the project

├── .aider.chat.history.md
├── .aider.input.history
├── .env.example
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── README.md
├── config
│   ├── default.toml
│   ├── development.toml
│   ├── production.toml
│   └── test.toml
├── src
│   ├── cli
│   │   └── db_commands.rs
│   ├── cli.rs
│   ├── config.rs
│   ├── main.rs
│   └── services
│       ├── database.rs
│       ├── mod.rs
│       └── web_server.rs
├── tests
│   ├── main_test.rs
│   └── utils.rs
└── watch.sh

Here's all the documentation, which is currently included in the context, and is also available to be included with Aider. Please note that you have all these files in the context, so don't ask for them. They can (and should) be included in the aider command.

documentation
├── design
│   ├── architecture.md
│   ├── code_standards.md
│   ├── data_model.md
│   ├── documentation_plan.md
│   ├── folder_structure.md
│   ├── tech_stack.md
│   ├── test_strategy.md
│   └── user_stories.md
├── process
│   ├── code standard guidelines.md
│   └── process.md

Please list all of the files from the project that might be relevant to complete the task in a space delimited list at the beginning of the prompt in the form aider <file_names> 

Do not generate it until you are ready to make the prompt. 

Please request any documentation you think might be relevant in order to complete the task. 

Refer to the included documentation and design the prompt so that the LLM will produce output conformant to the plan.

Ask any questions for clarification before responding.

