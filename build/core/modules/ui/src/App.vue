<template lang="pug">
.co-log(v-if='logEnabled') log level: {{ logLevel }}
div
  img.co-logo(alt='Vue logo' src='./assets/cosmos-combination-mark.svg')
coEditor
</template>

<script lang="ts">

import { log, logLevel, logEnabled } from './cosmos';
log('verbose', 'this is a verbose message');
log('normal', 'this is a normal message');
log('test', 'this shouldn\'t appear since test is in the log levels array');
log('extraverbose', 'IT\'S OVER 9000!!!');

import { defineComponent } from 'vue';
import coEditor from './components/editor/co-editor.vue';

export default defineComponent({
  name: 'App',
  components: {
    coEditor
  },
  data() {
    return {
      logLevel: logLevel,
      logEnabled: logEnabled
    };
  }
});
</script>

<style lang="scss">

*, *::before, *::after {
  margin: 0;
  padding: 0;
  box-sizing: inherit;
}

html {
  box-sizing: border-box;
  font-size: (10px / 16px) * 100%;
}

body {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  font-size: 1.4rem;
}

h1, h2, h3, h4, h5, h6 {
  padding-top: 1rem;
  margin-bottom: 2rem;
}
p {
  margin-bottom: 1rem;
}

.co-group {
  position: relative;
  padding: 1rem;
  padding-top: 5rem;
}
.co-logo {
  width: 20rem;
  padding: 1rem;
}
.co-panel-label {
  position: absolute;
  user-select: none;
  top: 1rem;
  left: 1.3rem;
  font-size: 2rem;
  font-weight: bold;
  opacity: 40%;
  white-space: pre-wrap;
}
.co-log {
  position: fixed;
  top: 1rem;
  right: 1rem;
  z-index: 9000;
  background: white;
  color: black;
  padding: .2rem 1rem;
  font-weight: bold;
  border: 2px dashed black;
  opacity: .5;
}
</style>
