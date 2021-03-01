<template lang="pug">
img.co-logo(alt='Vue logo' src='./assets/cosmos-combination-mark.svg')
coEditor
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import coEditor from './components/editor/co-editor.vue';

export default defineComponent({
  name: 'App',
  components: {
    coEditor
  }
});
</script>

<style lang="scss">

*, *::before, *::after {
  box-sizing: inherit;
}

html {
  box-sizing: border-box;
}

body {
  padding: 0;
  margin: 0;
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;

}
.co-group {
  position: relative;
  padding: 1rem;
  padding-top: 4rem;
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
}
</style>
