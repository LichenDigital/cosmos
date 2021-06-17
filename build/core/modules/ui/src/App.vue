<template lang="pug">
coLog
div
  img.co-logo(alt='Vue logo' src='./assets/cosmos-combination-mark.svg')
coEditor
</template>

<script lang="ts">

import { defineComponent } from 'vue';
import coEditor from './components/editor/co-editor.vue';
import coLog from './components/co-log.vue';

export default defineComponent({
  name: 'App',
  components: {
    coEditor,
    coLog
  },
  data() {
    return {
    };
  },
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
</style>
