# Contributing

**Please read [../CONTRIBUTING.md#configuration-requirements](../CONTRIBUTING.md#configuration-requirements) first!**

## Localizing

This project is internationalized, so when you add some text contents to the project, 
please add or use an existing localization key and i18n instead of hardcoding the text.

When you add a new localization key, you have to add an English value in (`locales/en.json5`).
If you understand other languages, you can add values for them but if you don't, please don't add them.
Maintainers of each language will add them.

## Adding languages

It's welcome to add new languages to the project.
If you want to add a new language, you will follow the following steps.

1. Fork the repository and create branch for the new language.
2. Create a new json5 file in `locales` folder with the language code.
   - For example, if you want to add Japanese, create `ja.json5`.
3. Edit code to import the new json5 file in `lib/i18n.ts` and add it to the `languageResources` object.
4. Create a draft pull request. 
5. Update the `CHANGELOG.md` file with the new language addition with pull request number.
6. Mark the pull request as ready for review.
7. The maintainer of the project will ask you that you can be a maintainer of the language.

   If you don't want to be a maintainer of the language,
   until someone else declares to be a maintainer, the language will not be merged.
8. The maintainer of the project will create a new discussion thread for the language.

   The discussion thread will be used to track missing or excess keys for the language.
   The GitHub Actions will update the discussion and reply to a specific thread if there is update, 
   so please track the thread if you are a maintainer of the language.
9. The maintainer of the project will update the actions script to add the language to the CI/CD process.

   For this process, please enable "Allow Edits from Maintainers" in the pull request.
10. The maintainer of the project will merge the pull request.

Notes for maintainer:

Maintainer have to get the node id for github discussions comment.

It's format like `DC_kwDOIza9ks4AjSve` and the random-like part is base64 encoded message pack of `[0, <repo id>, <comment id>]`. (`[0, 590790034, 9252424]`)

```bash
gh api graphql -f query='
  query {
    repository(owner: "vrc-get", name: "vrc-get") {
      discussion(number: <number>) {
        body
        id
        comments(first: 1) {
          nodes {
            body
            id
          }
        }
      }
    }
  }
'
```
