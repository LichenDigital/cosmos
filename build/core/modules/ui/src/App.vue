<template lang="pug">
img(alt='Vue logo' src='./assets/cosmos-combination-mark.svg')
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
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
