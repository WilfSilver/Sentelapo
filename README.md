# Natural Language Processor in Rust

This is a small personal project which I made the first commit within 2 days (while also learning about a part of Rust I had not touched yet), meaning it is quite rough around the edges and there is definitely better ways of doing things then currently here.

This is a basic natural language processor (so much so that it might not be called a natural language processor) that takes an input and tries to match it with the training data which to output a "Task" which is set in the config while also collecting data such as first_name or last_name known as fields, which have a field type which define examples of what that would be.

## Config

The structure of the config is built of Twilio's Autopilot idea, however it tries to improve upon it.

### Structure

```
config/<config_name>/field_types/<field_type_name>.td
                    /phrases/<phrase_name>.td
                    /tasks/<task_name>.task
                    /field_types.var
```

### File formats

#### `field_types.var`

This file is to match a field's used in phrases with a field_type, please note these names should not be the same as one of a phrase. Also a field's name should only be made up of alphabetical characters, numbers, `-` or `_`.

- Each line is for a new field
- The line should be organised: `<field_name>:<field_type_name>`

#### Phrases

All the file formats will follow this general format:

- Each line is a new example/sample for the phrase
- Items denoted by `{name}` are either fields (found in the `field_types.var` file) or other phrase

#### Field Types

These fields are mostly like phrases, however they have one special feature because they are used for data collection, they can have synonyms, meaning if a phrase that is known to be a synonym of another phrase, the other phrase will be returned. Please note that as of right now, these cannot have references to other field types.

For example "Hi" and "Hello", if "Hello" is set in the config as a synonym of "Hi", then when "Hello" is found, it will return "Hi" instead of "Hello"

- Each line is a new sample for the phrase
- Phrases on different lines can be split up by `:` to denote a synonym, with the first phrase on the line being what is returned.
  - E.g. `<group_name>:<alternative1>:<alternative2>`, or in our earlier example `Hi:Hello`

#### Tasks

These should be exactly the same as the phrase files, however note they cannot reference another task.

## Current Flaws

As this was quite a rushed job to start with there are a couple of known flaws with this.

- It can calculate phrases twice on the same word when it doesn't need to due to the structure of this.
  - To solve this, we could update it so that instead of phrases that are stored, it is ether 1 word or a field type. However also note that it might also be useful to allow substitutes for words (with a confidence rating) such as "That is" for "This" and have it perform correctly
  - Another solution if you want to stick to the current structure a bit more, is to make sure `GroupPhrases` can only store `WordPhrases` and have `MetaPhrases` expanded out and put into the node structure.
- `confidence` basically means nothing at the moment as the bot cannot infer or assume something.
- An input must be identical with one of the possibilities that is generated, this should be updated to be able to compensate with spelling mistakes or roughly the same words, but with an impact on the confidence.
- The field types cannot return multiple possibilities to try, (atm this is not particularly needed due to the above). If this were to be implemented it would allow us to try multiple versions of a field type if we are unsure and return the one that best fits.
- Field Type does not return `PhraseResult`, which could store information about other fields
- Has to compile everytime it boots up. Not really necessary as it is quite fast atm but still a good thing to note.
- `PhraseResult` and `NodeResult` share a lot in common so could probably be merged into one or something like that.
- Currently it doesn't do anything about punctuation, this should definitely be changed, maybe making is so it is completely ignored, or using punctuation rules to make different sentences that mean the same thing and then try them and could split up full stops and treat them as separate inputs.
- Also, atm it will try to match with the whole sentence (most of the time), but it could be more useful to instead allow for the possibility of multiple tasks in one input, meaning if we don't use all of the input the first time round we can try again the second. However this may cause issues with confidence due to the fact that a longer message is prioritised, but a return could mean that it matches with another task later on. This would probably rely on the possibilities idea earlier.
