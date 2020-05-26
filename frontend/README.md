# Stegos blockchain explorer frontend
This is Frontend of stegos blockchain explorer. Check it out at [explorer.stegos.com](http://explorer.stegos.com).

## Project setup

This project is written on `Vue.js`, and uses yarn package manager. In order to hack with it, first of all you will need to download and install
`Node.js` and `yarn`. 

Then install vue-cli with command:

```
yarn global add @vue/cli
```
<!-- TODO: add instruction for ubuntu, arch, win, macos -->

Frontend is depend on [explorer API](../explorer_backend/README.md).
You can try start it locally, or use our public api.

In order to change API address, enviroment variable `VUE_APP_STEGOS_ADDR` should be set. 
https://cli.vuejs.org/guide/mode-and-env.html#environment-variables

By default we provide `.env` file with `VUE_APP_STEGOS_ADDR` set to our public api `ex01.stegos.com`.

### Install dependencies:

This command will download and install any needed dependencies for frontend, you can use it once, and then forget:

```
yarn install
```

### Compiles and hot-reloads for development

For developming purposes, you can start your local server with frontend started in debug mode:

```
yarn serve
```

This will start server on port `8080`, with access from localhost, or local network.

### Compiles and minifies for production

```
yarn build
```

The artifacts provided by build would be available in `/dist` dirrectory.

### Lints and fixes files
```
yarn lint
```

### Customize configuration
See [Configuration Reference](https://cli.vuejs.org/config/).
